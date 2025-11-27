use serde::{Deserialize, Serialize};

// We want to have tables that can be serialized/deserialized to/from TOML.
// These tables will hold entries with names and associated items for the Wayline system,
// specifically random encounter tables.
//
// Some tables use different dice and some entries in the list may have weights for selection probability.
//
// Example TOML representation:
// ```toml
// [table]
// name = "Wilderness Encounters"
// roll = "2d6"
// [[rows]]
// name = "Goblin Ambush"
// result = [2, 3]
// [[rows]]
// name = "Bandit Raid"
// result = [4, 5]
// [[rows]]
// name = "Dragon Sighting"
// result = [12, 12]
// ```
//

#[derive(Debug, Serialize, Deserialize)]
pub struct Table {
  pub name: String,
  pub rows: Vec<Entry>,
  pub roll: String, // e.g., "2d6",
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entry {
  pub name: String,
  pub numbers: Vec<u32>, // Die results that correspond to this entry
}
