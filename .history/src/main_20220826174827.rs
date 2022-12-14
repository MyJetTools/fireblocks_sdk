use fireblocks_sdk::CallbackType;
use tokio::fs;


#[tokio::main]
async fn main() {
    let file = fs::read_to_string("./test.json").await.unwrap();
    let result = CallbackType::serialize(&file).await;

    println!("{:#?}", result);


    // println!("{}", file);

}

INSUFFICIENT_FUNDS - Not enough funds to fulfill the withdraw request
AMOUNT_TOO_SMALL - Attempt to withdraw an amount below the allowed minimum
UNSUPPORTED_ASSET - Asset is not supported
UNAUTHORISED__MISSING_PERMISSION - Third party (e.g. exchange) API missing permission
INVALID_SIGNATURE - Invalid transaction signature
API_INVALID_SIGNATURE - Third party (e.g. exchange) API call invalid signature
UNAUTHORISED__MISSING_CREDENTIALS - Missing third party (e.g. exchange) credentials
UNAUTHORISED__USER - Attempt to initiate or approve a transaction by an unauthorised user
UNAUTHORISED__DEVICE - Unauthorised user's device
INVALID_UNMANAGED_WALLET - Unmanaged wallet is disabled or does not exist
INVALID_EXCHANGE_ACCOUNT - Exchange account is disabled or does not exist
INSUFFICIENT_FUNDS_FOR_FEE - Not enough balance to fund the requested transaction
INVALID_ADDRESS - Unsupported address format
WITHDRAW_LIMIT - Transaction exceeds the exchange's withdraw limit
API_CALL_LIMIT - Exceeded third party (e.g. exchange) API call limit
ADDRESS_NOT_WHITELISTED - Attempt to withdraw from an exchange to a non whitelisted address
TIMEOUT - The transaction request has timed out
CONNECTIVITY_ERROR - Network error
THIRD_PARTY_INTERNAL_ERROR - Received an internal error response from a third party service
CANCELLED_EXTERNALLY - Transaction was canceled by a third party service
INVALID_THIRD_PARTY_RESPONSE - Unrecognized third party response
VAULT_WALLET_NOT_READY - Vault wallet is not ready
MISSING_DEPOSIT_ADDRESS - Could not retrieve a deposit address from the exchange
ONE_TIME_ADDRESS_DISABLED - Transfering to non-whitelisted addresses is disabled in your workspace.
INTERNAL_ERROR - Internal error while processing the transaction
UNKNOWN_ERROR - Unexpected error
AUTHORIZER_NOT_FOUND - No authorizer found to approve the operation or the only authorizer found is the initiator
INSUFFICIENT_RESERVED_FUNDING - Some assets require a minimum of reserved funds being kept on the account
MANUAL_DEPOSIT_ADDRESS_REQUIRED - Error while retrieving a deposit address from an exchange. Please generate a deposit address for your exchange account
INVALID_FEE - Transaction fee is not in the allowed range
ERROR_UNSUPPORTED_TRANSACTION_TYPE - Attempt to execute an unsupported transaction Type
UNSUPPORTED_OPERATION - Unsupported operation
3RD_PARTY_PROCESSING - The transaction is pending approval by the 3rd party service (e.g exchange)
PENDING_BLOCKCHAIN_CONFIRMATIONS - Pending Blockchain confirmations
3RD_PARTY_CONFIRMING - Pending confirmation on the exchange
CONFIRMED - Confirmed on the blockchain
3RD_PARTY_COMPLETED - Completed on the 3rd party service (e.g exchange)
REJECTED_BY_USER - The transaction was rejected by one of the signers
CANCELLED_BY_USER - The transaction was canceled via the Console or the API
3RD_PARTY_CANCELLED - Cancelled on the exchange
3RD_PARTY_REJECTED - Rejected or not approved in time by user
REJECTED_AML_SCREENING - Rejected on AML Screening
BLOCKED_BY_POLICY - Transaction is blocked due to a policy rule
FAILED_AML_SCREENING - AML screening failed
PARTIALLY_FAILED - Only for Aggregated transactions. One or more of the associated transaction records failed
3RD_PARTY_FAILED - Transaction failed at the exchange
DROPPED_BY_BLOCKCHAIN - The transaction was replaced by another transaction with higher fee
REJECTED_BY_BLOCKCHAIN - Transaction was rejected by the Blockchain due to too low fees, bad inputs or bad nonce
INVALID_FEE_PARAMS - Fee parameters are inconsistent or unknown.
MISSING_TAG_OR_MEMO - A tag or memo is required to send funds to a third party address, including all exchanges.
SIGNING_ERROR - The transaction signing failed, resubmit the transaction to sign again.
GAS_LIMIT_TOO_LOW - The transaction was rejected because the gas limit was set too low
TOO_MANY_INPUTS - The transaction includes more inputs than the allowed limit (only for UTXO based blockchains)
MAX_FEE_EXCEEDED - Gas price is currently above selected max fee
ACTUAL_FEE_TOO_HIGH - Chosen fee is below current price
INVALID_CONTRACT_CALL_DATA - Transaction data was not encoded properly
INVALID_NONCE_TOO_LOW - Illegal nonce
INVALID_NONCE_TOO_HIGH - Illegal nonce
INVALID_NONCE_FOR_RBF - No matching nonce
FAIL_ON_LOW_FEE - Current blockchain fee is higher than selected
TOO_LONG_MEMPOOL_CHAIN - Too many unconfirmed transactions from this address
TX_OUTDATED - Nonce already used
INCOMPLETE_USER_SETUP - MPC setup was not completed
SIGNER_NOT_FOUND - Signer not found
INVALID_TAG_OR_MEMO - Invalid Tag or Memo
ZERO_BALANCE_IN_PERMANENT_ADDRESS - Not enough BTC on legacy permanent address
NEED_MORE_TO_CREATE_DESTINATION - Insufficient funds for creating destination account
NON_EXISTING_ACCOUNT_NAME - Account does not exist
ENV_UNSUPPORTED_ASSET - Asset is not supported under this workspace settings

