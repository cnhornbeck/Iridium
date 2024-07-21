use ratatui::{
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    terminal::Frame,
    text::{self, Line, Span, Text},
    widgets::{Block, Borders, Padding, Paragraph, Tabs},
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
    let chunks =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(area);
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(10),
        Constraint::Length(3),
    ])
    .split(chunks[0]);

    draw_import_help_message(f, app, chunks[0]);
    draw_import_tab_input(f, app, chunks[1]);
    draw_import_tab_debug_string(f, app, chunks[2]);
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
                Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
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
            app.textarea
                .set_block(Block::default().borders(Borders::ALL).title("Import Input"));
        }
        app::InputMode::Editing => {
            app.textarea.set_block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Yellow))
                    .title("Import Input"),
            );
        }
    }

    app.textarea.input(app.input.clone());

    f.render_widget(app.textarea.widget(), area);

    match app.input_mode {
        app::InputMode::Normal => {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
        }
        app::InputMode::Editing => {
            // // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
            // f.set_cursor(
            //     // Put cursor past the end of the input text
            //     area.x + ((app.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
            //     // Move one line down, from the border to the input line
            //     area.y + 1,
            // );
        }
    }
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
            Span::styled("e", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" copy mod IDs to clipboard."),
        ],
        Style::default().add_modifier(Modifier::RAPID_BLINK),
    );

    let text = Text::from(Line::from(msg)).style(style);
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_export_tab_output(f: &mut Frame, app: &mut App, area: Rect) {
    
}

fn draw_export_tab_body(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .split(area);

    draw_export_help_message(f, chunks[0]);
    draw_export_tab_output(f, app, chunks[1]);
}

// fn draw_first_tab(f: &mut Frame, app: &mut App, area: Rect) {
//     let chunks = Layout::vertical([
//         Constraint::Length(9),
//         Constraint::Min(8),
//         Constraint::Length(7),
//         Constraint::Length(6),
//     ])
//     .split(area);
//     draw_gauges(f, app, chunks[0]);
//     draw_charts(f, app, chunks[1]);
//     draw_text(f, chunks[2]);
//     draw_user_input(f, app, chunks[3]);
// }

// fn draw_gauges(f: &mut Frame, app: &mut App, area: Rect) {
//     let chunks = Layout::vertical([
//         Constraint::Length(2),
//         Constraint::Length(3),
//         Constraint::Length(1),
//     ])
//     .margin(1)
//     .split(area);
//     let block = Block::bordered().title("Graphs");
//     f.render_widget(block, area);

//     let label = format!("{:.2}%", app.progress * 100.0);
//     let gauge = Gauge::default()
//         .block(Block::new().title("Gauge:"))
//         .gauge_style(
//             Style::default()
//                 .fg(Color::Magenta)
//                 .bg(Color::Black)
//                 .add_modifier(Modifier::ITALIC | Modifier::BOLD),
//         )
//         .use_unicode(app.enhanced_graphics)
//         .label(label)
//         .ratio(app.progress);
//     f.render_widget(gauge, chunks[0]);

//     let sparkline = Sparkline::default()
//         .block(Block::new().title("Sparkline:"))
//         .style(Style::default().fg(Color::Green))
//         .data(&app.sparkline.points)
//         .bar_set(if app.enhanced_graphics {
//             symbols::bar::NINE_LEVELS
//         } else {
//             symbols::bar::THREE_LEVELS
//         });
//     f.render_widget(sparkline, chunks[1]);

//     let line_gauge = LineGauge::default()
//         .block(Block::new().title("LineGauge:"))
//         .filled_style(Style::default().fg(Color::Magenta))
//         .line_set(if app.enhanced_graphics {
//             symbols::line::THICK
//         } else {
//             symbols::line::NORMAL
//         })
//         .ratio(app.progress);
//     f.render_widget(line_gauge, chunks[2]);
// }

// #[allow(clippy::too_many_lines)]
// fn draw_charts(f: &mut Frame, app: &mut App, area: Rect) {
//     let constraints = if app.show_chart {
//         vec![Constraint::Percentage(50), Constraint::Percentage(50)]
//     } else {
//         vec![Constraint::Percentage(100)]
//     };
//     let chunks = Layout::horizontal(constraints).split(area);
//     {
//         let chunks = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
//             .split(chunks[0]);
//         {
//             let chunks =
//                 Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
//                     .split(chunks[0]);

//             // Draw tasks
//             let tasks: Vec<ListItem> = app
//                 .tasks
//                 .items
//                 .iter()
//                 .map(|i| ListItem::new(vec![text::Line::from(Span::raw(*i))]))
//                 .collect();
//             let tasks = List::new(tasks)
//                 .block(Block::bordered().title("List"))
//                 .highlight_style(Style::default().add_modifier(Modifier::BOLD))
//                 .highlight_symbol("> ");
//             f.render_stateful_widget(tasks, chunks[0], &mut app.tasks.state);

//             // Draw logs
//             let info_style = Style::default().fg(Color::Blue);
//             let warning_style = Style::default().fg(Color::Yellow);
//             let error_style = Style::default().fg(Color::Magenta);
//             let critical_style = Style::default().fg(Color::Red);
//             let logs: Vec<ListItem> = app
//                 .logs
//                 .items
//                 .iter()
//                 .map(|&(evt, level)| {
//                     let s = match level {
//                         "ERROR" => error_style,
//                         "CRITICAL" => critical_style,
//                         "WARNING" => warning_style,
//                         _ => info_style,
//                     };
//                     let content = vec![text::Line::from(vec![
//                         Span::styled(format!("{level:<9}"), s),
//                         Span::raw(evt),
//                     ])];
//                     ListItem::new(content)
//                 })
//                 .collect();
//             let logs = List::new(logs).block(Block::bordered().title("List"));
//             f.render_stateful_widget(logs, chunks[1], &mut app.logs.state);
//         }

//         let barchart = BarChart::default()
//             .block(Block::bordered().title("Bar chart"))
//             .data(&app.barchart)
//             .bar_width(3)
//             .bar_gap(2)
//             .bar_set(if app.enhanced_graphics {
//                 symbols::bar::NINE_LEVELS
//             } else {
//                 symbols::bar::THREE_LEVELS
//             })
//             .value_style(
//                 Style::default()
//                     .fg(Color::Black)
//                     .bg(Color::Green)
//                     .add_modifier(Modifier::ITALIC),
//             )
//             .label_style(Style::default().fg(Color::Yellow))
//             .bar_style(Style::default().fg(Color::Green));
//         f.render_widget(barchart, chunks[1]);
//     }
//     if app.show_chart {
//         let x_labels = vec![
//             Span::styled(
//                 format!("{}", app.signals.window[0]),
//                 Style::default().add_modifier(Modifier::BOLD),
//             ),
//             Span::raw(format!(
//                 "{}",
//                 (app.signals.window[0] + app.signals.window[1]) / 2.0
//             )),
//             Span::styled(
//                 format!("{}", app.signals.window[1]),
//                 Style::default().add_modifier(Modifier::BOLD),
//             ),
//         ];
//         let datasets = vec![
//             Dataset::default()
//                 .name("data2")
//                 .marker(symbols::Marker::Dot)
//                 .style(Style::default().fg(Color::Cyan))
//                 .data(&app.signals.sin1.points),
//             Dataset::default()
//                 .name("data3")
//                 .marker(if app.enhanced_graphics {
//                     symbols::Marker::Braille
//                 } else {
//                     symbols::Marker::Dot
//                 })
//                 .style(Style::default().fg(Color::Yellow))
//                 .data(&app.signals.sin2.points),
//         ];
//         let chart = Chart::new(datasets)
//             .block(
//                 Block::bordered().title(Span::styled(
//                     "Chart",
//                     Style::default()
//                         .fg(Color::Cyan)
//                         .add_modifier(Modifier::BOLD),
//                 )),
//             )
//             .x_axis(
//                 Axis::default()
//                     .title("X Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds(app.signals.window)
//                     .labels(x_labels),
//             )
//             .y_axis(
//                 Axis::default()
//                     .title("Y Axis")
//                     .style(Style::default().fg(Color::Gray))
//                     .bounds([-20.0, 20.0])
//                     .labels(vec![
//                         Span::styled("-20", Style::default().add_modifier(Modifier::BOLD)),
//                         Span::raw("0"),
//                         Span::styled("20", Style::default().add_modifier(Modifier::BOLD)),
//                     ]),
//             );
//         f.render_widget(chart, chunks[1]);
//     }
// }

// fn draw_text(f: &mut Frame, area: Rect) {
//     let text = vec![
//         text::Line::from("This is a paragraph with several lines. You can change style your text the way you want"),
//         text::Line::from(""),
//         text::Line::from(vec![
//             Span::from("For example: "),
//             Span::styled("under", Style::default().fg(Color::Red)),
//             Span::raw(" "),
//             Span::styled("the", Style::default().fg(Color::Green)),
//             Span::raw(" "),
//             Span::styled("rainbow", Style::default().fg(Color::Blue)),
//             Span::raw("."),
//         ]),
//         text::Line::from(vec![
//             Span::raw("Oh and if you didn't "),
//             Span::styled("notice", Style::default().add_modifier(Modifier::ITALIC)),
//             Span::raw(" you can "),
//             Span::styled("automatically", Style::default().add_modifier(Modifier::BOLD)),
//             Span::raw(" "),
//             Span::styled("wrap", Style::default().add_modifier(Modifier::REVERSED)),
//             Span::raw(" your "),
//             Span::styled("text", Style::default().add_modifier(Modifier::UNDERLINED)),
//             Span::raw(".")
//         ]),
//         text::Line::from(
//             "One more thing is that it should display unicode characters: 10€"
//         ),
//     ];
//     let block = Block::bordered().title(Span::styled(
//         "Footer",
//         Style::default()
//             .fg(Color::Magenta)
//             .add_modifier(Modifier::BOLD),
//     ));
//     let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: true });
//     f.render_widget(paragraph, area);
// }

// fn draw_user_input(f: &mut Frame, app: &mut App, area: Rect) {}

// fn draw_second_tab(f: &mut Frame, app: &mut App, area: Rect) {
//     let chunks =
//         Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);
//     let up_style = Style::default().fg(Color::Green);
//     let failure_style = Style::default()
//         .fg(Color::Red)
//         .add_modifier(Modifier::RAPID_BLINK | Modifier::CROSSED_OUT | Modifier::ITALIC);
//     let rows = app.servers.iter().map(|s| {
//         let style = if s.status == "Up" {
//             up_style
//         } else {
//             failure_style
//         };
//         Row::new(vec![s.name, s.location, s.status]).style(style)
//     });
//     let table = Table::new(
//         rows,
//         [
//             Constraint::Length(15),
//             Constraint::Length(15),
//             Constraint::Length(10),
//         ],
//     )
//     .header(
//         Row::new(vec!["Server", "Location", "Status"])
//             .style(Style::default().fg(Color::Yellow))
//             .bottom_margin(1),
//     )
//     .block(Block::bordered().title("Servers"));
//     f.render_widget(table, chunks[0]);

//     let map = Canvas::default()
//         .block(Block::bordered().title("World"))
//         .paint(|ctx| {
//             ctx.draw(&Map {
//                 color: Color::White,
//                 resolution: MapResolution::High,
//             });
//             ctx.layer();
//             ctx.draw(&Rectangle {
//                 x: 0.0,
//                 y: 30.0,
//                 width: 10.0,
//                 height: 10.0,
//                 color: Color::Yellow,
//             });
//             ctx.draw(&Circle {
//                 x: app.servers[2].coords.1,
//                 y: app.servers[2].coords.0,
//                 radius: 10.0,
//                 color: Color::Green,
//             });
//             for (i, s1) in app.servers.iter().enumerate() {
//                 for s2 in &app.servers[i + 1..] {
//                     ctx.draw(&canvas::Line {
//                         x1: s1.coords.1,
//                         y1: s1.coords.0,
//                         y2: s2.coords.0,
//                         x2: s2.coords.1,
//                         color: Color::Yellow,
//                     });
//                 }
//             }
//             for server in &app.servers {
//                 let color = if server.status == "Up" {
//                     Color::Green
//                 } else {
//                     Color::Red
//                 };
//                 ctx.print(
//                     server.coords.1,
//                     server.coords.0,
//                     Span::styled("X", Style::default().fg(color)),
//                 );
//             }
//         })
//         .marker(if app.enhanced_graphics {
//             symbols::Marker::Braille
//         } else {
//             symbols::Marker::Dot
//         })
//         .x_bounds([-180.0, 180.0])
//         .y_bounds([-90.0, 90.0]);
//     f.render_widget(map, chunks[1]);
// }

// fn draw_third_tab(f: &mut Frame, _app: &mut App, area: Rect) {
//     let chunks = Layout::horizontal([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)]).split(area);
//     let colors = [
//         Color::Reset,
//         Color::Black,
//         Color::Red,
//         Color::Green,
//         Color::Yellow,
//         Color::Blue,
//         Color::Magenta,
//         Color::Cyan,
//         Color::Gray,
//         Color::DarkGray,
//         Color::LightRed,
//         Color::LightGreen,
//         Color::LightYellow,
//         Color::LightBlue,
//         Color::LightMagenta,
//         Color::LightCyan,
//         Color::White,
//     ];
//     let items: Vec<Row> = colors
//         .iter()
//         .map(|c| {
//             let cells = vec![
//                 Cell::from(Span::raw(format!("{c:?}: "))),
//                 Cell::from(Span::styled("Foreground", Style::default().fg(*c))),
//                 Cell::from(Span::styled("Background", Style::default().bg(*c))),
//             ];
//             Row::new(cells)
//         })
//         .collect();
//     let table = Table::new(
//         items,
//         [
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//             Constraint::Ratio(1, 3),
//         ],
//     )
//     .block(Block::bordered().title("Colors"));
//     f.render_widget(table, chunks[0]);
// }
