use std::{
    io::{self, stdout, Stdout},
    str::FromStr,
};

use color_eyre;
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
    // Create two chunks with equal horizontal screen space
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

    // Then cut the right vertical piece into two
    let right_side_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(50),
            Constraint::Percentage(10),
        ])
        .split(main_frame[1]); // Return the right chunk

    /* ------ Drawing footer. Should stay last -------*/
    let footer = Paragraph::new(Line::from(Span::styled(
        "(q) / (Esc) to quit",
        Style::default(),
    )))
    .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, right_side_chunks[2]);
}
