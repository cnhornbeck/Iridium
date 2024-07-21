use std::{error::Error, io, time::Duration};

use crossterm::event::KeyModifiers;
use ratatui::{
    backend::{Backend, CrosstermBackend},
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    terminal::Terminal,
};

use crate::app::{self, App};
use crate::ui;

pub fn run() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let app = App::new("Iridium");
    let res = run_app(&mut terminal, app);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, &mut app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match app.input_mode {
                    app::InputMode::Normal => {
                        if key.kind == KeyEventKind::Press {
                            handle_normal_mode(&mut app, key.code);
                        }
                    }
                    app::InputMode::Editing => handle_editing_mode(&mut app, key),
                }
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn handle_normal_mode(app: &mut App, key_code: KeyCode) {
    match key_code {
        // KeyCode::Enter => app.on_enter(),
        KeyCode::Left => app.on_left(),
        KeyCode::Right => app.on_right(),
        KeyCode::Tab => app.on_tab(),
        KeyCode::Esc => app.on_esc(),
        KeyCode::Char(c) => app.on_key(c),
        _ => {}
    }
}

fn handle_editing_mode(app: &mut App, key: event::KeyEvent) {
    match (key.code, key.modifiers, key.kind) {
        (KeyCode::Enter, KeyModifiers::CONTROL, KeyEventKind::Press) => app.on_enter(),
        (KeyCode::Esc, KeyModifiers::NONE, KeyEventKind::Press) => app.on_esc(),
        (KeyCode::Tab, KeyModifiers::NONE, KeyEventKind::Press) => app.on_tab(),
        _ => app.handle_text_edit_input(key.into()),
    }
}
