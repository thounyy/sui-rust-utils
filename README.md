# Sui Rust Utils

![sui rust banner](./assets/rust_on_sui_banner.png)

A Rust library providing helpful utilities and abstractions that complement the new official [Sui Rust SDK](https://github.com/mystenlabs/sui-rust-sdk). This library simplifies common operations when working with Sui objects and composing transactions with arguments.

It uses the same version as in [move-binding](https://github.com/MystenLabs/move-binding) so you can use the three libraries together seamlessly.

## Features

- **Object Management**: Easy retrieval and manipulation of Sui objects
- **Transaction Building**: Simplified transaction creation with automatic gas handling
- **Argument Construction**: Helper functions for creating different types of transaction arguments
- **Pagination Support**: Built-in pagination for large result sets
- **Error Handling**: Comprehensive error handling with `anyhow`

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
sui-utils = { git = "https://github.com/thounyy/sui-rust-utils" }
```

## Usage

### Objects Module

The `objects` module provides utilities for working with Sui objects:

```rust
use sui_utils::objects;
use sui_graphql_client::Client;
use sui_sdk_types::Address;

// Get a single object by ID
let object = objects::get(&client, object_id).await?;

// Get multiple objects by IDs
let objects = objects::get_multi(&client, vec![id1, id2, id3]).await?;

// Get all objects owned by an address
let owned_objects = objects::get_owned(&client, owner_address, None).await?;

// Get objects of a specific type owned by an address
let coins = objects::get_owned(&client, owner_address, Some("0x2::coin::Coin<0x2::sui::SUI>")).await?;

// Get coins owned by an address
let sui_coins = objects::get_owned_coins(&client, owner_address, Some("0x2::coin::Coin<0x2::sui::SUI>")).await?;

// Get objects with their Move fields (returns MoveValue)
let objects_with_fields = objects::get_owned_with_fields(&client, owner_address, Some("0x2::coin::Coin<0x2::sui::SUI>")).await?;

// Get dynamic fields of an object
let dynamic_fields = objects::get_dynamic_fields(&client, object_id).await?;
```

### Transaction Builder Module

The `transaction_builder` module simplifies transaction creation:

```rust
use sui_utils::transaction_builder;
use sui_crypto::ed25519::Ed25519PrivateKey;

// Create a new transaction builder with automatic gas handling
let mut builder = transaction_builder::new_with_gas(&client, caller_address, gas_budget).await?;

// Execute transaction and wait for effects
let effects = transaction_builder::execute_and_wait_for_effects(&client, builder, &private_key).await?;
```

### Argument Module

The `argument` module provides helper functions for creating arguments to pass in PTB commands:

```rust
use sui_utils::argument;
use sui_sdk_types::Address;

// Create a pure argument (for primitive values)
let pure_arg = argument::pure(&mut builder, 42u64)?;

// Create an owned object argument
let owned_arg = argument::owned(&client, &mut builder, object_id).await?;

// Create a receiving object argument
let receiving_arg = argument::receiving(&client, &mut builder, object_id).await?;

// Create a shared immutable reference argument
let shared_ref_arg = argument::shared_ref(&client, &mut builder, object_id).await?;

// Create a shared mutable reference argument
let shared_mut_arg = argument::shared_mut(&client, &mut builder, object_id).await?;
```

## Example

```rust
use sui_utils::{objects, transaction_builder, argument};
use sui_graphql_client::Client;
use sui_crypto::ed25519::Ed25519PrivateKey;

async fn example(private_key: &Ed25519PrivateKey) -> Result<()> {
    let client = Client::new_mainnet();
    let pk = Ed25519PrivateKey::generate(rand::thread_rng());
    let address = pk.public_key().derive_address();
    
    // Create transaction builder
    let mut builder = transaction_builder::new_with_gas(&client, address, 100000000).await?;
    
    // Get SUI coins owned by caller
    let sui_coins = objects::get_owned_coins(&client, address, Some("0x2::coin::Coin<0x2::sui::SUI>")).await?;
    let coin_to_transfer = &sui_coins[0].id();
    
    // Construct arguments
    let key = argument::pure(&mut builder, key);
    let coin = argument::owned(&client, &mut builder, coin_to_transfer);
    let config = argument::shared_ref(&client, &mut builder, "0xCAFE".parse()?)

    // Add move call to ptb
    builder.move_call(
        sui_transaction_builder::Function::new(
            "package_id".parse()?,
            "module_name".parse()?,
            "function_name".parse()?,
            vec!["coin_type".parse()?],
        ),
        vec![config, coin, key],
    );
    
    // Execute transaction
    let effects = transaction_builder::execute_and_wait_for_effects(&client, builder, &pk).await?;
    println!("Transaction executed: {:?}", effects);
    
    Ok(())
}
```

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.