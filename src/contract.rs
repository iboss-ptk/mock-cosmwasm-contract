#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::STUBS;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SetStubQuery { query, response } => {
            STUBS.save(deps.storage, query.0, &to_json_binary(&response)?)?;
        }
        ExecuteMsg::RemoveStubQuery { query } => {
            STUBS.remove(deps.storage, query.0);
        }
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    let response = STUBS.may_load(deps.storage, msg.0.clone())?;
    response.ok_or_else(|| ContractError::NoStubQuery { query: msg.0 })
}

#[cfg(test)]
mod tests {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{
        from_json,
        testing::{mock_dependencies, mock_env, mock_info},
        to_json_string,
    };

    use super::*;

    #[cw_serde]
    enum TestQueryMsg {
        QueryVariant { arg: String },
    }

    #[cw_serde]
    struct QueryResponse {
        value: String,
    }

    #[test]
    fn test_stub_query() {
        let mut deps = mock_dependencies();

        let query_msg = QueryMsg::new(&TestQueryMsg::QueryVariant {
            arg: "first".to_string(),
        })
        .unwrap();

        let query_response = to_json_string(&QueryResponse {
            value: "first".to_string(),
        })
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info("sender", &[]),
            ExecuteMsg::SetStubQuery {
                query: query_msg.clone(),
                response: query_response.clone(),
            },
        )
        .unwrap();

        let actual_query_response = query(deps.as_ref(), mock_env(), query_msg).unwrap();

        assert_eq!(
            query_response,
            from_json::<String>(&actual_query_response).unwrap()
        );

        let query_msg = QueryMsg::new(&TestQueryMsg::QueryVariant {
            arg: "first_no_stub".to_string(),
        })
        .unwrap();

        let err = query(deps.as_ref(), mock_env(), query_msg).unwrap_err();

        assert_eq!(
            err,
            ContractError::NoStubQuery {
                query: to_json_string(&TestQueryMsg::QueryVariant {
                    arg: "first_no_stub".to_string()
                })
                .unwrap()
            }
        );
    }
}
