use std::time::Duration;

use client_wallets_manager::{start, AppContext, VaultAccountsRepo, SettingsModel, WalletsRepo};
use fireblocks_sdk::{FireblocksApiClient, BaseApiTokenProvider, FireblocksSdk};
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    let settings = SettingsModel::load(".reachpay").await;
    
    let api_client = FireblocksApiClient::new(BaseApiTokenProvider::new(settings.fireblocks_api_key, settings.foreblock_secret_path.to_string()), None);
    let sdk = FireblocksSdk::new(api_client);
    let vault_repo = VaultAccountsRepo::new(&settings.db_connection_string);
    let wallets_repo = WalletsRepo::new(&settings.db_connection_string);

    let app = AppContext::new(sdk, vault_repo, wallets_repo);

    tokio::spawn(start(app, 8888));

    loop {
        sleep(Duration::from_secs(5)).await;
    }
}
