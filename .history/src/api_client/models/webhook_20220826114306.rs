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
pub requestedAmount: String,
pub amountInfo: String,
pub feeInfo: String,
pub amount: String,
pub netAmount: String,
pub amountUSD: String,
pub serviceFee: String,
pub treatAsGrossAmount: String,
pub networkFee: String,
pub createdAt: String,
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