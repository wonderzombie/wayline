mod api;
mod command;
mod table;

use std::collections::HashMap;

use iced::widget::{column, text_editor, text_editor::Content, text_input};
use iced::{Element, color};
use tracing::error;

use crate::command::Command;

#[derive(Debug, Default)]
pub struct Wayline {
    // UI state
    scrollback: Vec<String>,
    input: String,
    content: Content,

    // Table loaded from TOML
    current_table: Option<String>,
    tables: HashMap<String, table::Table>,

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

    pub fn table(&self) -> Option<&table::Table> {
        if let Some(current_table) = &self.current_table {
            self.tables.get(current_table)
        } else {
            None
        }
    }

    pub fn read_config(&self, path: &str) -> Option<String> {
        match std::fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(e) => {
                error!("Failed to read config file {}: {}", path, e);
                None
            }
        }
    }

    pub fn load_all(&mut self, toml_str: &str) {
        match api::parse_tables(toml_str) {
            Ok(tables) => {
                for table in tables {
                    self.tables.insert(table.name.to_lowercase(), table);
                }
            }
            Err(e) => {
                error!("Failed to parse tables: {}", e);
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
                .size(14)
                .style(|theme, status| {
                    let mut style = iced::widget::text_editor::default(theme, status);
                    style.value = color!(0xEEEEEE);
                    style
                })
                .height(iced::Length::FillPortion(9)),
            // Input area
            text_input("enter command", &self.input)
                .padding(10)
                .size(14)
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
                if let Some(config) = self.read_config("tables.toml") {
                    self.load_all(&config);
                    self.update_scrollback(format!(
                        "Loaded tables from tables.toml: {:?}.",
                        self.tables.keys()
                    ));
                    if let Some(first_table_name) = self.tables.keys().next() {
                        self.current_table = Some(first_table_name.clone());
                        self.update_scrollback(format!(
                            "Current table set to '{}'.",
                            first_table_name
                        ));
                    }
                } else {
                    self.update_scrollback("No tables.toml found.");
                }
            }
            _ => { /* Ignore other messages */ }
        }
    }

    /// If no table is loaded, do nothing.
    /// If multiple tables are loaded but none is selected, list table names.
    /// If one table is selected, list its entries.
    fn on_list_command(&mut self) {
        if self.tables.is_empty() {
            self.update_scrollback("No tables loaded.");
            return;
        }

        if let Some(table) = self.table() {
            let mut lines: Vec<String> = vec![
                format!("Table: {}", table.name),
                format!("Dice: {}", table.dice),
            ];
            for entry in &table.rows {
                lines.push(format!("- {}: {:?}", entry.name, entry.numbers));
            }
            for line in lines {
                self.update_scrollback(line);
            }
        } else {
            self.update_scrollback("Loaded tables:");
            let lines: Vec<String> = self
                .tables
                .keys()
                .map(|name| format!("- {}", name))
                .collect();
            for line in lines {
                self.update_scrollback(line);
            }
        }
    }

    fn on_time_command(&mut self) {
        let hours = self.current_time_minutes / 60;
        let minutes = self.current_time_minutes % 60;
        self.update_scrollback(format!("Current in-game time: {:02}:{:02}", hours, minutes));
    }

    fn add_minutes(&mut self, minutes: u32) {
        self.current_time_minutes += minutes;
        self.update_scrollback(format!(
            "Added {} minutes. New time: {:02}:{:02}",
            minutes,
            self.current_time_minutes / 60,
            self.current_time_minutes % 60
        ));
    }

    fn on_enter_pressed(&mut self) {
        // Handle the Enter key press event
        self.update_scrollback(format!("> {}", self.input));

        let cmd = command::parse_command(&self.input);

        match cmd {
            Command::RollTable => self.on_roll_command(),
            Command::RollDice(dice_str) => {
                if let Some(roll) = api::roll(&dice_str) {
                    self.update_scrollback(format!("Rolled {}: {}", dice_str, roll));
                } else {
                    self.update_scrollback(format!("Invalid dice notation: {}", dice_str));
                }
            }
            Command::List => self.on_list_command(),
            Command::Time => self.on_time_command(),
            Command::Add(minutes) => self.add_minutes(minutes),
            Command::Help => {
                self.update_scrollback("Available commands:");
                self.update_scrollback("- roll : Roll on the loaded table");
                self.update_scrollback("- dice <notation> : Roll custom dice (e.g., '2d6')");
                self.update_scrollback("- list : List the loaded table entries");
                self.update_scrollback("- time : Show current in-game time");
                self.update_scrollback("- add <minutes> : Add minutes to in-game time");
                self.update_scrollback("- help : Show this help message");
            }
            Command::Unknown(cmd) => {
                self.update_scrollback(format!("Unknown command: {}", cmd));
            }
            Command::Use(table_name) => {
                if self.tables.contains_key(&table_name) {
                    self.current_table = Some(table_name.clone());
                    self.update_scrollback(format!("Switched to table '{}'.", table_name));
                } else {
                    self.update_scrollback(format!("Table '{}' not found.", table_name));
                }
            }
        }

        self.input.clear();
    }

    fn on_roll_command(&mut self) {
        if let Some(table) = self.table() {
            let dice = &table.dice;
            let (roll, result) = api::roll_on(table, dice);
            if let Some(entry) = result {
                self.update_scrollback(format!("Rolled: {} ({})", entry.name, roll));
            } else {
                self.update_scrollback(format!("No matching entry found ({}).", roll));
            }
        } else {
            self.update_scrollback("No table loaded.");
        }
    }

    fn update_scrollback<S: Into<String>>(&mut self, new_line: S) {
        self.scrollback.push(new_line.into());
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
