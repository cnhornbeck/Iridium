use std::{
    sync::{Arc, Mutex},
    thread,
};

use rand::distributions::Distribution;
use tui_textarea::{Input, TextArea};

use profile_manager::profile::Profile;

use crate::profile_manager;

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub enum InputMode {
    Normal,
    Editing,
}

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub show_chart: bool,
    pub input: Input,
    pub textarea: TextArea<'a>,
    /// Current input mode
    pub input_mode: InputMode,
    pub debug_string: Arc<Mutex<String>>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> Self {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Import", "Export", "From Repo"]),
            show_chart: true,
            input: Input::default(),
            textarea: TextArea::default(),
            input_mode: InputMode::Normal,
            debug_string: Arc::new(Mutex::new(String::new())),
        }
    }

    // pub fn on_up(&mut self) {
    //     self.tasks.previous();
    // }

    // pub fn on_down(&mut self) {
    //     self.tasks.next();
    // }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            't' => {
                self.show_chart = !self.show_chart;
            }
            'e' => {
                if self.tabs.index == 0 {
                    self.input_mode = InputMode::Editing;
                }
            }
            _ => {}
        }
    }

    pub fn on_tab(&mut self) {
        self.tabs.next();
    }

    pub fn on_esc(&mut self) {
        if self.tabs.index == 0 {
            self.input_mode = InputMode::Normal;
        }
    }

    pub fn handle_text_edit_input(&mut self, input: Input) {
        self.input = input;
    }

    pub fn on_enter(&mut self) {
        if self.tabs.index == 0 {
            let import_string: Vec<String> = self
                .textarea
                .lines()
                .iter()
                .map(|line| line.to_string())
                .collect();
            let debug_string_clone = Arc::clone(&self.debug_string); // Clone the Arc for thread safety
            thread::spawn(move || {
                Profile::import_mods(import_string, debug_string_clone);
            });
        }
        self.textarea.select_all(); // Mutable borrow of textarea
    }
}
