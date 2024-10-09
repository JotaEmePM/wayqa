use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode},
    execute,
    style::Color,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::style::Style;
use ratatui::text::Span;
use ratatui::{prelude::*, widgets::*};
use std::time::{Duration, Instant};
use std::{error::Error, io};

use super::wayqa::{InputMode, Wayqa};

pub fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = Wayqa::new();
    run_app(&mut terminal, &mut app)?;

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

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
                    KeyCode::Char('m') => {
                        state.change_mode(InputMode::RequestMethod);
                    }
                    _ => {}
                },
                InputMode::RequestMethod => match key.code {
                    KeyCode::Down => {
                        //state.selected_method_index += 1;

                    }
                    _ => {}
                },
                InputMode::RequestUrl => todo!(),
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

    let title_Line = status_bar_generator(&state.input_mode);
    f.render_widget(
        Block::new()
            .borders(Borders::NONE)
            .title(title_Line),
        status_bar,
    );
}

fn render_request_layout(f: &mut Frame, block: Rect, state: &mut Wayqa) {
    
    let mut selected_method = 0;

    let vertical_request = Layout::vertical(
        [
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ]
        .as_ref(),
    )
    .margin(2)
    .vertical_margin(1)
    .split(block);

    let method_url_layout =
        Layout::horizontal([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref())
            .split(vertical_request[0]);

    let method_input = Paragraph::new(state.current_request.url.as_str())
    .style(match state.input_mode {
        InputMode::RequestMethod => Style::default().fg(Color::Yellow.into()),
        _ => Style::default(),
    })
    .block(Block::bordered().title("Method"));
f.render_widget(method_input, method_url_layout[0]);

    let url_input = Paragraph::new(state.current_request.url.as_str())
        .style(match state.input_mode {
            InputMode::RequestMethod => Style::default().fg(Color::Yellow.into()),
            _ => Style::default(),
        })
        .block(Block::bordered().title("URL"));
    f.render_widget(url_input, method_url_layout[1]);
}

fn status_bar_generator(input_mode: &InputMode) -> Line<'static>{
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
        InputMode::RequestMethod => {
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
        InputMode::RequestUrl => todo!(),
    };

    result
}
