
#[derive(Debug, Clone)]
pub enum Command {
  Roll,
  List,
  Time,
  Add(u32), // in minutes
  Unknown(String),
}

pub fn parse_command(input: &str) -> Command {
  let parts: Vec<&str> = input.trim().split_whitespace().collect();
  if parts.is_empty() {
      return Command::Unknown(input.to_string());
  }

  match parts[0].to_lowercase().as_str() {
      "roll" => Command::Roll,
      "list" => Command::List,
      "time" => Command::Time,
      "add" => {
          if parts.len() == 2 {
              if let Ok(minutes) = parts[1].parse::<u32>() {
                  return Command::Add(minutes);
              }
          }
          Command::Unknown(input.to_string())
      }
      _ => Command::Unknown(input.to_string()),
  }
}
