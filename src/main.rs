use borsh::{BorshDeserialize, BorshSerialize};
use near_primitives::merkle::MerklePath;
use near_primitives::transaction::{ExecutionOutcomeWithIdAndProof, ExecutionStatus, LogEntry};
use near_primitives_core::hash::CryptoHash;
use near_primitives_core::types::{AccountId, Balance, Gas};
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Clone, Default, Eq, Debug)]
pub struct OldExecutionOutcome {
    pub logs: Vec<LogEntry>,
    pub receipt_ids: Vec<CryptoHash>,
    pub gas_burnt: Gas,
    pub tokens_burnt: Balance,
    pub executor_id: AccountId,
    pub status: ExecutionStatus,
}

#[derive(PartialEq, Clone, Default, Debug, BorshSerialize, BorshDeserialize, Eq)]
pub struct OldExecutionOutcomeWithId {
    pub id: CryptoHash,
    pub outcome: OldExecutionOutcome,
}

#[derive(PartialEq, Clone, Default, Debug, BorshSerialize, BorshDeserialize, Eq)]
pub struct OldExecutionOutcomeWithIdAndProof {
    pub proof: MerklePath,
    pub block_hash: CryptoHash,
    pub outcome_with_id: OldExecutionOutcomeWithId,
}

fn main() {
    let value = std::fs::read("h6QEnvsiX5HcgEqKt5PNLKkm4ztZHM9BxYFmEhw6pxf").unwrap();
    let old_outcomes = Vec::<OldExecutionOutcomeWithIdAndProof>::try_from_slice(&value)
        .expect("BorshDeserialize should not fail");
}
