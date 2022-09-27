use crate::{consts::{SUPPORTEDS_ASSETS, VAULT_ACCOUNTS, VAULT_ACCOUNTS_WITH_PAGE_INFO}, FireblocksError};

use super::{FireblocsApiExecutor, FireblocksPageMode, AssetTypeResponse, VaultAccountResponse, PagedVaultAccountsResponse, AssetResponse, VaultAssetResponse, CreateVaultAccountRequest};

#[derive(Clone)]
pub struct FireblocksSdk<T: FireblocsApiExecutor>{
    api_client: T,
}

impl<T: FireblocsApiExecutor> FireblocksSdk<T> {
    pub fn new(api_client: T) -> Self {
        Self { api_client }
    }

    pub async fn get_supported_assets(&self) -> Result<Vec<AssetTypeResponse>, FireblocksError>{
        return self.api_client.issue_get_request(SUPPORTEDS_ASSETS, FireblocksPageMode::Disabled).await;
    }

    pub async fn get_vault_accounts_with_page_info(&self) ->  Result<PagedVaultAccountsResponse, FireblocksError>{
        return self.api_client.issue_get_request(VAULT_ACCOUNTS_WITH_PAGE_INFO, FireblocksPageMode::Disabled).await;
    }

    pub async fn get_vault_by_account_id(&self, vault_account_id: String) ->  Result<VaultAccountResponse, FireblocksError>{
        return self.api_client.issue_get_request(&format!("{}/{}", VAULT_ACCOUNTS, vault_account_id), FireblocksPageMode::Disabled).await;
    }

    pub async fn get_vault_account_asset(&self, vault_account_id: String, asset_id: String) ->  Result<AssetResponse, FireblocksError>{
        return self.api_client.issue_get_request(&format!("{}/{}/{}", VAULT_ACCOUNTS, vault_account_id, asset_id), FireblocksPageMode::Disabled).await;
    }

    pub async fn refresh_vault_asset_balance(&self, vault_account_id: String, asset_id: String) ->  Result<AssetResponse, FireblocksError>{
        return self.api_client.issue_get_request(&format!("{}/{}/{}/balance", VAULT_ACCOUNTS, vault_account_id, asset_id), FireblocksPageMode::Disabled).await;
    }

    pub async fn create_vault_asset(&self, vault_account_id: String, asset_id: String) ->  Result<VaultAssetResponse, FireblocksError>{
        return self.api_client.issue_post_request(&format!("{}/{}/{}", VAULT_ACCOUNTS, vault_account_id, asset_id), None, None).await;
    }

    pub async fn create_vault_account(&self, name: String, hidden_on_ui: Option<bool>, customer_ref_id: Option<String>, auto_fuel: Option<bool>, idempotency_key: Option<String>) -> Result<VaultAccountResponse, FireblocksError>{
        let request = CreateVaultAccountRequest{name, hidden_on_ui: hidden_on_ui, customer_ref_id, auto_fuel};
        let body = serde_json::to_string(&request).unwrap();
        return self.api_client.issue_post_request(&format!("{}", VAULT_ACCOUNTS), Some(body), idempotency_key).await;
    }
}

