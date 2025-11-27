mod api;
mod command;
mod table;

use iced::widget::{column, text_editor, text_editor::Content, text_input};
use iced::{Element, color};
use tracing;

use crate::command::Command;

#[derive(Debug, Default)]
pub struct Wayline {
    // UI state
    scrollback: Vec<String>,
    input: String,
    content: Content,

    // Table loaded from TOML
    table: Option<table::Table>,

    // In-game time tracking
    current_time_minutes: u32,
}

#[derive(Debug, Clone)]
pub enum Message {
    Noop,
    WindowOpened,
    WindowClosed,
    EnterPressed,
    ContentChanged(String),
}

impl Wayline {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::window::events().map(|(_, event)| match event {
            iced::window::Event::Opened { .. } => Message::WindowOpened,
            iced::window::Event::Closed => Message::WindowClosed,
            _ => Message::Noop,
        })
    }

    pub fn read_config(&self, path: &str) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(e) => {
                eprintln!("Failed to read config file {}: {}", path, e);
                None
            }
        }
    }

    pub fn load(&mut self, toml_str: &str) {
        match api::parse_table(toml_str) {
            Ok(table) => {
                self.table = Some(table);
            }
            Err(e) => {
                eprintln!("Failed to parse table: {}", e);
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // Editor and input take up the full width and height of the window.
        // Output from the Wayline system will be displayed in the editor (scrollback) area.

        column![
            // Scrollback
            text_editor(&self.content)
                .padding(10)
                .size(16)
                .style(|theme, status| {
                    let mut style = iced::widget::text_editor::default(theme, status);
                    style.value = color!(0xEEEEEE);
                    style
                })
                .height(iced::Length::FillPortion(9)),
            // Input area
            text_input("enter command", &self.input)
                .padding(10)
                .size(16)
                .on_input(Message::ContentChanged)
                .on_submit(Message::EnterPressed),
        ]
        .spacing(10)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EnterPressed => {
                self.on_enter_pressed();
            }
            Message::ContentChanged(new_input) => {
                self.input = new_input;
            }
            Message::WindowOpened => {
                self.update_scrollback("Wayline window opened.");
                if let Some(config) = self.read_config("config.toml") {
                    self.load(&config);
                    self.update_scrollback("Loaded table from config.toml.");
                    tracing::info!("Loaded table: {:?}", self.table);
                } else {
                    self.update_scrollback("No config.toml found.");
                }
            }
            _ => { /* Ignore other messages */ }
        }
    }

    fn on_list_command(&mut self) {
        let lines = if let Some(table) = &self.table {
            let mut lines: Vec<String> = vec![
                format!("Table: {}", table.name),
                format!("Dice: {}", table.dice),
            ];
            for entry in &table.rows {
                lines.push(format!("- {}: {:?}", entry.name, entry.numbers));
            }
            lines
        } else {
            vec!["No table loaded.".to_string()]
        };

        for line in lines {
            self.update_scrollback(&line);
        }
    }

    fn on_time_command(&mut self) {
        let hours = self.current_time_minutes / 60;
        let minutes = self.current_time_minutes % 60;
        self.update_scrollback(
            format!("Current in-game time: {:02}:{:02}", hours, minutes).as_str(),
        );
    }

    fn add_minutes(&mut self, minutes: u32) {
        self.current_time_minutes += minutes;
        self.update_scrollback(
            format!(
                "Added {} minutes. New time: {:02}:{:02}",
                minutes,
                self.current_time_minutes / 60,
                self.current_time_minutes % 60
            )
            .as_str(),
        );
    }

    fn on_enter_pressed(&mut self) {
        // Handle the Enter key press event
        self.update_scrollback(format!("> {}", self.input).as_str());

        let cmd = command::parse_command(&self.input);

        match cmd {
            Command::Roll => self.on_roll_command(),
            Command::List => self.on_list_command(),
            Command::Time => self.on_time_command(),
            Command::Add(minutes) => self.add_minutes(minutes),
            Command::Unknown(cmd) => {
                self.update_scrollback(format!("Unknown command: {}", cmd).as_str());
            }
        }
        self.input.clear()
    }

    fn on_roll_command(&mut self) {
        if let Some(table) = &self.table {
            let dice = &table.dice;
            let (roll, result) = api::roll_on(table, dice);
            if let Some(entry) = result {
                self.update_scrollback(format!("Rolled: {} ({})", entry.name, roll).as_str());
            } else {
                self.update_scrollback(format!("No matching entry found ({}).", roll).as_str());
            }
        } else {
            self.update_scrollback("No table loaded.");
        }
    }

    fn update_scrollback(&mut self, new_line: &str) {
        self.scrollback.push(new_line.to_string());
        let new_content = self.scrollback.join("\n");
        self.content = Content::with_text(&new_content);
    }
}

pub fn main() {
    iced::application("wayline", Wayline::update, Wayline::view)
        .theme(theme)
        .subscription(Wayline::subscription)
        .run()
        .expect("unable to run application")
}

fn theme(_state: &Wayline) -> iced::Theme {
    iced::Theme::Ferra
}
