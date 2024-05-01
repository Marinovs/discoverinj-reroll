use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Addr, Uint128 };
use cw_storage_plus::{ Item, Map };

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub enabled: bool,
    pub collection_address: Addr,
    pub roll_fees: Uint128,
    pub denom: String,
    pub decimals: u8,
}

#[cw_serde]
pub struct Reroll {
    pub sender: Addr,
    pub nft_id: String,
    pub rerolled: bool,
    pub timestamp: u64,
}

pub const CONFIG_KEY: &str = "config";
pub const CONFIG: Item<Config> = Item::new(CONFIG_KEY);

pub const REROLL_INFO: Map<String, Reroll> = Map::new("reroll");
