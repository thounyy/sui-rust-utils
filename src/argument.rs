use anyhow::Result;
use sui_graphql_client::Client;
use sui_sdk_types::{Address, Argument};
use sui_transaction_builder::{Serialized, TransactionBuilder};

use crate::objects;

pub fn pure<Pure: serde::Serialize>(
    builder: &mut TransactionBuilder,
    value: Pure,
) -> Result<Argument> {
    let value_arg = builder.input(Serialized(&value));
    Ok(value_arg)
}

pub async fn owned(
    client: &Client,
    builder: &mut TransactionBuilder,
    id: Address,
) -> Result<Argument> {
    let object_input = objects::get_as_input(client, id).await?;
    let object_arg = builder.input(object_input.with_owned_kind());
    Ok(object_arg)
}

pub async fn receiving(
    client: &Client,
    builder: &mut TransactionBuilder,
    id: Address,
) -> Result<Argument> {
    let object_input = objects::get_as_input(client, id).await?;
    let object_arg = builder.input(object_input.with_receiving_kind());
    Ok(object_arg)
}

pub async fn shared_ref(
    client: &Client,
    builder: &mut TransactionBuilder,
    id: Address,
) -> Result<Argument> {
    let object_input = objects::get_as_input(client, id).await?;
    let object_arg = builder.input(object_input.by_ref());
    Ok(object_arg)
}

pub async fn shared_mut(
    client: &Client,
    builder: &mut TransactionBuilder,
    id: Address,
) -> Result<Argument> {
    let object_input = objects::get_as_input(client, id).await?;
    let object_arg = builder.input(object_input.by_mut());
    Ok(object_arg)
}
