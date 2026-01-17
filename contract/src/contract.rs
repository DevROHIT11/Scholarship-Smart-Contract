use cosmwasm_std::{
    entry_point, to_json_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response,
    StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, Student, CONFIG, STUDENTS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let config = Config {
        admin: info.sender.to_string(),
        scholarship_amount: msg.scholarship_amount,
        denom: msg.denom, // Ensure this matches your Config struct in state.rs
    };

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", info.sender))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::RegisterStudent { address } => register_student(deps, info, address),
        ExecuteMsg::ApproveStudent { address } => approve_student(deps, info, address),
        ExecuteMsg::ClaimScholarship {} => claim_scholarship(deps, info),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStudent { address } => {
            let student = STUDENTS.load(deps.storage, &address)?;
            to_json_binary(&student)
        }
    }
}

/* ================= INTERNAL FUNCTIONS ================= */

fn register_student(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let validated_addr = deps.api.addr_validate(&address)?;

    STUDENTS.save(
        deps.storage,
        validated_addr.as_str(),
        &Student {
            approved: false,
            claimed: false,
        },
    )?;

    Ok(Response::new().add_attribute("action", "register_student"))
}

fn approve_student(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    let validated_addr = deps.api.addr_validate(&address)?;

    STUDENTS.update(
        deps.storage,
        validated_addr.as_str(),
        |s: Option<Student>| -> Result<Student, ContractError> {
            let mut student = s.ok_or(ContractError::NotRegistered {})?;
            student.approved = true;
            Ok(student)
        },
    )?;

    Ok(Response::new().add_attribute("action", "approve_student"))
}

fn claim_scholarship(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    STUDENTS.update(
        deps.storage,
        info.sender.as_str(),
        |s: Option<Student>| -> Result<Student, ContractError> {
            let mut st = s.ok_or(ContractError::NotRegistered {})?;

            if !st.approved {
                return Err(ContractError::NotApproved {});
            }
            if st.claimed {
                return Err(ContractError::AlreadyClaimed {});
            }

            st.claimed = true;
            Ok(st)
        },
    )?;

    let send_msg = BankMsg::Send {
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: config.denom,
            amount: config.scholarship_amount.into(),
        }],
    };

    Ok(Response::new()
        .add_message(send_msg)
        .add_attribute("action", "claim_scholarship")
        .add_attribute("recipient", info.sender))
}

/* ================= TESTS ================= */

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    #[test]
    fn test_full_flow() {
        let mut deps = mock_dependencies();
        let admin = mock_info("admin_user", &[]);

        // 1. Instantiate (Added denom to fix compiler error)
        let inst_msg = InstantiateMsg {
            scholarship_amount: 1000,
            denom: "ustake".to_string(),
        };
        instantiate(deps.as_mut(), mock_env(), admin.clone(), inst_msg).unwrap();

        // 2. Register
        let student_addr = "student_1".to_string();
        register_student(deps.as_mut(), admin.clone(), student_addr.clone()).unwrap();

        // 3. Approve
        approve_student(deps.as_mut(), admin, student_addr.clone()).unwrap();

        // 4. Claim
        let student_info = mock_info(&student_addr, &[]);
        let res = claim_scholarship(deps.as_mut(), student_info).unwrap();

        // Check for BankMsg
        assert_eq!(res.messages.len(), 1);
    }
}
