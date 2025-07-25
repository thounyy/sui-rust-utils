use sui_sdk_types::Address;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SuiUtilsError {
    #[error("GraphQL error: {0}")]
    GraphQL(String),
    #[error("Object not found: {0}")]
    ObjectNotFound(Address),
    #[error("Object contents not found: {0}")]
    ObjectContentsNotFound(String),
    #[error("No SUI coin with minimum budget found")]
    GasCoinNotFound,
    #[error("Error while building gas input")]
    InvalidGasInput,
    #[error("Could not get reference gas price")]
    ReferenceGasPriceError,
    #[error("Error while building transaction")]
    TransactionBuildingError(String),
    #[error("Error while signing transaction")]
    TransactionSigningError(String),
    #[error("Error while executing transaction")]
    TransactionExecutionError(String),
    #[error("Could not get transaction effects")]
    InvalidTransactionEffects,
}

impl From<sui_graphql_client::error::Error> for SuiUtilsError {
    fn from(err: sui_graphql_client::error::Error) -> Self {
        SuiUtilsError::GraphQL(err.to_string())
    }
}

impl From<Vec<cynic::GraphQlError>> for SuiUtilsError {
    fn from(errors: Vec<cynic::GraphQlError>) -> Self {
        SuiUtilsError::GraphQL(format!("{:?}", errors))
    }
}

pub type Result<T> = std::result::Result<T, SuiUtilsError>;