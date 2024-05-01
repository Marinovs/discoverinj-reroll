use cosmwasm_schema::cw_serde;
use cosmwasm_std::{ Addr, Uint128 };
use cw721::Cw721ReceiveMsg;
use serde::{ Deserialize, Serialize };

use crate::state::Reroll;

#[cw_serde]
pub struct InstantiateMsg {
    pub collection_address: Addr,
    pub roll_fees: Uint128,
    pub denom: String,
    pub decimals: u8,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateConfig {
        admin: Option<Addr>,
        enabled: Option<bool>,
        roll_fees: Option<Uint128>,
        denom: Option<String>,
    },
    CreateReroll {
        nft_id: String,
    },
    ReceiveNft(Cw721ReceiveMsg),
}

#[cw_serde]
pub enum NftReceiveMsg {
    Reroll {},
}

#[derive(Serialize, Deserialize)]
pub struct RerollData {
    pub id: String,
    pub reroll: Reroll,
}

#[cw_serde]
pub enum QueryMsg {
    GetRerolls {},
    GetUserRerolls {
        address: Addr,
    },
}
