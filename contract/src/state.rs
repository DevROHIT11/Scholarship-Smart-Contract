use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; // Add this import

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // Add JsonSchema here
pub struct Config {
    pub admin: String,
    pub scholarship_amount: u128,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // Add JsonSchema here
pub struct Student {
    pub approved: bool,
    pub claimed: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const STUDENTS: Map<&str, Student> = Map::new("students");
