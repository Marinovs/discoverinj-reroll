use cosmwasm_std::{ to_json_binary, Addr, BankMsg, Coin, CosmosMsg, StdResult, Uint128, WasmMsg };
use cw20::Cw20ExecuteMsg;

pub fn transfer_token_message(
    denom: String,
    is_cw20: bool,
    amount: Uint128,
    receiver: Addr
) -> StdResult<CosmosMsg> {
    if !is_cw20 {
        Ok(
            (BankMsg::Send {
                to_address: receiver.clone().into(),
                amount: vec![Coin {
                    denom: denom.clone(),
                    amount,
                }],
            }).into()
        )
    } else {
        Ok(
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: denom.clone(),
                funds: vec![],
                msg: to_json_binary(
                    &(Cw20ExecuteMsg::Transfer {
                        recipient: receiver.clone().into(),
                        amount,
                    })
                )?,
            })
        )
    }
}
