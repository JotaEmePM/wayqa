use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode, KeyEventKind},
    style::Color,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen},
};

use ratatui::{prelude::*, widgets::*};
use std::{error::Error, io};
use std::{
    io::Stdout,
    time::{Duration, Instant},
};

use super::{
    components::request::render_request_layout,
    wayqa::{InputMode, RequestTab, Wayqa},
};
use crate::core::components::footer;

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

fn teardown_terminal(
    _terminal: &mut Terminal<CrosstermBackend<Stdout>>,
) -> Result<(), Box<dyn Error>> {
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
            if key.kind == KeyEventKind::Press {
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
                                || now.duration_since(
                                    state.last_toggle_project_layout_visible.unwrap(),
                                ) > Duration::from_secs(1)
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
                        KeyCode::Char('1') => {
                            state.change_mode(InputMode::RequestParamsTab);
                            state.current_request_active_tab = RequestTab::Params;
                        }
                        KeyCode::Char('2') => {
                            state.current_request_active_tab = RequestTab::Authorization;
                        }
                        KeyCode::Char('3') => {
                            state.current_request_active_tab = RequestTab::Headers;
                        }
                        KeyCode::Char('4') => {
                            state.current_request_active_tab = RequestTab::Body;
                        }
                        KeyCode::Char('5') => {
                            state.current_request_active_tab = RequestTab::Settings;
                        }
                        KeyCode::Char('6') => {
                            state.change_mode(InputMode::RequestResponseTab);
                            state.current_request_active_tab = RequestTab::Response;
                        }
                        KeyCode::F(5) => {
                            {
                                state.request_running = true;
                                state.on_tick();
                                // ToDo: Comprobar que el request sea valido.
                                let response = state.current_request.execute_request().await;
                                match response {
                                    Ok(_) => {
                                        // state.request_running = false;
                                    }
                                    Err(e) => {
                                        // state.request_running = false;
                                    }
                                }
                            }
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
                    InputMode::RequestParamsTab => match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Request);
                        }
                        _ => {}
                    },
                    InputMode::RequestResponseTab => match key.code {
                        KeyCode::Esc => {
                            state.change_mode(InputMode::Request);
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

    footer::render_footer_layout(f, status_bar, state);
}
