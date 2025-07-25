use std::time::{Duration, Instant};
use sui_crypto::{ed25519::Ed25519PrivateKey, SuiSigner};
use sui_graphql_client::Client;
use sui_sdk_types::{Address, ExecutionStatus, TransactionEffects};
use sui_transaction_builder::{unresolved::Input, TransactionBuilder};

use crate::{
    error::{Result, SuiUtilsError},
    objects,
};

pub async fn new_with_gas(
    client: &Client,
    caller: Address,
    gas_budget: u64,
) -> Result<TransactionBuilder> {
    let mut builder = TransactionBuilder::new();
    // get all sui coins
    let sui_coins =
        objects::get_owned_coins(client, caller, Some("0x2::coin::Coin<0x2::sui::SUI>")).await?;
    // find the coin with the minimum balance according to the budget
    let gas_coin = sui_coins
        .iter()
        .find(|c| c.balance() >= gas_budget)
        .ok_or(SuiUtilsError::GasCoinNotFound)?;
    // build the gas input from the coin
    let gas_input: Input = (&client
        .object(gas_coin.id().to_owned().into(), None)
        .await?
        .ok_or(SuiUtilsError::InvalidGasInput)?)
        .into();
    // get the reference gas price
    let gas_price = client
        .reference_gas_price(None)
        .await?
        .ok_or(SuiUtilsError::ReferenceGasPriceError)?;

    builder.add_gas_objects(vec![gas_input.with_owned_kind()]);
    builder.set_gas_price(gas_price);
    builder.set_gas_budget(gas_budget);
    builder.set_sender(caller);

    Ok(builder)
}

pub async fn execute_and_wait_for_effects(
    client: &Client,
    builder: TransactionBuilder,
    pk: &Ed25519PrivateKey,
) -> Result<TransactionEffects> {
    let tx = builder
        .finish()
        .map_err(|e| SuiUtilsError::TransactionBuildingError(e.to_string()))?;
    let sig = pk
        .sign_transaction(&tx)
        .map_err(|e| SuiUtilsError::TransactionSigningError(e.to_string()))?;

    let effects = client.execute_tx(vec![sig], &tx).await?;
    // wait for the transaction to be finalized
    while client.transaction(tx.digest()).await?.is_none() {
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(100) {
            std::future::pending::<()>().await;
        }
    }

    match effects {
        Some(effects) => {
            let status = effects.status();
            if status == &ExecutionStatus::Success {
                Ok(effects)
            } else {
                Err(SuiUtilsError::TransactionExecutionError(format!(
                    "Transaction failed: {:?}",
                    status
                )))
            }
        }
        None => Err(SuiUtilsError::InvalidTransactionEffects),
    }
}
