#[cfg(not(feature = "library"))]
pub mod contract;

mod error;
pub mod msg;
mod state;

pub use crate::error::ContractError;
pub use crate::msg::{
    query_msg, remove_stub_query_msg, set_stub_query_msg, ExecuteMsg, InstantiateMsg, QueryMsg,
};

pub const WASM_BYTES: &[u8] = include_bytes!("./mock_cosmwasm_contract.wasm");
