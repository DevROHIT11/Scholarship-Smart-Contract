use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Student not registered")]
    NotRegistered {},

    #[error("Scholarship not yet approved")]
    NotApproved {},

    #[error("Scholarship already claimed")]
    AlreadyClaimed {},
}
