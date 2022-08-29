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
            tennant_id: value["type"].as_str().unwrap().to_owned(),
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
    // pub fee_info: String,
    // pub amount: f64,
    // pub net_amount: f64,
    // pub amount_usd: f64,
    // pub service_fee: String,
    // pub treat_as_gross_amount: String,
    // pub network_fee: String,
    // pub created_at: String,
    // pub last_updated: String,
    // pub status: String,
    // pub tx_hash: String,
    // pub index: String,
    // pub sub_status: String,
    // pub source_address: String,
    // pub destination_address: String,
    // pub destination_address_description: String,
    // pub destination_tag: String,
    // pub signed_by: String,
    // pub created_by: String,
    // pub rejected_by: String,
    // pub address_type: String,
    // pub note: String,
    // pub exchange_tx_id: String,
    // pub fee_currency: String,
    // pub operation: String,
    // pub aml_screening_result: String,
    // pub customer_ref_id: String,
    // pub num_of_confirmations: String,
    // pub network_records: String,
    // pub replaced_tx_hash: String,
    // pub external_tx_id: String,
    // pub destinations: String,
    // pub block_info: String,
    // pub rewards_info: String,
    // pub authorization_info: String,
    // pub signed_messages: String,
    // pub extra_parameters: String,
    // pub tennant_id: String,
}


// pub enum TransferPeerPathResponseType{
//     VaultAccount,
//     ExchangeAccount,
//     InternalWallet,
//     ExternalWallet,
//     OneTimeAddress,
//     NetworkConnection,
//     FiatAccount,
//     Compound,
//     Unknown,
// }

// pub enum TransferPeerPathResponseSubType{

// }

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