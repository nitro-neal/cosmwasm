use cosmwasm_std::Uint128;
use cw20::Cw20ReceiveMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub count: i32,
}

// JoinLottery
// Claim
// (only owner) -> start lottery
// (only owner) -> end lottery
// (only owner) -> give lottery funds to random
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Increment {},
    Reset { count: i32 },
    JoinLottery(Cw20ReceiveMsg),
    ClaimLottery {},
}

// GetClaimAmount
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    GetCount {},
    GetClaimAmount {},
}

// ClaimResponse
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CountResponse {
    pub count: i32,
}

// ClaimResponse
// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ClaimAmountResponse {
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReceiveMsg {
    // Send sends token to an id with defined pot
    Send { id: Uint128 },
}
