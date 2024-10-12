use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::core::wayqa::{InputMode, Wayqa};

pub fn render_footer_layout(f: &mut Frame, block: Rect, state: &mut Wayqa) {
    let footer_layout =
        Layout::horizontal([Constraint::Percentage(90), Constraint::Percentage(10)]);
    let [keybindings, status] = footer_layout.areas(block);

    let title_line = status_bar_generator(&state.input_mode);
    f.render_widget(
        Block::new().borders(Borders::NONE).title(title_line),
        keybindings,
    );
    let throbber = throbber_widgets_tui::Throbber::default()
        .label("Running...")
        .style(ratatui::style::Style::default().fg(ratatui::style::Color::Cyan))
        .throbber_style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Red)
                .add_modifier(ratatui::style::Modifier::BOLD),
        )
        .throbber_set(throbber_widgets_tui::CLOCK)
        .use_type(throbber_widgets_tui::WhichUse::Spin);
    match state.request_running {
        true => {
            state.on_tick();
            f.render_stateful_widget(throbber, status, &mut state.throbber_state);
        }
        false => {}
    }
}

pub fn status_bar_generator(input_mode: &InputMode) -> Line<'static> {
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
        }
        InputMode::RequestParamsTab => {
            let mixed_line = Line::from(vec![
                Span::styled("ESC", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("-> Normal mode"),
            ]);
            mixed_line
        }
        InputMode::RequestResponseTab => {
            let mixed_line = Line::from(vec![
                Span::styled("ESC", Style::default().fg(Color::Green.into()))
                    .add_modifier(Modifier::BOLD),
                Span::from("-> Normal mode"),
            ]);
            mixed_line
        }
    };
    result
}
