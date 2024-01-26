use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_json_string, StdResult};
use serde::Serialize;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    #[serde(rename = "@set_stub_query")]
    SetStubQuery { query: QueryMsg, response: String },

    #[serde(rename = "@remove_stub_query")]
    RemoveStubQuery { query: QueryMsg },
}

#[cw_serde]
#[serde(transparent)]
pub struct QueryMsg(pub String);

impl QueryMsg {
    pub fn new<T>(msg: &T) -> StdResult<Self>
    where
        T: Serialize + ?Sized,
    {
        Ok(Self(to_json_string(&msg)?))
    }
}
