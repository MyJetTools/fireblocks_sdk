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
    pub id
pub assetId
pub source
pub destination
pub requestedAmount
pub amountInfo
pub feeInfo
pub amount
pub netAmount
pub amountUSD
pub serviceFee
pub treatAsGrossAmount
pub networkFee
pub createdAt
pub lastUpdated
pub status
pub txHash
pub index
pub subStatus
pub sourceAddress
pub destinationAddress
pub destinationAddressDescription
pub destinationTag
pub signedBy
pub createdBy
pub rejectedBy
pub addressType
pub note
pub exchangeTxId
pub feeCurrency
pub operation
pub amlScreeningResult
pub customerRefId
pub numOfConfirmations
pub networkRecords
pub replacedTxHash
pub externalTxId
pub destinations
pub blockInfo
pub rewardsInfo
pub authorizationInfo
pub signedMessages
pub extraParameters
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