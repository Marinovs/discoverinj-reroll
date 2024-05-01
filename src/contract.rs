use cosmwasm_std::{
    entry_point,
    from_json,
    to_json_binary,
    Addr,
    Binary,
    Deps,
    DepsMut,
    Env,
    MessageInfo,
    Order,
    Response,
    StdError,
    StdResult,
    Uint128,
};
use cw2::set_contract_version;
use cw721::Cw721ReceiveMsg;
use crate::{
    msg::{ ExecuteMsg, InstantiateMsg, NftReceiveMsg, QueryMsg, RerollData },
    state::{ Config, Reroll, CONFIG, REROLL_INFO },
    utils::transfer_token_message,
};

const CONTRACT_NAME: &str = "Discoverinj Reroll";
const CONTRACT_VERSION: &str = "1.0";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg
) -> StdResult<Response> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let config = Config {
        enabled: true,
        admin: info.sender.clone(),
        collection_address: msg.collection_address.clone(),
        roll_fees: msg.roll_fees.clone(),
        denom: msg.denom.clone(),
        decimals: msg.decimals.clone(),
    };

    CONFIG.save(deps.storage, &config)?;
    Ok(
        Response::default()
            .add_attribute("action", "init contract")
            .add_attribute("collection_address", msg.collection_address)
            .add_attribute("roll_fees", msg.roll_fees)
            .add_attribute("denom", msg.denom)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        ExecuteMsg::UpdateConfig { admin, enabled, roll_fees, denom } =>
            update_config(deps, info, admin, enabled, roll_fees, denom),
        ExecuteMsg::CreateReroll { nft_id } => create_roll(deps, env, info, nft_id),
        ExecuteMsg::ReceiveNft(wrapper) => execute_receive_nft(deps, info, wrapper),
    }
}

fn update_config(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<Addr>,
    enabled: Option<bool>,
    roll_fees: Option<Uint128>,
    denom: Option<String>
) -> StdResult<Response> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender.clone() != config.admin {
        return Err(StdError::generic_err("Unauthorized"));
    }

    if admin.is_some() {
        config.admin = admin.unwrap();
    }

    if enabled.is_some() {
        config.enabled = enabled.unwrap();
    }

    if roll_fees.is_some() {
        config.roll_fees = roll_fees.unwrap();
    }

    if denom.is_some() {
        config.denom = denom.unwrap();
    }

    Ok(Response::default())
}

fn create_roll(deps: DepsMut, env: Env, info: MessageInfo, nft_id: String) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if !config.enabled {
        return Err(StdError::generic_err("Reroll not enabled"));
    }
    let found = REROLL_INFO.load(deps.storage, nft_id.clone());
    if found.is_ok() {
        return Err(StdError::generic_err("Nft already rerolled"));
    }

    if info.funds[0].denom != config.denom || info.funds[0].amount != config.roll_fees {
        return Err(StdError::generic_err("Payment failed"));
    }

    let reroll = Reroll {
        sender: info.sender.clone(),
        nft_id: nft_id.clone(),
        timestamp: env.block.time.seconds(),
        rerolled: false,
    };

    REROLL_INFO.save(deps.storage, nft_id.clone(), &reroll)?;

    Ok(
        Response::default()
            .add_attribute("action", "create reroll")
            .add_attribute("collection_address", config.collection_address)
            .add_attribute("nft_id", nft_id)
    )
}

fn execute_receive_nft(
    deps: DepsMut,
    info: MessageInfo,
    wrapper: Cw721ReceiveMsg
) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;
    if !config.enabled {
        return Err(StdError::generic_err("Reroll not enabled"));
    }

    let msg: NftReceiveMsg = from_json(&wrapper.msg)?;

    match msg {
        NftReceiveMsg::Reroll {} => {
            if info.sender.clone() != config.collection_address.clone() {
                return Err(
                    StdError::generic_err(
                        format!(
                            "Wrong collection address, expected {} received {} ",
                            config.collection_address.clone(),
                            info.sender.clone()
                        )
                    )
                );
            }
            let found = REROLL_INFO.load(deps.storage, wrapper.token_id.clone());

            if found.is_err() {
                return Err(StdError::generic_err("Nft not found"));
            }

            let mut reroll = found.unwrap();

            reroll.rerolled = true;

            let msg = transfer_token_message(
                config.collection_address.clone().to_string(),
                "cw20".to_string(),
                Uint128::from(u64::pow(10, config.decimals as u32)),
                reroll.sender.clone()
            )?;

            REROLL_INFO.save(deps.storage, wrapper.token_id.clone(), &reroll)?;
            Ok(
                Response::default()
                    .add_attribute("action", "execute reroll")
                    .add_attribute("collection_address", config.collection_address)
                    .add_attribute("nft_id", wrapper.token_id)
                    .add_message(msg)
            )
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRerolls {} => to_json_binary(&query_rerolls(deps)?),
        QueryMsg::GetUserRerolls { address } => to_json_binary(&query_user_rerolls(deps, address)?),
    }
}

fn query_rerolls(deps: Deps) -> StdResult<Binary> {
    let mut rerolls: Vec<RerollData> = vec![];

    // Collect all rerolls into a vector
    for item in REROLL_INFO.range(deps.storage, None, None, Order::Ascending) {
        let (key, value) = item?;
        let key_str = String::from_utf8(key.into()).map_err(|_|
            StdError::generic_err("Invalid UTF-8 key")
        )?;
        rerolls.push(RerollData { id: key_str, reroll: value });
    }

    // Serialize the vector of rerolls to binary
    to_json_binary(&rerolls)
}

fn query_user_rerolls(deps: Deps, address: Addr) -> StdResult<Binary> {
    let rerolls: Vec<Reroll> = REROLL_INFO.range(deps.storage, None, None, Order::Ascending)
        .filter_map(|item| {
            item.ok().and_then(|(_, reroll)| {
                if reroll.sender == address { Some(reroll) } else { None }
            })
        })
        .collect();

    to_json_binary(&rerolls)
}
