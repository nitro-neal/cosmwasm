#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, to_binary, Addr, Api, BankMsg, Binary, CanonicalAddr, Coin, CosmosMsg, Deps,
    DepsMut, Env, MessageInfo, Response, StdResult, Uint128,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{
    ClaimAmountResponse, CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg, ReceiveMsg,
};
use crate::state::{State, STATE};
// use crate::state::{all_escrow_ids, Escrow, GenericBalance, ESCROWS};
use cw20::{Cw20Contract, Cw20ExecuteMsg, Cw20ReceiveMsg};

// use crate::state::{all_escrow_ids, Escrow, GenericBalance, ESCROWS};

// FOLLOW THIS - https://docs.cosmwasm.com/dev-academy/develop-smart-contract/develop/

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:lottery";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        amount: Uint128::new(0),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
        ExecuteMsg::JoinLottery(msg) => try_execute_join_lottery(deps, info, msg),
        ExecuteMsg::ClaimLottery {} => try_claim(deps),
    }
}

pub fn try_claim(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}

pub fn try_execute_join_lottery(
    deps: DepsMut,
    info: MessageInfo,
    wrapped: Cw20ReceiveMsg,
) -> Result<Response, ContractError> {
    // cw20 address authentication
    // let config = CONFIG.load(deps.storage)?;
    // if config.cw20_addr != info.sender {
    //     return Err(ContractError::Unauthorized {});
    // }

    let msg: ReceiveMsg = from_binary(&wrapped.msg)?;
    match msg {
        ReceiveMsg::Send { id } => receive_join_lottery_send(deps, id, wrapped.amount, info.sender),
    }
}

pub fn receive_join_lottery_send(
    deps: DepsMut,
    pot_id: Uint128,
    amountReceive: Uint128,
    cw20_addr: Addr,
) -> Result<Response, ContractError> {
    // load pot
    // let mut pot = POTS.load(deps.storage, pot_id.u128().into())?;

    // pot.collected += amount;

    // POTS.save(deps.storage, pot_id.u128().into(), &pot)?;

    let mut res = Response::new().add_attribute("collected", amountReceive);

    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.amount = amountReceive;
        Ok(state)
    })?;

    // if pot.collected >= pot.threshold {
    //     // Cw20Contract is a function helper that provides several queries and message builder.
    //     let cw20 = Cw20Contract(cw20_addr);
    //     // Build a cw20 transfer send msg, that send collected funds to target address
    //     let msg = cw20.call(Cw20ExecuteMsg::Transfer {
    //         recipient: pot.target_addr.into_string(),
    //         amount: pot.collected,
    //     })?;
    //     res = res.add_message(msg);
    // }

    Ok(res)
}

// // this is a helper to move the tokens, so the business logic is easy to read
// fn send_tokens<A: Api>(
//     api: &A,
//     from_address: &CanonicalAddr,
//     to_address: &CanonicalAddr,
//     amount: Vec<Coin>,
//     action: &str,
// ) -> HandleResult {
//     let from_human = api.human_address(from_address)?;
//     let to_human = api.human_address(to_address)?;
//     let log = vec![log("action", action), log("to", to_human.as_str())];

//     let r = HandleResponse {
//         messages: vec![CosmosMsg::Bank(BankMsg::Send {
//             to_address: to_human,
//             amount,
//         })],
//         data: None,
//     };
//     Ok(r)
// }

// fn send_tokens(to: &Addr, balance: &GenericBalance) -> StdResult<Vec<SubMsg>> {
//     let native_balance = &balance.native;
//     let mut msgs: Vec<SubMsg> = if native_balance.is_empty() {
//         vec![]
//     } else {
//         vec![SubMsg::new(BankMsg::Send {
//             to_address: to.into(),
//             amount: native_balance.to_vec(),
//         })]
//     };

//     let cw20_balance = &balance.cw20;
//     let cw20_msgs: StdResult<Vec<_>> = cw20_balance
//         .iter()
//         .map(|c| {
//             let msg = Cw20ExecuteMsg::Transfer {
//                 recipient: to.into(),
//                 amount: c.amount,
//             };
//             let exec = SubMsg::new(WasmMsg::Execute {
//                 contract_addr: c.address.to_string(),
//                 msg: to_binary(&msg)?,
//                 funds: vec![],
//             });
//             Ok(exec)
//         })
//         .collect();
//     msgs.append(&mut cw20_msgs?);
//     Ok(msgs)
// }

// USE THIS ONE - https://github.com/CosmWasm/cw-plus/blob/main/contracts/cw20-base/src/contract.rs#L318
// TUTORIAL -https://docs.cosmwasm.com/dev-academy/develop-smart-contract/develop/
// pub fn execute_send(
//     deps: DepsMut,
//     _env: Env,
//     info: MessageInfo,
//     contract: String,
//     amount: Uint128,
//     msg: Binary,
// ) -> Result<Response, ContractError> {
//     if amount == Uint128::zero() {
//         return Err(ContractError::InvalidZeroAmount {});
//     }

//     let rcpt_addr = deps.api.addr_validate(&contract)?;

//     // move the tokens to the contract
//     BALANCES.update(
//         deps.storage,
//         &info.sender,
//         |balance: Option<Uint128>| -> StdResult<_> {
//             Ok(balance.unwrap_or_default().checked_sub(amount)?)
//         },
//     )?;
//     BALANCES.update(
//         deps.storage,
//         &rcpt_addr,
//         |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
//     )?;

//     let res = Response::new()
//         .add_attribute("action", "send")
//         .add_attribute("from", &info.sender)
//         .add_attribute("to", &contract)
//         .add_attribute("amount", amount)
//         .add_message(
//             Cw20ReceiveMsg {
//                 sender: info.sender.into(),
//                 amount,
//                 msg,
//             }
//             .into_cosmos_msg(contract)?,
//         );
//     Ok(res)
// }

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}
pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> {
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
        QueryMsg::GetClaimAmount {} => to_binary(&query_claim_amount(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}

fn query_claim_amount(deps: Deps) -> StdResult<ClaimAmountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(ClaimAmountResponse {
        amount: state.amount,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::{coins, from_binary, Addr, CosmosMsg, WasmMsg};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: CountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }

    #[test]
    fn test_receive_send() {
        let mut deps = mock_dependencies();

        /* INIT */
        let msg = InstantiateMsg { count: 2 };
        let mut info = mock_info("creator", &[]);

        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        /* JOIN LOTTERY */
        let msg = ExecuteMsg::JoinLottery(Cw20ReceiveMsg {
            sender: String::from("cw20"),
            amount: Uint128::new(55),
            msg: to_binary(&ReceiveMsg::Send {
                id: Uint128::new(1),
            })
            .unwrap(),
        });

        info.sender = Addr::unchecked("cw20");
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        println!("Join Lottery {:?}", _res);

        /* QUERY CLAIM AMOUNT */
        let get_claim_amount_res =
            query(deps.as_ref(), mock_env(), QueryMsg::GetClaimAmount {}).unwrap();
        let value: ClaimAmountResponse = from_binary(&get_claim_amount_res).unwrap();
        println!("Claim amount res {:?}", value);

        // assert_eq!(
        //     pot,
        //     Pot {
        //         target_addr: Addr::unchecked("Some"),
        //         collected: Uint128::new(55),
        //         threshold: Uint128::new(100)
        //     }
        // );

        // let msg = ExecuteMsg::Receive(Cw20ReceiveMsg {
        //     sender: String::from("cw20"),
        //     amount: Uint128::new(55),
        //     msg: to_binary(&ReceiveMsg::Send {
        //         id: Uint128::new(1),
        //     })
        //     .unwrap(),
        // });
        // let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        // let msg = res.messages[0].clone().msg;
        // assert_eq!(
        //     msg,
        //     CosmosMsg::Wasm(WasmMsg::Execute {
        //         contract_addr: String::from("cw20"),
        //         msg: to_binary(&Cw20ExecuteMsg::Transfer {
        //             recipient: String::from("Some"),
        //             amount: Uint128::new(110)
        //         })
        //         .unwrap(),
        //         funds: vec![]
        //     })
        // );

        // // query pot
        // let msg = QueryMsg::GetPot {
        //     id: Uint128::new(1),
        // };
        // let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        // let pot: Pot = from_binary(&res).unwrap();
        // assert_eq!(
        //     pot,
        //     Pot {
        //         target_addr: Addr::unchecked("Some"),
        //         collected: Uint128::new(110),
        //         threshold: Uint128::new(100)
        //     }
        // );
    }
}
