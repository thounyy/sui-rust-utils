use cynic::QueryBuilder;
use sui_graphql_client::{
    query_types::{MoveValue, ObjectFilter, ObjectsQuery, ObjectsQueryArgs},
    Client, Direction, DynamicFieldOutput, PaginationFilter,
};
use sui_sdk_types::{framework::Coin, Address, Object};
use sui_transaction_builder::unresolved::Input;

use crate::error::{Result, SuiUtilsError};

pub async fn get(client: &Client, id: Address) -> Result<Object> {
    client
        .object(id, None)
        .await?
        .ok_or(SuiUtilsError::ObjectNotFound(id))
}

pub async fn get_as_input(client: &Client, id: Address) -> Result<Input> {
    let object = get(client, id).await?;
    let input = Input::from(&object);

    Ok(input)
}

pub async fn get_multi(client: &Client, mut ids: Vec<Address>) -> Result<Vec<Object>> {
    let mut objects = Vec::new();
    let mut cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        let filter = PaginationFilter {
            direction: Direction::Forward,
            cursor: cursor.clone(),
            limit: Some(50),
        };

        let mut object_ids = Some(ids.clone());
        if ids.len() > 50 {
            object_ids = Some(ids.split_off(50));
        }

        let resp = client
            .objects(
                Some(ObjectFilter {
                    object_ids,
                    ..Default::default()
                }),
                filter,
            )
            .await?;
        objects.extend(resp.data().iter().cloned());

        cursor = resp.page_info().end_cursor.clone();
        has_next_page = resp.page_info().has_next_page;
    }

    Ok(objects)
}

pub async fn get_owned(
    client: &Client,
    owner: Address,
    type_: Option<&str>,
) -> Result<Vec<Object>> {
    let mut objects = Vec::new();
    let mut cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        let filter = PaginationFilter {
            direction: Direction::Forward,
            cursor: cursor.clone(),
            limit: Some(50),
        };

        let resp = client
            .objects(
                Some(ObjectFilter {
                    owner: Some(owner),
                    type_,
                    object_ids: None,
                }),
                filter,
            )
            .await?;
        objects.extend(resp.data().iter().cloned());

        cursor = resp.page_info().end_cursor.clone();
        has_next_page = resp.page_info().has_next_page;
    }

    Ok(objects)
}

pub async fn get_owned_coins(
    client: &Client,
    owner: Address,
    type_: Option<&str>,
) -> Result<Vec<Coin<'static>>> {
    let mut coins = Vec::new();
    let mut cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        let filter = PaginationFilter {
            direction: Direction::Forward,
            cursor: cursor.clone(),
            limit: Some(50),
        };

        let resp = client.coins(owner, type_, filter).await?;
        coins.extend(resp.data().iter().cloned());

        cursor = resp.page_info().end_cursor.clone();
        has_next_page = resp.page_info().has_next_page;
    }

    Ok(coins)
}

// gets `MoveValue`s from sui-graphql-client (returning the fields in json)
pub async fn get_owned_with_fields(
    client: &Client,
    owner: Address,
    type_: Option<&str>,
) -> Result<Vec<MoveValue>> {
    let mut move_values = Vec::new();
    let mut cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        let operation = ObjectsQuery::build(ObjectsQueryArgs {
            after: cursor.as_deref(),
            before: None,
            filter: Some(ObjectFilter {
                owner: Some(owner),
                type_,
                ..Default::default()
            }),
            first: Some(50),
            last: None,
        });

        let response = client.run_query(&operation)
            .await
            .map_err(|e| SuiUtilsError::GraphQL(e.to_string()))?;

        if let Some(objects) = response.data {
            for object in objects.objects.nodes {
                let object_string = format!("{:?}", object);
                let move_value = object
                    .as_move_object
                    .and_then(|move_object| move_object.contents)
                    .ok_or(SuiUtilsError::ObjectContentsNotFound(object_string))?;
                move_values.push(move_value);
            }

            cursor = objects.objects.page_info.end_cursor;
            has_next_page = objects.objects.page_info.has_next_page;
        }
    }

    Ok(move_values)
}

pub async fn get_dynamic_fields(client: &Client, id: Address) -> Result<Vec<DynamicFieldOutput>> {
    let mut dfs = Vec::new();
    let mut cursor = None;
    let mut has_next_page = true;

    while has_next_page {
        let filter = PaginationFilter {
            cursor: cursor.clone(),
            ..PaginationFilter::default()
        };

        let resp = client.dynamic_fields(id, filter).await?;
        dfs.extend(resp.data().iter().cloned());

        cursor = resp.page_info().end_cursor.clone();
        has_next_page = resp.page_info().has_next_page;
    }

    Ok(dfs)
}
