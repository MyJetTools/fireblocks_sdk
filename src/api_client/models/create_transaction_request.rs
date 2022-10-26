use serde::{Deserialize, Serialize};

pub enum FireblocksSource {
    VaultAccount(String),
    ExchangeAccount(String),
    FiatAccount(String),
    GasStation(String),
}

impl Into<TransferPeerPath> for FireblocksSource {
    fn into(self) -> TransferPeerPath {
        match self{
            FireblocksSource::VaultAccount(id) => TransferPeerPath{ id, account_type: "VAULT_ACCOUNT".into() },
            FireblocksSource::ExchangeAccount(id) => TransferPeerPath{ id, account_type: "EXCHANGE_ACCOUNT".into() },
            FireblocksSource::FiatAccount(id) => TransferPeerPath{ id, account_type: "FIAT_ACCOUNT".into() },
            FireblocksSource::GasStation(id) => TransferPeerPath{ id, account_type: "GAS_STATION".into() },
        }
    }
}

pub enum FireblocksDestination {
    VaultAccount(String),
    ExchangeAccount(String),
    InternalWallet(String),
    ExternalWallet(String),
    OneTimeAddress(String, Option<String>),
    NetworkConnection(String),
    FiatAccount(String),
    Compound(String),
}

impl Into<DestinationTransferPeerPath> for FireblocksDestination {
    fn into(self) -> DestinationTransferPeerPath {
        match self{
            FireblocksDestination::VaultAccount(id) => DestinationTransferPeerPath{ destination_type: "VAULT_ACCOUNT".to_string(), id, one_time_address: None },
            FireblocksDestination::ExchangeAccount(id) => DestinationTransferPeerPath{ destination_type: "EXCHANGE_ACCOUNT".to_string(), id, one_time_address: None },
            FireblocksDestination::InternalWallet(id) => DestinationTransferPeerPath{ destination_type: "INTERNAL_WALLET".to_string(), id, one_time_address: None },
            FireblocksDestination::ExternalWallet(id) => DestinationTransferPeerPath{ destination_type: "EXTERNAL_WALLET".to_string(), id, one_time_address: None },
            FireblocksDestination::OneTimeAddress(address,tag) => {
                let tag = if let Some(tag) = tag{
                    tag
                }else{
                    "".to_string()
                };
                return DestinationTransferPeerPath{ destination_type: "ONE_TIME_ADDRESS".to_string().to_string(), id: "".to_string(), one_time_address: Some(OneTimeAddress { address, tag })}
            },
            FireblocksDestination::NetworkConnection(id) => DestinationTransferPeerPath{ destination_type: "NETWORK_CONNECTION".to_string(), id, one_time_address: None },
            FireblocksDestination::FiatAccount(id) => DestinationTransferPeerPath{ destination_type: "FIAT_ACCOUNT".to_string(), id, one_time_address: None },
            FireblocksDestination::Compound(id) => DestinationTransferPeerPath{ destination_type: "COMPOUND".to_string(), id, one_time_address: None },
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OneTimeAddress{
    pub address: String,
    pub tag: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TransferPeerPath{
    pub id: String,
    #[serde(rename = "type")]
    pub account_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DestinationTransferPeerPath{
    #[serde(rename = "type")]
    pub destination_type: String,
    pub id: String,
    #[serde(rename = "oneTimeAddress")]
    pub one_time_address: Option<OneTimeAddress>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FireblocksTransactionFeeLevel{
    #[serde(rename = "LOW")]
    Low,
    #[serde(rename = "MEDIUM")]
    Medium,
    #[serde(rename = "HIGH")]
    High
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTransactionRequest{
    #[serde(rename = "assetId")]
    pub asset_id: String,
    pub source: TransferPeerPath,
    pub destination: DestinationTransferPeerPath,
    pub amount: String,
    pub note: String,
    #[serde(rename = "treatAsGrossAmount")]
    pub treat_as_gross_amount: bool,
    #[serde(rename = "failOnLowFee")]
    pub fail_on_low_fee: bool,
    #[serde(rename = "feeLevel")]
    pub fee_level: FireblocksTransactionFeeLevel,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateTransactionResponse{
    pub id: String,
    pub status: String
}