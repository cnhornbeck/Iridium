use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{self, Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Tabs, Wrap},
};

use crate::app::{self, App};

pub fn draw(f: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(f.size());
    let tabs = app
        .tabs
        .titles
        .iter()
        .map(|t| text::Line::from(Span::styled(*t, Style::default().fg(Color::Green))))
        .collect::<Tabs>()
        .block(Block::bordered().title(app.title))
        .highlight_style(Style::default().fg(Color::Yellow))
        .select(app.tabs.index);
    f.render_widget(tabs, chunks[0]);
    match app.tabs.index {
        0 => draw_import_tab(f, app, chunks[1]),
        1 => draw_export_tab(f, app, chunks[1]),
        _ => {}
    }
}

fn draw_import_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(15)]).split(area);
    draw_import_tab_header(f, chunks[0]);
    draw_import_tab_body(f, app, chunks[1]);
}

fn draw_import_tab_header(f: &mut Frame, area: Rect) {
    let import_tab_text = Paragraph::new("Import Mods")
        .block(
            Block::default()
                .style(Style::default().bg(Color::Black))
                .padding(Padding::new(
                    0,               // left
                    0,               // right
                    area.height / 2, // top
                    0,               // bottom
                )),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(import_tab_text, area);
}

fn draw_import_tab_body(f: &mut Frame, app: &mut App, area: Rect) {
    {
        let chunks = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(10),
            Constraint::Length(3),
        ])
        .split(area);

        draw_import_help_message(f, app, chunks[0]);
        draw_import_tab_input(f, app, chunks[1]);
        draw_import_tab_debug_string(f, app, chunks[2]);
    }
}

fn draw_import_help_message(f: &mut Frame, app: &mut App, area: Rect) {
    let (msg, style) = match app.input_mode {
        app::InputMode::Normal => (
            vec![
                Span::raw("Press "),
                Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to exit app, "),
                Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to enter mod IDs."),
            ],
            Style::default().add_modifier(Modifier::RAPID_BLINK),
        ),
        app::InputMode::Editing => (
            vec![
                Span::raw("Press "),
                Span::styled("Esc", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to stop editing, "),
                Span::styled("Ctrl+Enter", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to submit mods for download"),
            ],
            Style::default(),
        ),
    };

    let text = Text::from(Line::from(msg)).style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_import_tab_input(f: &mut Frame, app: &mut App, area: Rect) {
    match app.input_mode {
        app::InputMode::Normal => {
            app.textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Import String"),
            );
        }
        app::InputMode::Editing => {
            app.textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
                    .title("Import String"),
            );
        }
    }

    app.textarea.input(app.input.clone());

    f.render_widget(app.textarea.widget(), area);
}

fn draw_import_tab_debug_string(f: &mut Frame, app: &App, area: Rect) {
    let debug_string;
    {
        // Lock the debug_string and clone it within the scope
        let debug_string_lock = app.debug_string.lock().unwrap();
        debug_string = debug_string_lock.clone();
    }

    let paragraph = Paragraph::new(debug_string);
    f.render_widget(paragraph, area);
}

fn draw_export_tab(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]).split(area);
    draw_export_tab_header(f, chunks[0]);
    draw_export_tab_body(f, app, chunks[1]);
}

fn draw_export_tab_header(f: &mut Frame, area: Rect) {
    let export_tab_text = Paragraph::new("Export Mods")
        .block(
            Block::default()
                .style(Style::default().bg(Color::Black))
                .padding(Padding::new(
                    0,               // left
                    0,               // right
                    area.height / 2, // top
                    0,               // bottom
                )),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    f.render_widget(export_tab_text, area);
}

fn draw_export_help_message(f: &mut Frame, area: Rect) {
    let (msg, style) = (
        vec![
            Span::raw("Press "),
            Span::styled("q", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" to exit app, "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" copy mod IDs to clipboard."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );

    let text = Text::from(Line::from(msg)).style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_export_tab_output(f: &mut Frame, app: &mut App, area: Rect) {
    let export_status;
    {
        let export_status_lock = app.export_status.lock().unwrap();
        export_status = export_status_lock.clone();
    }

    let export_status_widget = Paragraph::new(export_status)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Export Mods"),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(export_status_widget, area);
}

fn draw_export_tab_body(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([Constraint::Length(1), Constraint::Length(3)]).split(area);

    draw_export_help_message(f, chunks[0]);
    draw_export_tab_output(f, app, chunks[1]);
}
