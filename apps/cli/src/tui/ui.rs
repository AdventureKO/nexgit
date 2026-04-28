use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use super::app::TuiApp;

pub fn draw(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "nexgit",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  stacked Git workflow scaffold"),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .title(app.title.as_str()),
    );

    frame.render_widget(header, chunks[0]);

    let body = Paragraph::new(vec![
        Line::from("Welcome to the Nexgit TUI."),
        Line::from(""),
        Line::from("This scaffold is ready for repo status, stack graphs, PR state, and operation progress."),
        Line::from("The same Rust core will power headless commands, this TUI, and the desktop app-server."),
    ])
    .wrap(Wrap { trim: true })
    .block(Block::default().borders(Borders::ALL).title("Dashboard"));

    frame.render_widget(body, chunks[1]);

    let footer = Paragraph::new(Line::from(vec![
        Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" quit  •  "),
        Span::styled("Ctrl-C", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" quit  •  "),
        Span::raw(app.status.as_str()),
    ]))
    .block(Block::default().borders(Borders::ALL).title("Keys"));

    frame.render_widget(footer, chunks[2]);
}
