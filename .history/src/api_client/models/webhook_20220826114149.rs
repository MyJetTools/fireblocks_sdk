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
    pub tennant_id: String,
    #[serde(rename = "requestedAmount")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
    #[serde(rename = "assetId")]
    pub tennant_id: String,
}