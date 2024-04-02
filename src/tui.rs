use std::{
    io::{self, stdout, Stdout},
    str::FromStr,
};

use crossterm::{execute, terminal::*};
use ratatui::{prelude::*, widgets::*};

use super::app::App;

/// A type alias for the terminal type used in this application
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Initialize the terminal
pub fn init() -> io::Result<Tui> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

/// Restore the terminal to its original state
pub fn restore() -> io::Result<()> {
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

/// Draw the interface
pub fn ui(frame: &mut Frame, app: &mut App) {
    // Split screen to left and right sides
    let main_frame = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Fill(1)])
        .split(frame.size());

    // Create a List from all list items and highlight the currently selected one
    let items = List::new(app.name_list.clone())
        .block(Block::default().borders(Borders::ALL).title("Tasks"))
        .highlight_style(
            Style::default()
                .bg(Color::from_str("#13a463").unwrap())
                .add_modifier(Modifier::BOLD)
                .black(),
        );
    // We can now render the item list
    frame.render_stateful_widget(items, main_frame[0], &mut app.items.state);

    // Then cut the right vertical piece into three
    let right_side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(50),
            Constraint::Min(3),
        ])
        .split(main_frame[1]);

    /* ------ Drawing Right Top -------*/

    let right_top = Block::default().title("Description").borders(Borders::ALL);
    let right_top_text = Paragraph::new(Line::from(app.get_desc()))
        .block(right_top)
        .wrap(Wrap { trim: false });
    frame.render_widget(right_top_text, right_side_chunks[0]);

    /* ------ Drawing Right Bottom -------*/

    let right_btm = Block::default().title("Summary").borders(Borders::ALL);
    let right_btm_text = Paragraph::new(Line::from(app.get_summary()))
        .block(right_btm)
        .wrap(Wrap { trim: false });
    frame.render_widget(right_btm_text, right_side_chunks[1]);

    /* ------ Drawing footer -------*/
    let footer = Paragraph::new(Line::from(vec![
        "(q) / (Esc)".fg(Color::from_str("#d45e7b").unwrap()),
        " to quit | ".into(),
        "(Enter)".fg(Color::from_str("#ee9966").unwrap()),
        " to select task".into(),
    ]))
    .centered()
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, right_side_chunks[2]);
}
