use cosmwasm_std::Binary;
use cw_storage_plus::Map;

pub const STUBS: Map<String, Binary> = Map::new("stubs");
