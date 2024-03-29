use std::fmt;
use std::str;

use flurl::FlUrlError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum FireblocksError {
    GetVaultAccountsInvalidParams(i32, String),
    GetVaultAccountsUnexpectedError(i32, String),
    PostVaultAccountsInvalidParams(i32, String),
    PostVaultAccountsUnexpectedError(i32, String),
    GetVaultAccountByIdNotFound(i32, String),
    GetVaultAccountByIdUnexpectedError(i32, String),
    GetVaultAssetFailedNotFound(i32, String),
    GetVaultAssetFailedUnexpectedError(i32, String),
    CreateVaultAssetFailedInvalidParams(i32, String),
    CreateVaultAssetFailedUnexpectedError(i32, String),
    CreateVaultAssetAddressFailedInvalidParams(i32, String),
    CreateVaultAssetAddressFailedUnexpectedError(i32, String),
    GetVaultAssetAddressFailedUnexpectedError(i32, String),
    PutVaultAccountsInvalidParams(i32, String),
    PutVaultAccountsUnexpectedError(i32, String),
    PutAddressIdInvalidParams(i32, String),
    PutAddressIdUnexpectedError(i32, String),
    HideVaultAccountNotFound(i32, String),
    HideVaultAccountSendFailed(i32, String),
    HideVaultAccountInvalidParams(i32, String),
    UnhideVaultAccountNotFound(i32, String),
    UnhideVaultAccountSendFailed(i32, String),
    UnhideVaultAccountInvalidParams(i32, String),
    CreateVaultAssetUnsupportedError(i32, String),
    CreateVaultAssetNotAllowedError(i32, String),
    MaxSpendableAmountAssetNotAllowedError(i32, String),
    MaxSpendableAmountInvalidParameters(i32, String),
    MaxSpendableAmountInternalError(i32, String),
    MaxSpendableAmountUnexpectedError(i32, String),
    GetVaultAssetsBlanceInvalidParameters(i32, String),
    GetVaultAssetsBlanceNotFoundError(i32, String),
    GetVaultAssetsBlanceUnexpectedError(i32, String),
    CreateVaultAssetFailedInvalidAssetTestnet(i32, String),
    GetExchangeAccountsUnexpectedError(i32, String),
    GetExchangeAccountByIdUnexpectedError(i32, String),
    GetExchangeAccountByIdNotFound(i32, String),
    GetExchangeAssetByIdUnexpectedError(i32, String),
    GetExchangeAssetByIdNotFound(i32, String),
    InternalTransferInvalidParams(i32, String),
    InternalTransferUnexpectedError(i32, String),
    GetInternalWalletsUnexpectedError(i32, String),
    CreateInternalWalletError(i32, String),
    GetInternalWalletByIdUnexpectedError(i32, String),
    GetInternalWalletByIdInvalidParams(i32, String),
    DeleteInternalWalletByIdUnexpectedError(i32, String),
    DeleteInternalWalletByIdInvalidParams(i32, String),
    CreateInternalWalletAssetUnexpectedError(i32, String),
    CreateInternalWalletAssetInvalidParams(i32, String),
    GetInternalWalletAssetInvalidParams(i32, String),
    GetInternalWalletAssetUnexpectedError(i32, String),
    DeleteInternalWalletAssetInvalidParams(i32, String),
    DeleteInternalWalletAssetUnexpectedError(i32, String),
    CreateInternalWalletAssetUnsupportedError(i32, String),
    CreateInternalWalletAssetNotAllowedError(i32, String),
    CreateInternalWalletAssetAlreadyExistsError(i32, String),
    CreateInternalWalletAsssetInvalidAddress(i32, String),
    GetExternalWalletsUnexpectedError(i32, String),
    CreateExternalWalletError(i32, String),
    GetExternalWalletByIdUnexpectedError(i32, String),
    GetExternalWalletByIdInvalidParams(i32, String),
    DeleteExternalWalletByIdUnexpectedError(i32, String),
    DeleteExternalWalletByIdInvalidParams(i32, String),
    CreateExternalWalletAssetUnexpectedError(i32, String),
    CreateExternalWalletAssetInvalidParams(i32, String),
    GetExternalWalletAssetInvalidParams(i32, String),
    GetExternalWalletAssetUnexpectedError(i32, String),
    DeleteExternalWalletAssetInvalidParams(i32, String),
    DeleteExternalWalletAssetUnexpectedError(i32, String),
    CreateExternalWalletAssetUnsupportedError(i32, String),
    CreateExternalWalletAssetNotAllowedError(i32, String),
    CreateExternalWalletAssetAlreadyExistsError(i32, String),
    CreateExternalWalletAssetInvalidAddress(i32, String),
    CreateTransactionUnsupportedAction(i32, String),
    CreateTransactionSourceBalance(i32, String),
    CreateTransactionDestinationBalance(i32, String),
    CreateTransactionUnexpectedError(i32, String),
    GetTransactionNotFound(i32, String),
    CancelTransactionFailed(i32, String),
    GetTransactionInvalidParams(i32, String),
    CreateTransactionInvalidParams(i32, String),
    CreateTransactionUnmanagedWalletContainerNotFound(i32, String),
    CreateTransactionUnmanagedWalletNotFound(i32, String),
    CreateTransactionUnmanagedWalletSuspended(i32, String),
    SetConfirmationThresholdFailedTransactions(i32, String),
    SetConfirmationThresholdFailedTxHash(i32, String),
    GetTransactionsByTxhashNotFound(i32, String),
    CreateTransactionDestinationRippleInvalid(i32, String),
    CreateTransactionRippleMissingTag(i32, String),
    UnfreezeTransactionFailed(i32, String),
    CreateTransactionSourceError(i32, String),
    CreateTransactionDestinationError(i32, String),
    ValidateAddressInvalidParameter(i32, String),
    ValidateAddressUnexpectedError(i32, String),
    ValidateAddressNotFound(i32, String),
    CreateTransactionInvalidAmount(i32, String),
    CreateTransactionInvalidDestTag(i32, String),
    FreezeTransactionFailed(i32, String),
    ListUnspentsUnknownAsset(i32, String),
    ListUnspentsUnexpectedError(i32, String),
    RestrictedSource(i32, String),
    CreateTransactionExternalIdAlreadyExists(i32, String),
    DropTransactionFailed(i32, String),
    GasPriceTooLowForRbf(i32, String),
    RbfTransactionAlreadyMined(i32, String),
    GetSupportedAssetsUnexpectedError(i32, String),
    GetConnectionByIdNotFound(i32, String),
    GetConnectionByIdUnauthorized(i32, String),
    GetFiatAccountByIdUnexpectedError(i32, String),
    GetFiatAccountByIdNotFound(i32, String),
    GetFiatAssetByIdNotFound(i32, String),
    RedeemToDdaInvalidParams(i32, String),
    RedeemToDdaUnexpectedError(i32, String),
    DepositFromDdaInvalidParams(i32, String),
    DepositFromDdaUnexpectedError(i32, String),
    TransferAssistServiceError(i32, String),
    EstimateFeesUnknownAsset(i32, String),
    EstimateFeesInvalidAsset(i32, String),
    EstimateFeesCalculatingFailed(i32, String),
    EstimateFeesInvalidParams(i32, String),
    EstimateFeesInsufficientFunds(i32, String),
    PublicKeyUnexpectedError(i32, String),
    PublicKeyNotFound(i32, String),
    GasStationInvalidParams(i32, String),
    GasStationUnexpectedError(i32, String),
    AmlInvalidParams(i32, String),
    AmlUnexpectedError(i32, String),
    AmlPolicyNotFound(i32, String),
    AmlProviderConfigurationNotFound(i32, String),
    AllocateUnkownAsset(i32, String),
    AllocatedAccountNotFound(i32, String),
    AllocateInternalError(i32, String),
    AllocateToFeeBankProhibited(i32, String),
    InvalidCustomerRefId(i32, String),
    AccessDeniedForIp(i32, String),
    IdempotencyKeyInvalid(i32, String),
    IdempotencyKeyInProgress(i32, String),
    Unexpected(String),
    NetworkError(String),
    ResponseSerializeError(String, String),
}

impl From<FlUrlError> for FireblocksError {
    fn from(value: FlUrlError) -> Self {
        Self::NetworkError(format!("{:?}", value))
    }
}

impl fmt::Display for FireblocksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fireblocks error. Error: {:?}", self)
    }
}

impl FireblocksError {
    pub fn serialize_error(body: Vec<u8>) -> FireblocksError {
        let result: Result<FireblocksErrorResponse, serde_json::Error> =
            serde_json::from_slice(&body[..]);
        let result = result.unwrap();

        match &result.code {
            1000 => FireblocksError::GetVaultAccountsInvalidParams(1000, result.message),
            1001 => FireblocksError::GetVaultAccountsUnexpectedError(1001, result.message),
            1002 => FireblocksError::PostVaultAccountsInvalidParams(1002, result.message),
            1003 => FireblocksError::PostVaultAccountsUnexpectedError(1003, result.message),
            1004 => FireblocksError::GetVaultAccountByIdNotFound(1004, result.message),
            1005 => FireblocksError::GetVaultAccountByIdUnexpectedError(1005, result.message),
            1006 => FireblocksError::GetVaultAssetFailedNotFound(1006, result.message),
            1007 => FireblocksError::GetVaultAssetFailedUnexpectedError(1007, result.message),
            1008 => FireblocksError::CreateVaultAssetFailedInvalidParams(1008, result.message),
            1009 => FireblocksError::CreateVaultAssetFailedUnexpectedError(1009, result.message),
            1010 => {
                FireblocksError::CreateVaultAssetAddressFailedInvalidParams(1010, result.message)
            }
            1011 => {
                FireblocksError::CreateVaultAssetAddressFailedUnexpectedError(1011, result.message)
            }
            1012 => {
                FireblocksError::GetVaultAssetAddressFailedUnexpectedError(1012, result.message)
            }
            1013 => FireblocksError::PutVaultAccountsInvalidParams(1013, result.message),
            1014 => FireblocksError::PutVaultAccountsUnexpectedError(1014, result.message),
            1015 => FireblocksError::PutAddressIdInvalidParams(1015, result.message),
            1016 => FireblocksError::PutAddressIdUnexpectedError(1016, result.message),
            1017 => FireblocksError::HideVaultAccountNotFound(1017, result.message),
            1018 => FireblocksError::HideVaultAccountSendFailed(1018, result.message),
            1019 => FireblocksError::HideVaultAccountInvalidParams(1019, result.message),
            1020 => FireblocksError::UnhideVaultAccountNotFound(1020, result.message),
            1021 => FireblocksError::UnhideVaultAccountSendFailed(1021, result.message),
            1022 => FireblocksError::UnhideVaultAccountInvalidParams(1022, result.message),
            1025 => FireblocksError::CreateVaultAssetUnsupportedError(1025, result.message),
            1026 => FireblocksError::CreateVaultAssetNotAllowedError(1026, result.message),
            1027 => FireblocksError::MaxSpendableAmountAssetNotAllowedError(1027, result.message),
            1028 => FireblocksError::MaxSpendableAmountInvalidParameters(1028, result.message),
            1029 => FireblocksError::MaxSpendableAmountInternalError(1029, result.message),
            1030 => FireblocksError::MaxSpendableAmountUnexpectedError(1030, result.message),
            1031 => FireblocksError::GetVaultAssetsBlanceInvalidParameters(1031, result.message),
            1032 => FireblocksError::GetVaultAssetsBlanceNotFoundError(1032, result.message),
            1033 => FireblocksError::GetVaultAssetsBlanceUnexpectedError(1033, result.message),
            1034 => {
                FireblocksError::CreateVaultAssetFailedInvalidAssetTestnet(1034, result.message)
            }
            1101 => FireblocksError::GetExchangeAccountsUnexpectedError(1101, result.message),
            1102 => FireblocksError::GetExchangeAccountByIdUnexpectedError(1102, result.message),
            1103 => FireblocksError::GetExchangeAccountByIdNotFound(1103, result.message),
            1104 => FireblocksError::GetExchangeAssetByIdUnexpectedError(1104, result.message),
            1105 => FireblocksError::GetExchangeAssetByIdNotFound(1105, result.message),
            1106 => FireblocksError::InternalTransferInvalidParams(1106, result.message),
            1107 => FireblocksError::InternalTransferUnexpectedError(1107, result.message),
            1201 => FireblocksError::GetInternalWalletsUnexpectedError(1201, result.message),
            1202 => FireblocksError::CreateInternalWalletError(1202, result.message),
            1203 => FireblocksError::GetInternalWalletByIdUnexpectedError(1203, result.message),
            1204 => FireblocksError::GetInternalWalletByIdInvalidParams(1204, result.message),
            1205 => FireblocksError::DeleteInternalWalletByIdUnexpectedError(1205, result.message),
            1206 => FireblocksError::DeleteInternalWalletByIdInvalidParams(1206, result.message),
            1207 => FireblocksError::CreateInternalWalletAssetUnexpectedError(1207, result.message),
            1208 => FireblocksError::CreateInternalWalletAssetInvalidParams(1208, result.message),
            1209 => FireblocksError::GetInternalWalletAssetInvalidParams(1209, result.message),
            1210 => FireblocksError::GetInternalWalletAssetUnexpectedError(1210, result.message),
            1211 => FireblocksError::DeleteInternalWalletAssetInvalidParams(1211, result.message),
            1212 => FireblocksError::DeleteInternalWalletAssetUnexpectedError(1212, result.message),
            1213 => {
                FireblocksError::CreateInternalWalletAssetUnsupportedError(1213, result.message)
            }
            1214 => FireblocksError::CreateInternalWalletAssetNotAllowedError(1214, result.message),
            1215 => {
                FireblocksError::CreateInternalWalletAssetAlreadyExistsError(1215, result.message)
            }
            1216 => FireblocksError::CreateInternalWalletAsssetInvalidAddress(1216, result.message),
            1301 => FireblocksError::GetExternalWalletsUnexpectedError(1301, result.message),
            1302 => FireblocksError::CreateExternalWalletError(1302, result.message),
            1303 => FireblocksError::GetExternalWalletByIdUnexpectedError(1303, result.message),
            1304 => FireblocksError::GetExternalWalletByIdInvalidParams(1304, result.message),
            1305 => FireblocksError::DeleteExternalWalletByIdUnexpectedError(1305, result.message),
            1306 => FireblocksError::DeleteExternalWalletByIdInvalidParams(1306, result.message),
            1307 => FireblocksError::CreateExternalWalletAssetUnexpectedError(1307, result.message),
            1308 => FireblocksError::CreateExternalWalletAssetInvalidParams(1308, result.message),
            1309 => FireblocksError::GetExternalWalletAssetInvalidParams(1309, result.message),
            1310 => FireblocksError::GetExternalWalletAssetUnexpectedError(1310, result.message),
            1311 => FireblocksError::DeleteExternalWalletAssetInvalidParams(1311, result.message),
            1312 => FireblocksError::DeleteExternalWalletAssetUnexpectedError(1312, result.message),
            1313 => {
                FireblocksError::CreateExternalWalletAssetUnsupportedError(1313, result.message)
            }
            1314 => FireblocksError::CreateExternalWalletAssetNotAllowedError(1314, result.message),
            1315 => {
                FireblocksError::CreateExternalWalletAssetAlreadyExistsError(1315, result.message)
            }
            1316 => FireblocksError::CreateExternalWalletAssetInvalidAddress(1316, result.message),
            1401 => FireblocksError::CreateTransactionUnsupportedAction(1401, result.message),
            1402 => FireblocksError::CreateTransactionSourceBalance(1402, result.message),
            1403 => FireblocksError::CreateTransactionDestinationBalance(1403, result.message),
            1404 => FireblocksError::CreateTransactionUnexpectedError(1404, result.message),
            1405 => FireblocksError::GetTransactionNotFound(1405, result.message),
            1406 => FireblocksError::CancelTransactionFailed(1406, result.message),
            1408 => FireblocksError::GetTransactionInvalidParams(1408, result.message),
            1409 => FireblocksError::CreateTransactionInvalidParams(1409, result.message),
            1410 => FireblocksError::CreateTransactionUnmanagedWalletContainerNotFound(
                1410,
                result.message,
            ),
            1411 => FireblocksError::CreateTransactionUnmanagedWalletNotFound(1411, result.message),
            1412 => {
                FireblocksError::CreateTransactionUnmanagedWalletSuspended(1412, result.message)
            }
            1421 => {
                FireblocksError::SetConfirmationThresholdFailedTransactions(1421, result.message)
            }
            1422 => FireblocksError::SetConfirmationThresholdFailedTxHash(1422, result.message),
            1423 => FireblocksError::GetTransactionsByTxhashNotFound(1423, result.message),
            1424 => {
                FireblocksError::CreateTransactionDestinationRippleInvalid(1424, result.message)
            }
            1425 => FireblocksError::CreateTransactionRippleMissingTag(1425, result.message),
            1426 => FireblocksError::UnfreezeTransactionFailed(1426, result.message),
            1427 => FireblocksError::CreateTransactionSourceError(1427, result.message),
            1428 => FireblocksError::CreateTransactionDestinationError(1428, result.message),
            1429 => FireblocksError::ValidateAddressInvalidParameter(1429, result.message),
            1430 => FireblocksError::ValidateAddressUnexpectedError(1430, result.message),
            1431 => FireblocksError::ValidateAddressNotFound(1431, result.message),
            1432 => FireblocksError::CreateTransactionInvalidAmount(1432, result.message),
            1433 => FireblocksError::CreateTransactionInvalidDestTag(1433, result.message),
            1434 => FireblocksError::FreezeTransactionFailed(1434, result.message),
            1435 => FireblocksError::ListUnspentsUnknownAsset(1435, result.message),
            1436 => FireblocksError::ListUnspentsUnexpectedError(1436, result.message),
            1437 => FireblocksError::RestrictedSource(1437, result.message),
            1438 => FireblocksError::CreateTransactionExternalIdAlreadyExists(1438, result.message),
            1440 => FireblocksError::DropTransactionFailed(1440, result.message),
            1441 => FireblocksError::GasPriceTooLowForRbf(1441, result.message),
            1443 => FireblocksError::RbfTransactionAlreadyMined(1443, result.message),
            1501 => FireblocksError::GetSupportedAssetsUnexpectedError(1501, result.message),
            1601 => FireblocksError::GetConnectionByIdNotFound(1601, result.message),
            1602 => FireblocksError::GetConnectionByIdUnauthorized(1602, result.message),
            1702 => FireblocksError::GetFiatAccountByIdUnexpectedError(1702, result.message),
            1703 => FireblocksError::GetFiatAccountByIdNotFound(1703, result.message),
            1705 => FireblocksError::GetFiatAssetByIdNotFound(1705, result.message),
            1706 => FireblocksError::RedeemToDdaInvalidParams(1706, result.message),
            1707 => FireblocksError::RedeemToDdaUnexpectedError(1707, result.message),
            1708 => FireblocksError::DepositFromDdaInvalidParams(1708, result.message),
            1709 => FireblocksError::DepositFromDdaUnexpectedError(1709, result.message),
            1801 => FireblocksError::TransferAssistServiceError(1801, result.message),
            1901 => FireblocksError::EstimateFeesUnknownAsset(1901, result.message),
            1902 => FireblocksError::EstimateFeesInvalidAsset(1902, result.message),
            1903 => FireblocksError::EstimateFeesCalculatingFailed(1903, result.message),
            1904 => FireblocksError::EstimateFeesInvalidParams(1904, result.message),
            1905 => FireblocksError::EstimateFeesInsufficientFunds(1905, result.message),
            2001 => FireblocksError::PublicKeyUnexpectedError(2001, result.message),
            2002 => FireblocksError::PublicKeyNotFound(2002, result.message),
            2101 => FireblocksError::GasStationInvalidParams(2101, result.message),
            2102 => FireblocksError::GasStationUnexpectedError(2102, result.message),
            2103 => FireblocksError::AmlInvalidParams(2103, result.message),
            2104 => FireblocksError::AmlUnexpectedError(2104, result.message),
            2105 => FireblocksError::AmlPolicyNotFound(2105, result.message),
            2106 => FireblocksError::AmlProviderConfigurationNotFound(2106, result.message),
            2201 => FireblocksError::AllocateUnkownAsset(2201, result.message),
            2202 => FireblocksError::AllocatedAccountNotFound(2202, result.message),
            2203 => FireblocksError::AllocateInternalError(2203, result.message),
            2204 => FireblocksError::AllocateToFeeBankProhibited(2204, result.message),
            9001 => FireblocksError::InvalidCustomerRefId(9001, result.message),
            9002 => FireblocksError::AccessDeniedForIp(9002, result.message),
            9003 => FireblocksError::IdempotencyKeyInvalid(9003, result.message),
            9005 => FireblocksError::IdempotencyKeyInProgress(9005, result.message),
            _ => FireblocksError::Unexpected(result.message),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct FireblocksErrorResponse {
    pub message: String,
    pub code: i32,
}
