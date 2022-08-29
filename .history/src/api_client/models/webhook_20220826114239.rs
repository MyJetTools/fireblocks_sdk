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
    id
assetId
source
destination
requestedAmount
amountInfo
feeInfo
amount
netAmount
amountUSD
serviceFee
treatAsGrossAmount
networkFee
createdAt
lastUpdated
status
txHash
index
subStatus
sourceAddress
destinationAddress
destinationAddressDescription
destinationTag
signedBy
createdBy
rejectedBy
addressType
note
exchangeTxId
feeCurrency
operation
amlScreeningResult
customerRefId
numOfConfirmations
networkRecords
replacedTxHash
externalTxId
destinations
blockInfo
rewardsInfo
authorizationInfo
signedMessages
extraParameters
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