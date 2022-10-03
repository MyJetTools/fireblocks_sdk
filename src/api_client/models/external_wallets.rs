use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalWallet{
    pub id: String,
    pub name: String,
    #[serde(rename = "customerRefId")]
    pub customer_ref_id: Option<String>,
    pub assets: Vec<ExternalWalletAsset>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalWalletAsset{
    pub id: String,
    pub status: String,
    #[serde(rename = "activationTime")]
    pub activation_time: Option<String>,
    pub address: Option<String>,
    pub tag: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateExternalWalletRequest{
    pub name: String,
    #[serde(rename = "customerRefId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_ref_id: Option<String>,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct CreateExternalWalletAssetRequest{
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}