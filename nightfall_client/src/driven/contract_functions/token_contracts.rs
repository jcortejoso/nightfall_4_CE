//! Implementations of the [`TokenContract`] interface defined in `ports/contracts.rs`.

use super::contract_type_conversions::{Addr, Uint256};
use crate::{domain::error::TokenContractError, ports::contracts::TokenContract};
use alloy::{
    primitives::{Address, U256},
    providers::Provider,
    signers::local::PrivateKeySigner,
};
use ark_bn254::Fr as Fr254;
use ark_ff::{BigInteger, BigInteger256};
use ark_std::Zero;
use configuration::{addresses::get_addresses, settings::get_settings};
use lib::{
    blockchain_client::BlockchainClientConnection, error::BlockchainClientConnectionError,
    initialisation::get_blockchain_client_connection,
};
use log::info;
use nightfall_bindings::artifacts::{IERC20, IERC721, IERC1155, IERC3525};
use std::sync::Arc;

const APPROVAL_GAS_LIMIT: u64 = 200_000;

async fn get_provider_and_signer()
-> Result<(Arc<dyn Provider>, PrivateKeySigner), BlockchainClientConnectionError> {
    let client_lock = get_blockchain_client_connection().await;
    let (provider, signer) = {
        let guard = client_lock.read().await;
        (guard.get_client(), guard.get_signer())
    };
    Ok((provider, signer))
}

fn provider_error<E: std::fmt::Display>(e: E) -> BlockchainClientConnectionError {
    BlockchainClientConnectionError::ProviderError(format!("Contract error: {e}"))
}

fn log_transaction_details(
    request_id: Option<&str>,
    context: &str,
    from: Address,
    to: Address,
    nonce: u64,
    gas_limit: u64,
    gas_price: impl std::fmt::Debug,
    max_fee_per_gas: impl std::fmt::Debug,
    max_priority_fee_per_gas: impl std::fmt::Debug,
    value: impl std::fmt::Debug,
) {
    match request_id {
        Some(id) => info!(
            "{id} {context} tx -> from: {:?}, to: {:?}, value: {:?}, nonce: {:?}, gas_limit: {}, gas_price: {:?}, max_fee_per_gas: {:?}, max_priority_fee_per_gas: {:?}",
            from, to, value, nonce, gas_limit, gas_price, max_fee_per_gas, max_priority_fee_per_gas
        ),
        None => info!(
            "{context} tx -> from: {:?}, to: {:?}, value: {:?}, nonce: {:?}, gas_limit: {}, gas_price: {:?}, max_fee_per_gas: {:?}, max_priority_fee_per_gas: {:?}",
            from, to, value, nonce, gas_limit, gas_price, max_fee_per_gas, max_priority_fee_per_gas
        ),
    }
}

impl TokenContract for IERC20::IERC20Calls {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the token ID is zero
        if token_id != BigInteger256::zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC20 approvals should have a token ID of 0".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_value = Uint256::from(value);

        let (provider, signer) = get_provider_and_signer().await?;
        let signer_address = signer.address();
        let client = provider.root();
        let nonce = client
            .get_transaction_count(signer_address)
            .await
            .map_err(|e| provider_error(e))?;
        let gas_price = client
            .get_gas_price()
            .await
            .map_err(|e| provider_error(e))?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;

        log_transaction_details(
            None,
            "ERC20 approval",
            signer_address,
            solidity_erc_address.0,
            nonce,
            APPROVAL_GAS_LIMIT,
            gas_price.clone(),
            max_fee_per_gas.clone(),
            max_priority_fee_per_gas.clone(),
            U256::ZERO,
        );

        let call = IERC20::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_value.0)
            .nonce(nonce)
            .gas(APPROVAL_GAS_LIMIT)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id)
            .build_raw_transaction(signer)
            .await
            .map_err(|e| provider_error(e))?;

        client
            .send_raw_transaction(&call)
            .await
            .map_err(|e| provider_error(e))?
            .get_receipt()
            .await
            .map_err(|e| provider_error(format!("Failed to get transaction receipt: {e}")))?;

        Ok(())
    }
}

impl TokenContract for IERC721::IERC721Calls {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the value is zero
        if !value.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC721 approvals should have a value of 0".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_token_id = Uint256::from(token_id);

        let (provider, signer) = get_provider_and_signer().await?;
        let signer_address = signer.address();
        let client = provider.root();
        let nonce = client
            .get_transaction_count(signer_address)
            .await
            .map_err(|e| provider_error(e))?;
        let gas_price = client
            .get_gas_price()
            .await
            .map_err(|e| provider_error(e))?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;

        log_transaction_details(
            None,
            "ERC721 approval",
            signer_address,
            solidity_erc_address.0,
            nonce,
            APPROVAL_GAS_LIMIT,
            gas_price.clone(),
            max_fee_per_gas.clone(),
            max_priority_fee_per_gas.clone(),
            U256::ZERO,
        );

        let call = IERC721::new(solidity_erc_address.0, client.clone())
            .approve(solidity_approval_address, solidity_token_id.0)
            .nonce(nonce)
            .gas(APPROVAL_GAS_LIMIT)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id)
            .build_raw_transaction(signer)
            .await
            .map_err(|e| provider_error(e))?;

        client
            .send_raw_transaction(&call)
            .await
            .map_err(|e| provider_error(e))?
            .get_receipt()
            .await
            .map_err(|e| provider_error(format!("Failed to get transaction receipt: {e}")))?;

        Ok(())
    }
}

impl TokenContract for IERC1155::IERC1155Calls {
    async fn set_approval(
        erc_address: Fr254,
        value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Check the value is zero
        if value.is_zero() & token_id.is_zero() {
            return Err(TokenContractError::TokenTypeError(
                "ERC1155 approvals should have one of value or token ID non-zero".to_string(),
            ));
        }
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();

        let (provider, signer) = get_provider_and_signer().await?;
        let signer_address = signer.address();
        let client = provider.root();
        let nonce = client
            .get_transaction_count(signer_address)
            .await
            .map_err(|e| provider_error(e))?;
        let gas_price = client
            .get_gas_price()
            .await
            .map_err(|e| provider_error(e))?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;

        log_transaction_details(
            None,
            "ERC1155 approval",
            signer_address,
            solidity_erc_address.0,
            nonce,
            APPROVAL_GAS_LIMIT,
            gas_price.clone(),
            max_fee_per_gas.clone(),
            max_priority_fee_per_gas.clone(),
            U256::ZERO,
        );

        let call = IERC1155::new(solidity_erc_address.0, client.clone())
            .setApprovalForAll(solidity_approval_address, true)
            .nonce(nonce)
            .gas(APPROVAL_GAS_LIMIT)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id)
            .build_raw_transaction(signer)
            .await
            .map_err(|e| provider_error(e))?;

        client
            .send_raw_transaction(&call)
            .await
            .map_err(|e| provider_error(e))?
            .get_receipt()
            .await
            .map_err(|e| provider_error(format!("Failed to get transaction receipt: {e}")))?;

        Ok(())
    }
}

impl TokenContract for IERC3525::IERC3525Calls {
    async fn set_approval(
        erc_address: Fr254,
        _value: Fr254,
        token_id: BigInteger256,
    ) -> Result<(), TokenContractError> {
        // Perform type conversions
        let solidity_erc_address = Addr::try_from(erc_address)?;
        let solidity_approval_address = get_addresses().nightfall();
        let solidity_token_id = Uint256::from(token_id);

        let (provider, signer) = get_provider_and_signer().await?;
        let signer_address = signer.address();
        let client = provider.root();
        let nonce = client
            .get_transaction_count(signer_address)
            .await
            .map_err(|e| provider_error(e))?;
        let gas_price = client
            .get_gas_price()
            .await
            .map_err(|e| provider_error(e))?;
        let max_fee_per_gas = gas_price * 2;
        let max_priority_fee_per_gas = gas_price;

        log_transaction_details(
            None,
            "ERC3525 approval",
            signer_address,
            solidity_erc_address.0,
            nonce,
            APPROVAL_GAS_LIMIT,
            gas_price.clone(),
            max_fee_per_gas.clone(),
            max_priority_fee_per_gas.clone(),
            U256::ZERO,
        );

        let call = IERC3525::new(solidity_erc_address.0, client.clone())
            .approve_0(solidity_approval_address, solidity_token_id.0)
            .nonce(nonce)
            .gas(APPROVAL_GAS_LIMIT)
            .max_fee_per_gas(max_fee_per_gas)
            .max_priority_fee_per_gas(max_priority_fee_per_gas)
            .chain_id(get_settings().network.chain_id)
            .build_raw_transaction(signer)
            .await
            .map_err(|e| provider_error(e))?;

        client
            .send_raw_transaction(&call)
            .await
            .map_err(|e| provider_error(e))?
            .get_receipt()
            .await
            .map_err(|e| provider_error(format!("Failed to get transaction receipt: {e}")))?;

        Ok(())
    }
}
