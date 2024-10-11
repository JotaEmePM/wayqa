use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode, KeyEventKind},
    style::Color,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::{prelude::*, widgets::*};
use std::{io::Stdout, time::{Duration, Instant}};
use std::{error::Error, io};

use super::wayqa::{InputMode, RequestTab, Wayqa};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = setup_terminal()?;

    let mut app = Wayqa::new();    
    run_app(&mut terminal, &mut app)?;

    // restore terminal
    teardown_terminal(&mut terminal)?;

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>, Box<dyn Error>> {
    let mut stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Terminal::new(CrosstermBackend::new(stdout)).map_err(|e| Box::new(e) as Box<dyn Error>)
}

fn teardown_terminal(_terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
  }

#[tokio::main]
async fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    state: &mut Wayqa,
) -> Result<(), std::io::Error> {

    loop {
        terminal.draw(|f| ui(f, state))?;

        if let Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press{
                match state.input_mode {
                
                    InputMode::Normal => match key.code {
                        //&& key.modifiers.contains(KeyModifiers::CONTROL)
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char('p') => {
                            state.change_mode(InputMode::Project);
                        }
                        KeyCode::Char('r') => {
                            state.change_mode(InputMode::Request);
                        }
                        KeyCode::Char('l') => {
                            // TODO: Cambio de tecla para configuracion de layout
                            let now = Instant::now();
                            if state.last_toggle_project_layout_visible.is_none()
                                || now.duration_since(state.last_toggle_project_layout_visible.unwrap())
                                    > Duration::from_secs(1)
                            {
                                state.toggle_project_layout();
                                state.last_toggle_project_layout_visible = Some(now);
                            }
                        }
    
                        _ => {}
                    },
                    InputMode::Project => match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char('n') => {
                            // TODO: New Project
                            if state.saved_info == false {
                                // TODO: Mostrar popup confirmando guardar informacion
                            }
                        }
                        _ => {}
                    },
                    InputMode::Request => match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Normal);
                        }
                        KeyCode::Char('u') => {
                            state.change_mode(InputMode::RequestUrl);
                        }
                        KeyCode::Char('m') => {
                            let now: Instant = Instant::now();
                            if state.last_selected_method.is_none()
                                || now.duration_since(state.last_selected_method.unwrap())
                                    >= Duration::from_secs(1)
                            {
                                state.current_request.change_next_method();
                                state.last_selected_method = Some(now);
                            }
                        }
                        KeyCode::char('1') => {
                            state.current_request_active_tab = RequestTab::Params;
                        }
                        KeyCode::char('2') => {
                            state.current_request_active_tab = RequestTab::Authorization;
                        }
                        KeyCode::char('3') => {
                            state.current_request_active_tab = RequestTab::Headers;
                        }
                        KeyCode::char('4') => {
                            state.current_request_active_tab = RequestTab::Body;
                        }
                        KeyCode::char('5') => {
                            state.current_request_active_tab = RequestTab::Settings;
                        }
                        KeyCode::char('6') => {
                            state.current_request_active_tab = RequestTab::Response;
                        }
                        KeyCode::F(5) =>{
                            // ToDo: Comprobar que el request sea valido.
                            
                        }                        
                        _ => {}
                    },
                    InputMode::RequestUrl => match key.code {
                        KeyCode::Tab => {
                            state.change_mode(InputMode::Request);
                        }
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Request);
                        }
                        KeyCode::Left => {
                            state.move_cursor_url_left();
                        }
                        KeyCode::Right => {
                            state.move_cursor_url_right();
                        }
                        KeyCode::Char(c) => {
                            state.enter_char_request_url(c);
                        }
                        KeyCode::Backspace => {
                            state.delete_char_request_url();
                        }
    
                        _ => {}
                    },
                }
            }
            
        }
    }
}

fn ui(f: &mut Frame, state: &mut Wayqa) {
    let vertical = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
        Constraint::Length(1),
    ]);
    let [title_bar, main_area, status_bar] = vertical.areas(f.area());

    let [layout_project_size, layout_request_size] = if state.project_layout_visible {
        [20, 80]
    } else {
        [0, 100]
    };

    let horizontal = Layout::horizontal([
        Constraint::Percentage(layout_project_size),
        Constraint::Percentage(layout_request_size),
    ]);
    let [left, right] = horizontal.areas(main_area);

    f.render_widget(Block::bordered().title("Project"), left);
    f.render_widget(Block::bordered().title("Request"), right);

    if state.project_layout_visible {
        // TODO: Render Project Layout
    }
    render_request_layout(f, right, state);

    let title_str: String = match state.in_project {
        true => {
            format!("WAYQA - {}", state.project_name)
        }
        false => String::from("WAYQA"),
    };

    let title_bar_block = Block::new()
        .borders(Borders::TOP)
        .title(title_str)
        .title_alignment(Alignment::Center)
        .fg(Color::Cyan);
    f.render_widget(title_bar_block, title_bar);

    let title_line = status_bar_generator(&state.input_mode);
    f.render_widget(
        Block::new().borders(Borders::NONE).title(title_line),
        status_bar,
    );
}

fn render_request_layout(f: &mut Frame, block: Rect, state: &mut Wayqa) {
    let vertical_request = Layout::vertical(
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1)
        ]
        .as_ref(),
    )
    .margin(2)
    .vertical_margin(1)
    .split(block);

    let method_url_layout =
        Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(vertical_request[0]);

    let method_input = Paragraph::new(state.current_request.get_method_str().to_string())
        // .style(match state.input_mode {
        //     InputMode::RequestMethod => Style::default().fg(Color::Yellow.into()),
        //     _ => Style::default(),
        // })
        .block(Block::bordered().title("Method"));
    f.render_widget(method_input, method_url_layout[0]);

    let url_input = Paragraph::new(state.current_request.url.to_string())
        .style(match state.input_mode {
            InputMode::RequestUrl => Style::default().fg(Color::Yellow.into()),
            _ => Style::default(),
        })
        .block(Block::bordered().title("URL"));
    f.render_widget(url_input, method_url_layout[1]);

    match state.input_mode {
        InputMode::RequestUrl => {
            f.set_cursor_position(Position::new(
                method_url_layout[1].x + state.url_cursor_position as u16 +1,
                method_url_layout[1].y + 1
            ));
        }
        _ => {}
    }

    render_request_tab(f, vertical_request[1],vertical_request[2], state);
}

fn render_request_tab(f: &mut Frame, tab_block: Rect,content_block: Rect, state: &mut Wayqa) {
    let tab = Tabs::new(state.get_tab_titles())
        .block(Block::default())
        
        //.select(state.current_request.get_selected_tab())
        //.style(Style::default().fg(Color::Yellow))
        .highlight_style(Style::default().fg(ratatui::style::Color::Yellow).add_modifier(Modifier::BOLD))
        .divider(Span::raw(" "))
        .padding("","")
        .select(match state.current_request_active_tab {
            super::wayqa::RequestTab::Params => 0,
            super::wayqa::RequestTab::Authorization => 1,
            super::wayqa::RequestTab::Headers => 2,
            super::wayqa::RequestTab::Body => 3,
            super::wayqa::RequestTab::Settings => 4,
            super::wayqa::RequestTab::Response => 5,
            _ => {}
        });
    f.render_widget(tab, tab_block);

    match state.current_request_active_tab {
        super::wayqa::RequestTab::Params => state.render_params_tab(f, content_block),
        _ => {}
    }
}

fn status_bar_generator(input_mode: &InputMode) -> Line<'static> {
    /*
    InputMode::Normal => {
            let text = Span::from(vec![
                Span::styled("P ", Style::default().fg(Color::Red.into())),
                Span::styled("roject | ", Style::default().fg(Color::Blue.into())),
                Span::styled("Q ", Style::default().fg(Color::Red.into())),
                Span::styled("uit", Style::default().fg(Color::Blue)),
            ]);
            format!("{:?}", text)
        }
        InputMode::Project => {
            format!("O -> Open project | N -> New project | Esc -> Normal Mode")
        }
        InputMode::Request => {
            format!("M -> Select Method | U -> URL | Esc -> Normal Mode")
        },
    */
    let result = match input_mode {
        InputMode::Normal => {
            let mixed_line = Line::from(vec![
                Span::styled("P", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("roject | New "),
                Span::styled("R", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("equest"),
            ]);
            mixed_line
        }
        InputMode::Project => {
            let mixed_line = Line::from(vec![
                Span::styled("O", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("pen project | "),
                Span::styled("N", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("ew project | "),
                Span::styled("ESC", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("-> Normal mode"),
            ]);
            mixed_line
        }
        InputMode::Request => {
            //format!("M -> Select Method | U -> URL | Esc -> Normal Mode")
            let mixed_line = Line::from(vec![
                Span::from("Select "),
                Span::styled("M", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("ethod | "),
                Span::styled("U", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("RL | "),
                Span::styled("ESC", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("-> Normal mode"),
            ]);
            mixed_line
        }
        // InputMode::RequestMethod => {
        //     let mixed_line = Line::from(vec![
        //         Span::from("Select "),
        //         Span::styled("M", Style::default().fg(Color::Green.into()))
        //             .add_modifier(Modifier::BOLD),
        //         Span::from("ethod | "),
        //         Span::styled("U", Style::default().fg(Color::Green.into()))
        //             .add_modifier(Modifier::BOLD),
        //         Span::from("RL | "),
        //         Span::styled("ESC", Style::default().fg(Color::Green.into()))
        //             .add_modifier(Modifier::BOLD),
        //         Span::from("-> Normal mode"),
        //     ]);
        //     mixed_line
        // },
        InputMode::RequestUrl => {
            let mixed_line = Line::from(vec![
                Span::from("Select "),
                Span::styled("M", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("ethod | "),
                Span::styled("U", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("RL | "),
                Span::styled("ESC", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("-> Normal mode"),
            ]);
            mixed_line
        },
    };

    pub fn render_params_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from params");
        f.render_widget(text, block);
        
    }

    pub fn render_authorization_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from authorization");
        f.render_widget(text, block);
    }

    pub fn render_headers_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from headers");
        f.render_widget(text, block);
    }

    pub fn render_body_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from body");
        f.render_widget(text, block);
    }

    pub fn render_settings_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from settings");
        f.render_widget(text, block);
    }

    pub fn render_response_tab<B: Backed>(f: &mut Frame<B>, area: Rect, state: &mut Wayqa) {
        let horizontal = Layout::horizontal([
            Constraint::Percentage(100),            
        ]);
        let block = horizontal.areas(area);
        let text = Paragraph::new("Hello from response");
        f.render_widget(text, block);
    }

    result
}


