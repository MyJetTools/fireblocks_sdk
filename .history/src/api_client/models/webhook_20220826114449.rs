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
    pub lastUpdated: String,
    pub status: String,
    pub txHash: String,
    pub index: String,
    pub subStatus: String,
    pub sourceAddress: String,
    pub destinationAddress: String,
    pub destinationAddressDescription: String,
    pub destinationTag: String,
    pub signedBy: String,
    pub createdBy: String,
    pub rejectedBy: String,
    pub addressType: String,
    pub note: String,
    pub exchangeTxId: String,
    pub feeCurrency: String,
    pub operation: String,
    pub amlScreeningResult: String,
    pub customerRefId: String,
    pub numOfConfirmations: String,
    pub networkRecords: String,
    pub replacedTxHash: String,
    pub externalTxId: String,
    pub destinations: String,
    pub blockInfo: String,
    pub rewardsInfo: String,
    pub authorizationInfo: String,
    pub signedMessages: String,
    pub extraParameters: String,
    pub tennant_id: String,
}