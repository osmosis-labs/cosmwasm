#![cfg(feature = "tokenfactory")]

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{Addr, Coin, Decimal};

#[non_exhaustive]
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum TokenFactoryQuery {
    /// Returns the denomination that can be bonded (if there are multiple native tokens on the chain)
    DenomAuthorityMetadata { denom: String },
    /// AllDelegations will return all delegations by the delegator
    DenomsFromCreator { creator: String },
}

/// DenomAuthorityMetadataResponse is data format returned from TokenFactoryRequest::DenomAuthorityMetadata query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DenomAuthorityMetadataResponse {
    pub authority_metadata: DenomAuthorityMetadata,
}

// DenomAuthorityMetadata struct
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DenomAuthorityMetadata {
    pub admin: String,
}

/// DelegationsResponse is data format returned from StakingRequest::AllDelegations query
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DenomsFromCreatorResponse {
    pub denoms: Vec<String>,
}
