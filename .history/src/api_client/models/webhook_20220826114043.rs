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
pub struct TransactionDetails{
    #[serde(rename = "tenantId")]
    pub tennant_id: String,s
}