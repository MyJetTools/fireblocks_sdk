use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultAccountResponse{
    pub id: String,
    pub name: String,
    #[serde(rename = "hiddenOnUI")]
    pub hidden_on_ui: bool,
    #[serde(rename = "customerRefId")]
    pub customer_ref_id: Option<String>,
    #[serde(rename = "autoFuel")]
    pub auto_fuel: Option<bool>,
    pub assets: Vec<AssetResponse>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetResponse{
    pub id: String,
    pub total: String,
    pub balance: Option<String>,
    pub available: String,
    pub pending: String,
    pub staked: Option<String>,
    pub frozen: String,
    #[serde(rename = "lockedAmount")]
    pub locked_amount: String,
    #[serde(rename = "maxBip44AddressIndexUsed")]
    pub max_bip_44_address_index_used: Option<i32>,
    #[serde(rename = "maxBip44ChangeAddressIndexUsed")]
    pub max_bip_44_change_address_index_used: Option<i32>,
    #[serde(rename = "totalStakedCPU")]
    pub total_staked_cpu: Option<String>,
    #[serde(rename = "totalStakedNetwork")]
    pub total_staked_network: Option<String>,
    #[serde(rename = "selfStakedCPU")]
    pub self_staked_cpu: Option<String>,
    #[serde(rename = "selfStakedNetwork")]
    pub self_staked_network: Option<String>,
    #[serde(rename = "pendingRefundCPU")]
    pub pending_refund_cpu: Option<String>,
    #[serde(rename = "pendingRefundNetwork")]
    pub pending_refund_network: Option<String>,
    #[serde(rename = "blockHeight")]
    pub block_height: Option<String>,
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BalanceRevardInfo{
    #[serde(rename = "pendingRewards")]
    pub pending_revards: String,

}

#[derive(Debug, Deserialize, Serialize)]
pub struct AllocatedBalances{
    #[serde(rename = "allocationId")]
    pub allocation_id: String,
    #[serde(rename = "thirdPartyAccountId")]
    pub third_party_account_id: Option<String>,
    pub affiliation: Option<String>,
    #[serde(rename = "virtualType")]
    pub virtual_type: Option<String>,
    pub total: String,
    pub available: String,
    pub pending: Option<String>,
    pub frozen: Option<String>,
    pub locked: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PagedVaultAccountsResponse{
    pub accounts: Vec<VaultAccountResponse>,
    pub paging: PagedVaultAccountsPagingResponse,
    #[serde(rename = "previousUrl")]
    pub previous_url: Option<String>,
    #[serde(rename = "nextUrl")]
    pub next_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PagedVaultAccountsPagingResponse{
    pub before: Option<String>,
    pub after: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultAssetResponse{
    pub id : String,
    pub address : String,
    #[serde(rename = "legacyAddress")]
    pub legacy_address : String,
    #[serde(rename = "enterpriseAddress")]
    pub enterprise_address : Option<String>,
    pub tag : String,
    #[serde(rename = "eosAccountName")]
    pub eos_account_name : Option<String>,
    pub status : Option<String>,
    #[serde(rename = "activationTxId")]
    pub activation_tx_id : Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateVaultAccountRequest{
    pub name: String,
    #[serde(rename = "hiddenOnUI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden_on_ui: Option<bool>,
    #[serde(rename = "customerRefId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
    #[serde(rename = "autoFuel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_fuel: Option<bool>,
}