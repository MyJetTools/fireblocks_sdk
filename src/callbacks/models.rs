use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub enum CallbackType{
    TransactionCreated(FireblocksCallbackBaseData, TransactionDetails),
    TransactionStatusUpdate(FireblocksCallbackBaseData, TransactionDetails),
    TransactionApprovalStatusUpdate(FireblocksCallbackBaseData, TransactionDetails),
    VaultAccountAdded,
    VaultAccountAssetAdded,
    InternalWalletAssetAdded,
    ExternalWalletAssetAdded,
    ExchangeAccountAdded,
    FiatAccountAdded,
    NetworkConnectionAdded,
    Other(FireblocksCallbackBaseData, String)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FireblocksCallbackBaseData{
    #[serde(rename = "tenantId")]
    pub tennant_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails{
    pub id: String,
    pub asset_id: String,
    pub source: TransferPeerPathResponse,
    pub destination: TransferPeerPathResponse,
    pub requested_amount: f64,
    pub amount_info: AmountInfo,
    pub fee_info: FeeInfo,
    pub amount: f64,
    pub net_amount: Option<f64>,
    #[serde(rename = "amountUSD")]
    pub amount_usd: f64,
    pub service_fee: Option<f64>,
    pub treat_as_gross_amount: Option<bool>,
    pub network_fee: Option<f64>,
    pub created_at: u64,
    pub last_updated: u64,
    pub status: TransactionDetailsStatus,
    pub tx_hash: String,
    pub index: Option<i32>,
    pub sub_status: String,
    pub source_address: String,
    pub destination_address: String,
    pub destination_address_description: String,
    pub destination_tag: String,
    pub signed_by: Vec<String>,
    pub created_by: String,
    pub rejected_by: String,
    pub address_type: String,
    pub note: String,
    pub exchange_tx_id: String,
    pub fee_currency: String,
    pub operation: String,
    pub aml_screening_result: Option<AmlScreeningResult>,
    pub customer_ref_id: Option<String>,
    pub num_of_confirmations: Option<i32>,
    pub network_records: Option<Vec<NetworkRecord>>,
    pub replaced_tx_hash: Option<String>,
    pub external_tx_id: Option<String>,
    pub destinations: Vec<DestinationsResponse>,
    pub block_info: BlockInfo,
    pub rewards_info: Option<RewardsInfo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TransactionDetailsStatus{
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "QUEUED")]
    Queued,
    #[serde(rename = "PENDING_AUTHORIZATION")]
    PendingAuthorization,
    #[serde(rename = "PENDING_SIGNATURE")]
    PendingSignature,
    #[serde(rename = "BROADCASTING")]
    Broadcasting,
    #[serde(rename = "PENDING_3RD_PARTY_MANUAL_APPROVAL")]
    Pending3rdPartyManualApproval,
    #[serde(rename = "PENDING_3RD_PARTY")]
    Pending3rdParty,
    #[serde(rename = "CONFIRMING")]
    Confirming,
    #[serde(rename = "PARTIALLY_COMPLETED")]
    PartiallyCompleted,
    #[serde(rename = "PENDING_AML_SCREENING")]
    PendingAmlScreening,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "REJECTED")]
    Rejected,
    #[serde(rename = "BLOCKED")]
    Blocked,
    #[serde(rename = "FAILED")]
    Failed
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockInfo{
    pub block_height: Option<String>,
    pub block_hash: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct RewardsInfo{
    pub src_rewards: String,
    pub dest_rewards: String,   
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DestinationsResponse{
    pub amount: String,
    pub destination: TransferPeerPathResponse,
    #[serde(rename = "amountUSD")]
    pub amount_usd: f64,
    pub destination_address: String,
    pub destination_address_description: String,
    pub aml_screening_result: Option<AmlScreeningResult>,
    pub customer_ref_id: String

}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetworkRecord{
    pub source: TransferPeerPathResponse,
    pub destination: TransferPeerPathResponse,
    pub tx_hash: String,
    pub network_fee: Option<f64>,
    pub asset_id: String,
    pub net_amount: Option<f64>,
    pub status: NetworkStatus,
    #[serde(rename = "type")]
    pub record_type: String,
    pub destination_address: String,
    pub source_address: String
}

#[derive(Debug, Deserialize, Serialize)]
pub enum NetworkStatus{
    #[serde(rename = "DROPPED")]
    Dropped,
    #[serde(rename = "BROADCASTED")]
    Broadcastions,
    #[serde(rename = "CONFIRMING")]
    Confirming,
    #[serde(rename = "FAILED")]
    Failed,
    #[serde(rename = "CONFIRMED")]
    Confirmed
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AmlScreeningResult{
    pub provider: String,
    pub payload: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferPeerPathResponse{
    #[serde(rename = "type")]
    pub response_type: String,
    pub id: String,
    pub name: String,
    pub sub_type: String,
}


#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmountInfo{
    pub amount: String,
    pub requested_amount: String,
    pub net_amount: Option<String>,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeInfo{
    pub network_fee: Option<String>,
    pub service_fee: Option<String>,
}