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
    pbu id
pbu assetId
pbu source
pbu destination
pbu requestedAmount
pbu amountInfo
pbu feeInfo
pbu amount
pbu netAmount
pbu amountUSD
pbu serviceFee
pbu treatAsGrossAmount
pbu networkFee
pbu createdAt
pbu lastUpdated
pbu status
pbu txHash
pbu index
pbu subStatus
pbu sourceAddress
pbu destinationAddress
pbu destinationAddressDescription
pbu destinationTag
pbu signedBy
pbu createdBy
pbu rejectedBy
pbu addressType
pbu note
pbu exchangeTxId
pbu feeCurrency
pbu operation
pbu amlScreeningResult
pbu customerRefId
pbu numOfConfirmations
pbu networkRecords
pbu replacedTxHash
pbu externalTxId
pbu destinations
pbu blockInfo
pbu rewardsInfo
pbu authorizationInfo
pbu signedMessages
pbu extraParameters
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