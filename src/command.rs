#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    RollTable(Option<String>),
    RollDice(String),
    List(Option<String>),
    Time,
    Add(u32), // in minutes
    Use(String),
    Help,
    Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.is_empty() {
        return Command::Unknown(input.to_string());
    }

    match parts[0].to_lowercase().as_str() {
        "roll" => {
            if parts.len() == 1 {
                Command::RollTable(None)
            } else {
                let table_name = parts[1..].join(" ").to_lowercase();
                Command::RollTable(Some(table_name))
            }
        }
        "list" => if parts.len() == 1 {
            Command::List(None)
        } else {
            Command::List(Some(parts[1..].join(" ").to_lowercase()))
        }
        "time" => Command::Time,
        "use" => {
            let table_name = if parts.len() >= 2 {
                parts[1..].join(" ").to_lowercase()
            } else {
                "".into()
            };
            Command::Use(table_name)
        }
        "dice" => {
            if parts.len() == 2 {
                return Command::RollDice(parts[1].to_string());
            }
            Command::Unknown(input.to_string())
        }
        "add" => {
            if parts.len() == 2
                && let Ok(minutes) = parts[1].parse::<u32>() {
                    return Command::Add(minutes);
                }
            Command::Unknown(input.to_string())
        }
        "help" => Command::Help,
        _ => Command::Unknown(input.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_command() {
        assert_eq!(parse_command("roll"), Command::RollTable(None));
        assert_eq!(parse_command("roll monsters"), Command::RollTable(Some("monsters".to_string())));
        assert_eq!(parse_command("list"), Command::List(None));
        assert_eq!(parse_command("time"), Command::Time);
        assert_eq!(parse_command("use treasures"), Command::Use("treasures".to_string()));
        assert_eq!(parse_command("dice 2d6"), Command::RollDice("2d6".to_string()));
        assert_eq!(parse_command("add 15"), Command::Add(15));
        assert_eq!(parse_command("help"), Command::Help);
        assert_eq!(parse_command("unknown command"), Command::Unknown("unknown command".to_string()));
    }
}
