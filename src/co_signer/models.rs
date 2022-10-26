use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoSignerCallback {
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "txId")]
    pub tx_id: String,
    pub operation: OperationType,
    #[serde(rename = "sourceType")]
    pub source_type: String,
    #[serde(rename = "sourceId")]
    pub source_id: String,
    #[serde(rename = "destType")]
    pub dest_type: String,
    #[serde(rename = "destAddressType")]
    pub dest_address_type: String,
    #[serde(rename = "destId")]
    pub dest_id: String,
    pub asset: String,
    pub amount: f64,
    #[serde(rename = "amountStr")]
    pub amount_str: String,
    #[serde(rename = "requestedAmount")]
    pub requested_amount: f64,
    #[serde(rename = "destAddress")]
    pub dest_address: String,
    pub fee: String,
    pub note: String,
    pub destinations: Vec<TransactionRequestCallbackDestination>,
    #[serde(rename = "extraParameters")]
    pub extra_parameters: Option<String>,
    #[serde(rename = "rawTx")]
    pub raw_tx: Option<Vec<RawTx>>,
    pub players: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum OperationType {
    #[serde(rename = "BURN")]
    Burn,
    #[serde(rename = "CONTRACT_CALL")]
    ContractCall,
    #[serde(rename = "MINT")]
    Mint,
    #[serde(rename = "RAW")]
    Raw,
    #[serde(rename = "REDEEM_FROM_COMPOUND")]
    RedeemFromCompound,
    #[serde(rename = "SUPPLY_TO_COMPOUND")]
    SupplyToCompound,
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "TYPED_MESSAGE")]
    TypedMessage,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum DstAddressType {
    #[serde(rename = "WHITELISTED")]
    Whitelisted,
    #[serde(rename = "ONE_TIME")]
    OneTime,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionRequestCallbackDestination{
    #[serde(rename = "amountNative")]
    pub amount_native : f64,
    #[serde(rename = "amountNativeStr")]
    pub amount_native_str : String,
    #[serde(rename = "amountUSD")]
    pub amount_usd : f64,
    #[serde(rename = "dstAddressType")]
    pub dst_address_type : DstAddressType,
    #[serde(rename = "dstId")]
    pub dst_id : String,
    #[serde(rename = "dstName")]
    pub dst_name : Option<String>,
    #[serde(rename = "dstSubType")]
    pub dst_sub_type : Option<String>,
    #[serde(rename = "dstType")]
    pub dst_type : String,
    #[serde(rename = "displayDstAddress")]
    pub display_dst_address : String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RawTx{
    #[serde(rename = "rawTx")]
    pub raw_tx: String,
    #[serde(rename = "keyDerivationPath")]
    pub key_derivation_path: Vec<f64>
}

#[derive(Debug, Deserialize, Serialize)]
pub enum CoSignerCallbackResponseAction {
    #[serde(rename = "APPROVE")]
    Approve,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "IGNORE")]
    Ignore
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoSignerCallbackResponse{
    pub action: CoSignerCallbackResponseAction,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "rejectionReason")]
    pub reject_reason: String
}