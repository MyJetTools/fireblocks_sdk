use serde::{Serialize, Deserialize};

pub enum CallbackType{
    TransactionCreated,
    TransactionStatusUpdate,
    TransactionApprovalStatusUpdate,
    VaultAccountAdded,
    VaultAccountAssetAdded,
    InternalWalletAssetAdded,
    ExternalWalletAssetAdded,
    ExchangeAccountAdded,
    FiatAccountAdded,
    NetworkConnectionAdded,
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

pub struct TransferPeerPathResponse{
    
}