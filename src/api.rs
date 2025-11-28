use crate::table;
use rand;

pub fn parse_table(toml_str: &str) -> Result<table::Table, toml::de::Error> {
    toml::from_str(toml_str)
}

pub fn parse_tables(toml_str: &str) -> Result<Vec<table::Table>, toml::de::Error> {
  let list: table::TableList = toml::from_str(toml_str)?;
  Ok(list.table)
}

pub fn roll(dice: &str) -> Option<u32> {
    // Simple parser for dice notation like "2d6"
    let parts: Vec<&str> = dice.split('d').collect();
    if parts.len() != 2 {
        return None;
    }
    let number_of_dice: u32 = parts[0].parse().ok()?;
    let die_type: u32 = parts[1].parse().ok()?;

    let mut rng = rand::rng();
    let mut total_roll = 0;

    for _ in 0..number_of_dice {
        let roll: u32 = rand::Rng::random_range(&mut rng, 1..=die_type);
        total_roll += roll;
    }

    Some(total_roll)
}

pub fn roll_on<'a>(table: &'a table::Table, dice: &str) -> (u32, Option<&'a table::Entry>) {
    let total_roll = roll(dice).unwrap_or(0);

    // Find the corresponding entry in the table
    for entry in &table.rows {
        if entry.numbers.contains(&total_roll) {
            return (total_roll, Some(entry));
        }
    }

    (total_roll, None)
}
