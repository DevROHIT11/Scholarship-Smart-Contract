use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; // Add this import

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // Add JsonSchema here
pub struct InstantiateMsg {
    pub scholarship_amount: u128,
    pub denom: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // Add JsonSchema here
pub enum ExecuteMsg {
    RegisterStudent { address: String },
    ApproveStudent { address: String },
    ClaimScholarship {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] // Add JsonSchema here
pub enum QueryMsg {
    GetStudent { address: String },
}
