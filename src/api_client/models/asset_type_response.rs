use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AssetTypeResponse{
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub asset_type: String,
    #[serde(rename = "contractAddress")]
    pub contract_address: String,
    #[serde(rename = "nativeAsset")]
    pub native_asset: String,
    pub decimals: i32,
}