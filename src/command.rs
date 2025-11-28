
#[derive(Debug, Clone)]
pub enum Command {
  RollTable,
  RollDice(String),
  List,
  Time,
  Add(u32), // in minutes
  Help,
  Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
  let parts: Vec<&str> = input.trim().split_whitespace().collect();
  if parts.is_empty() {
      return Command::Unknown(input.to_string());
  }

  match parts[0].to_lowercase().as_str() {
      "roll" => Command::RollTable,
      "list" => Command::List,
      "time" => Command::Time,
      "dice" => {
        if parts.len() == 2 {
          return Command::RollDice(parts[1].to_string());
        }
        Command::Unknown(input.to_string())
      },
      "add" => {
          if parts.len() == 2 {
              if let Ok(minutes) = parts[1].parse::<u32>() {
                  return Command::Add(minutes);
              }
          }
          Command::Unknown(input.to_string())
      }
      "help" => Command::Help,
      _ => Command::Unknown(input.to_string()),
  }
}
