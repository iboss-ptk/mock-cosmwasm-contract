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

pub fn set_stub_query_msg<Q, R>(query: &Q, response: &R) -> StdResult<ExecuteMsg>
where
    Q: Serialize + ?Sized,
    R: Serialize + ?Sized,
{
    Ok(ExecuteMsg::SetStubQuery {
        query: query_msg(query)?,
        response: to_json_string(&response)?,
    })
}

pub fn remove_stub_query_msg<Q>(query: &Q) -> StdResult<ExecuteMsg>
where
    Q: Serialize + ?Sized,
{
    Ok(ExecuteMsg::RemoveStubQuery {
        query: query_msg(query)?,
    })
}

#[cw_serde]
#[serde(transparent)]
pub struct QueryMsg(pub String);

pub fn query_msg<T>(msg: &T) -> StdResult<QueryMsg>
where
    T: Serialize + ?Sized,
{
    Ok(QueryMsg(to_json_string(&msg)?))
}
