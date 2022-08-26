use serde::{Serialize, Deserialize};
use serde_json::Value;

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

impl CallbackType{
    pub async fn serialize(data: &str)-> Self{
        let value: Value = serde_json::from_str(&data).unwrap();
        let callback_type = value["type"].as_str().unwrap().to_owned();
        let fireblocks_base_data = FireblocksCallbackBaseData{
            tennant_id: value["tenantId"].as_str().unwrap().to_owned(),
            timestamp: value["timestamp"].as_u64().unwrap(),
        };

        let data_value = value.get("data").unwrap();

        return match callback_type.as_str(){
            "TRANSACTION_CREATED" => {
                let data: TransactionDetails = serde_json::from_value(data_value.clone()).unwrap();
                CallbackType::TransactionCreated(fireblocks_base_data, data)
            },
            "TRANSACTION_STATUS_UPDATED" => {
                let data: TransactionDetails = serde_json::from_value(data_value.clone()).unwrap();
                CallbackType::TransactionStatusUpdate(fireblocks_base_data, data)
            },
            "TRANSACTION_APPROVAL_STATUS_UPDATED" => {
                let data: TransactionDetails = serde_json::from_value(data_value.clone()).unwrap();
                CallbackType::TransactionApprovalStatusUpdate(fireblocks_base_data, data)
            },
            "VAULT_ACCOUNT_ADDED" => CallbackType::VaultAccountAdded,
            "VAULT_ACCOUNT_ASSET_ADDED" => CallbackType::VaultAccountAssetAdded,
            "INTERNAL_WALLET_ASSET_ADDED" => CallbackType::InternalWalletAssetAdded,
            "EXTERNAL_WALLET_ASSET_ADDED" => CallbackType::ExternalWalletAssetAdded,
            "EXCHANGE_ACCOUNT_ADDED" => CallbackType::ExchangeAccountAdded,
            "FIAT_ACCOUNT_ADDED" => CallbackType::FiatAccountAdded,
            "NETWORK_CONNECTION_ADDED" => CallbackType::NetworkConnectionAdded,
            _ => Self::Other(fireblocks_base_data, data.to_string()),
        };

        // println!("{}", callback_type);

        // let callback_type = serde_json::from_str::<CallbackType>(&data).unwrap();
        // let serialized = serde_json::to_string(&callback_type).unwrap();
        // serialized
    }




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
    pub net_amount: f64,
    #[serde(rename = "amountUSD")]
    pub amount_usd: f64,
    pub service_fee: Option<f64>,
    pub treat_as_gross_amount: Option<bool>,
    pub network_fee: f64,
    pub created_at: u64,
    pub last_updated: u64,
    pub status: String,
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
    pub num_of_confirmations: i32,
    pub network_records: Option<Vec<NetworkRecord>>,
    pub replaced_tx_hash: Option<String>,
    pub external_tx_id: Option<String>,
    pub destinations: Vec<DestinationsResponse>,
    pub block_info: BlockInfo,
    pub rewards_info: Option<RewardsInfo>,
    // pub authorization_info: String,
    // pub signed_messages: String,
    // pub extra_parameters: String,
    // pub tennant_id: String,
}

pub enum TransactionDetailsStatus{
    Submitted,
    Queued,
    PendingAuthorization,
    PendingSignature,
    Broadcasting,
    Pending3rdPartyManualApproval,
    Pending3rdParty,
    Confirming,
    PartiallyCompleted,
    Cancelled,
    Rejected,
    Blocked,
    Failed,
    Other
}

impl TransactionDetails {
    pub fn get_transaction_status(&self) -> TransactionDetailsStatus{
        match self.status.as_str(){
            "SUBMITTED" => TransactionDetailsStatus::Submitted,
            "QUEUED" => TransactionDetailsStatus::Queued,
            "PENDING_AUTHORIZATION" => TransactionDetailsStatus::PendingAuthorization,
            "PENDING_SIGNATURE" => TransactionDetailsStatus::PendingSignature,
            "BROADCASTING" => TransactionDetailsStatus::Broadcasting,
            "PENDING_3RD_PARTY_MANUAL_APPROVAL" => TransactionDetailsStatus::Pending3rdPartyManualApproval,
            "PENDING_3RD_PARTY" => TransactionDetailsStatus::Pending3rdParty,
            "CONFIRMING" => TransactionDetailsStatus::Confirming,
            "PARTIALLY_COMPLETED" => TransactionDetailsStatus::PartiallyCompleted,
            "CANCELLED" => TransactionDetailsStatus::Cancelled,
            "REJECTED" => TransactionDetailsStatus::Rejected,
            "BLOCKED" => TransactionDetailsStatus::Blocked,
            "FAILED" => TransactionDetailsStatus::Failed,
            _ => TransactionDetailsStatus::Other
        }
    }
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
    pub network_fee: f64,
    pub asset_id: String,
    pub net_amount: f64,
    pub status: String,
    #[serde(rename = "type")]
    pub record_type: String,
    pub destination_address: String,
    pub source_address: String
}

pub enum NetworkStatus{
    Dropped,
    Broadcastions,
    Confirming,
    Failed,
    Confirmed
}

impl NetworkRecord {
    pub fn get_network_status(&self) -> NetworkStatus {
        match self.status.as_str() {
            "DROPPED" => NetworkStatus::Dropped,
            "BROADCASTED" => NetworkStatus::Broadcastions,
            "CONFIRMING" => NetworkStatus::Confirming,
            "FAILED" => NetworkStatus::Failed,
            "CONFIRMED" => NetworkStatus::Confirmed,
            _ => NetworkStatus::Dropped,
        }
    }

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
    pub net_amount: String,
    #[serde(rename = "amountUSD")]
    pub amount_usd: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeInfo{
    pub network_fee: String,
    pub service_fee: Option<String>,
}