use prost_types::Struct;
use serde::{Deserialize, Serialize};
use namada_sdk::types::{address::Address, dec::Dec, token::Amount};
use crate::model::shared::{SuffixedDur, PaginationInfo, PaginationQueryParams};


#[derive(Deserialize)]
pub struct ValidatorsQueryParams {
  pub status: Option<CosmosValStatus>,
  pub pagination: Option<PaginationQueryParams>,
}

#[derive(Serialize)]
pub struct PoolResponse {
  pub pool: PoolInfo,
}

#[derive(Serialize)]
pub struct PoolInfo {
  pub not_bonded_tokens: String,
  pub bonded_tokens: Amount,
}

#[derive(Serialize)]
pub struct ParamsResponse {
  pub unbonding_time: SuffixedDur,
  pub max_validators: u32,
  pub max_entries: u32,
  pub historical_entries: u32,
  pub bond_denom: String,
}

#[derive(Serialize)]
pub struct ValidatorsResponse {
  pub validators: Vec<ValidatorInfo>,
  pub pagination: PaginationInfo,
}

impl ValidatorsResponse {
  pub fn new() -> Self {
    ValidatorsResponse {
      validators: Vec::new(),
      pagination: PaginationInfo::default(),
    }
  }
}

#[derive(Serialize)]
pub struct ValidatorResponse {
  pub validator: ValidatorInfo,
}
impl Default for ValidatorInfo {
  fn default() -> Self {
    ValidatorInfo {
      operator_address: None,
      consensus_pubkey: None,
      jailed: None,
      status: None,
      tokens: None,
      delegator_shares:None,
      description: None,
      unbonding_height: None,
      unbonding_time: None,
      commission: None,
      min_self_delegation: None
    }
  }
}

impl ValidatorResponse {
  pub fn new() -> Self {
    ValidatorResponse {
      validator: ValidatorInfo::default(),
    }
  }
}

#[derive(Serialize)]
pub struct ValidatorInfo {
  pub operator_address: Option<Address>,
  pub consensus_pubkey: Option<ConsensusKeyInfo>,
  pub jailed: Option<bool>,
  pub status: Option<CosmosValStatus>,
  pub tokens: Option<Amount>,
  pub delegator_shares: Option<String>,
  pub description: Option<ValidatorDescription>,
  pub unbonding_height: Option<String>,
  pub unbonding_time: Option<String>,
  pub commission: Option<CommissionInfo>,
  pub min_self_delegation: Option<String>,
}


#[derive(Serialize)]
pub struct ConsensusKeyInfo {
  #[serde(rename = "@type")]
  pub at_type: String,
  pub key: String,
}

#[derive(Serialize)]
pub struct ValidatorDescription {
  pub moniker: Address,
  pub identity: Option<String>,
  pub website: Option<String>,
  pub security_contact: Option<String>,
  pub details: Option<String>,
}

impl ValidatorDescription {
  pub fn empty(address: &Address) -> Self {
    ValidatorDescription {
      moniker: address.clone(),
      identity: Some("".to_string()),
      website: Some("".to_string()),
      security_contact: Some("".to_string()),
      details: Some("".to_string()),
    }
  }
}

#[derive(Serialize)]
pub struct CommissionInfo {
  pub commission_rates: RatesInfo,
  pub update_time: String, // time
}

impl Default for CommissionInfo {
  fn default() -> Self {
    CommissionInfo {
      commission_rates: RatesInfo {
        rate: Dec::one(),
        max_rate: Dec::one(),
        max_change_rate: Dec::one(),
      },
      update_time: "".to_string(),
    }
  }
}

#[derive(Serialize)]
pub struct RatesInfo {
  pub rate: Dec,
  pub max_rate: Dec,
  pub max_change_rate: Dec,
}

#[allow(non_camel_case_types)]
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CosmosValStatus {
  BOND_STATUS_UNSPECIFIED,
  BOND_STATUS_BONDED,
  BOND_STATUS_UNBONDING,
  BOND_STATUS_UNBONDED,
}

impl PartialEq for CosmosValStatus {
  fn eq(&self, other: &Self) -> bool {
    match  (self, other) {
      (CosmosValStatus::BOND_STATUS_UNSPECIFIED, CosmosValStatus::BOND_STATUS_UNSPECIFIED) => true,
      (CosmosValStatus::BOND_STATUS_BONDED, CosmosValStatus::BOND_STATUS_BONDED) => true,
      (CosmosValStatus::BOND_STATUS_UNBONDING, CosmosValStatus::BOND_STATUS_UNBONDING) => true,
      (CosmosValStatus::BOND_STATUS_UNBONDED, CosmosValStatus::BOND_STATUS_UNBONDED) => true,
      _ => false,
    }
  }
}
