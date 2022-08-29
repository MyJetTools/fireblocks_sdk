use serde::{Serialize, Deserialize};
use serde_json::Value;

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
    Other(String)
}

impl CallbackType{
    pub async fn serialize(data: &str){
        let value: Value = serde_json::from_str(&data).unwrap();
        let callback_type = value["type"].as_str().unwrap().to_owned().as_str();

        match callback_type{
            "" => ""
        };


        println!("{}", callback_type);

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
    pub source: String,
    pub destination: String,
    pub requested_amount: String,
    pub amount_info: String,
    pub fee_info: String,
    pub amount: String,
    pub net_amountt: String,
    pub amount_usd: String,
    pub service_fee: String,
    pub treat_as_gross_amount: String,
    pub network_fee: String,
    pub created_at: String,
    pub last_updated: String,
    pub status: String,
    pub tx_hash: String,
    pub index: String,
    pub sub_status: String,
    pub source_address: String,
    pub destination_address: String,
    pub destination_address_description: String,
    pub destination_tag: String,
    pub signed_by: String,
    pub created_by: String,
    pub rejected_by: String,
    pub address_type: String,
    pub note: String,
    pub exchange_tx_id: String,
    pub fee_currency: String,
    pub operation: String,
    pub aml_screening_result: String,
    pub customer_ref_id: String,
    pub num_of_confirmations: String,
    pub network_records: String,
    pub replaced_tx_hash: String,
    pub external_tx_id: String,
    pub destinations: String,
    pub block_info: String,
    pub rewards_info: String,
    pub authorization_info: String,
    pub signed_messages: String,
    pub extra_parameters: String,
    pub tennant_id: String,
}

pub enum TransferPeerPathResponseType{
    VaultAccount,
    ExchangeAccount,
    InternalWallet,
    ExternalWallet,
    OneTimeAddress,
    NetworkConnection,
    FiatAccount,
    Compound,
    Unknown,
}

pub enum TransferPeerPathResponseSubType{

}

pub struct TransferPeerPathResponse{

}