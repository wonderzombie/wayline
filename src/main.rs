mod api;
mod table;

use iced::{Element, color};
use iced::widget::{column, text_editor, text_editor::Content, text_input};

#[derive(Debug, Default)]
pub struct Wayline {
    // UI state
    scrollback: Vec<String>,
    input: String,
    content: Content,

    // Table loaded from TOML
    table: Option<table::Table>,
}

#[derive(Debug, Clone)]
pub enum Message {
    EnterPressed,
    ContentChanged(String),
}

impl Wayline {
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
        }
    }

    fn on_enter_pressed(&mut self) {
        // Handle the Enter key press event
        self.update_scrollback(format!("> {}", self.input).as_str());

        if !self.input.starts_with("roll") {
            self.update_scrollback("Unknown command. Use 'roll' to roll on the table.");
            self.input.clear();
            return;
        }

        if let Some(table) = &self.table {
            if let Some(entry) = api::roll(table, "2d6") {
                self.update_scrollback(format!("Rolled: {}", entry.name).as_str());
            } else {
                self.update_scrollback("No matching entry found.");
            }
        } else {
            self.update_scrollback("No table loaded.");
        }

        self.input.clear();
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
        .run()
        .expect("unable to run application")
}

fn theme(_state: &Wayline) -> iced::Theme {
    iced::Theme::Ferra
}
