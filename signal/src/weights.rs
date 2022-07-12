// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for gamedao_signal
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-07-11, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// ./target/release/subzero
// benchmark
// pallet
// --pallet=gamedao_signal
// --extrinsic=*
// --steps=20
// --repeat=10
// --output=gamedao-protocol/signal/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for gamedao_signal.
pub trait WeightInfo {
	fn general_proposal() -> Weight;
	fn membership_proposal() -> Weight;
	fn withdraw_proposal() -> Weight;
	fn simple_vote_general(b: u32, ) -> Weight;
	fn simple_vote_withdraw(b: u32, ) -> Weight;
	fn unlock_balance(b: u32, ) -> Weight;
}

/// Weights for gamedao_signal using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Control OrgState (r:1 w:0)
	// Storage: Control OrgMemberState (r:1 w:0)
	// Storage: Signal ProposalTimeLimit (r:1 w:0)
	// Storage: Signal ProposalsByBlock (r:1 w:1)
	// Storage: Signal Nonce (r:1 w:1)
	// Storage: Signal Proposals (r:1 w:1)
	// Storage: Signal ProposalsCount (r:1 w:1)
	// Storage: Signal ProposalsByOrgCount (r:1 w:1)
	// Storage: Signal ProposalsByOwnerCount (r:1 w:1)
	// Storage: Signal ProposalsByOrg (r:1 w:1)
	// Storage: Signal ProposalsArray (r:0 w:1)
	// Storage: Signal ProposalsByOrgArray (r:0 w:1)
	// Storage: Signal ProposalsIndex (r:0 w:1)
	// Storage: Signal Owners (r:0 w:1)
	// Storage: Signal ProposalsByOwnerIndex (r:0 w:1)
	// Storage: Signal Metadata (r:0 w:1)
	// Storage: Signal ProposalsByOwnerArray (r:0 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	// Storage: Signal ProposalsByOrgIndex (r:0 w:1)
	fn general_proposal() -> Weight {
		(76_689_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	fn membership_proposal() -> Weight {
		(6_878_000 as Weight)
	}
	// Storage: Flow CampaignState (r:1 w:0)
	// Storage: Flow CampaignOwner (r:1 w:0)
	// Storage: Signal ProposalTimeLimit (r:1 w:0)
	// Storage: Signal CampaignBalanceUsed (r:1 w:0)
	// Storage: Flow CampaignBalance (r:1 w:0)
	// Storage: Signal ProposalsByBlock (r:1 w:1)
	// Storage: Signal Nonce (r:1 w:1)
	// Storage: Signal Proposals (r:1 w:1)
	// Storage: Flow CampaignOrg (r:1 w:0)
	// Storage: Signal ProposalsCount (r:1 w:1)
	// Storage: Signal ProposalsByOrgCount (r:1 w:1)
	// Storage: Signal ProposalsByOwnerCount (r:1 w:1)
	// Storage: Signal ProposalsByOrg (r:1 w:1)
	// Storage: Signal ProposalsArray (r:0 w:1)
	// Storage: Signal ProposalsByOrgArray (r:0 w:1)
	// Storage: Signal ProposalsIndex (r:0 w:1)
	// Storage: Signal Owners (r:0 w:1)
	// Storage: Signal ProposalsByOwnerIndex (r:0 w:1)
	// Storage: Signal Metadata (r:0 w:1)
	// Storage: Signal ProposalsByOwnerArray (r:0 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	// Storage: Signal ProposalsByOrgIndex (r:0 w:1)
	fn withdraw_proposal() -> Weight {
		(81_474_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(13 as Weight))
			.saturating_add(T::DbWeight::get().writes(16 as Weight))
	}
	// Storage: Signal Proposals (r:1 w:0)
	// Storage: Signal ProposalStates (r:1 w:0)
	// Storage: Signal ProposalVotes (r:1 w:1)
	// Storage: Signal VotedBefore (r:1 w:1)
	// Storage: Signal ProposalSimpleVotes (r:1 w:1)
	// Storage: Signal ProposalApprovers (r:1 w:1)
	// Storage: Signal ProposalsByVoterCount (r:1 w:1)
	// Storage: Signal ProposalVotesByVoters (r:1 w:1)
	// Storage: Signal ProposalsByVoter (r:1 w:1)
	// Storage: Signal ProposalVoters (r:1 w:1)
	fn simple_vote_general(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((196_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Signal Proposals (r:1 w:0)
	// Storage: Signal ProposalStates (r:1 w:0)
	// Storage: Signal ProposalVotes (r:1 w:1)
	// Storage: Signal VotedBefore (r:1 w:1)
	// Storage: Signal ProposalSimpleVotes (r:1 w:1)
	// Storage: Signal ProposalApprovers (r:1 w:1)
	// Storage: Flow CampaignContributorsCount (r:1 w:0)
	// Storage: Signal ProposalsByVoterCount (r:1 w:1)
	// Storage: Signal ProposalVotesByVoters (r:1 w:1)
	// Storage: Signal ProposalsByVoter (r:1 w:1)
	// Storage: Signal ProposalVoters (r:1 w:1)
	fn simple_vote_withdraw(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((345_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(11 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	// Storage: Signal Metadata (r:1 w:0)
	// Storage: Flow CampaignBalance (r:1 w:0)
	// Storage: Signal CampaignBalanceUsed (r:1 w:1)
	// Storage: Signal Owners (r:1 w:0)
	// Storage: Flow CampaignOrg (r:1 w:0)
	// Storage: Control OrgTreasury (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	fn unlock_balance(b: u32, ) -> Weight {
		(79_958_000 as Weight)
			// Standard Error: 55_000
			.saturating_add((344_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Control OrgState (r:1 w:0)
	// Storage: Control OrgMemberState (r:1 w:0)
	// Storage: Signal ProposalTimeLimit (r:1 w:0)
	// Storage: Signal ProposalsByBlock (r:1 w:1)
	// Storage: Signal Nonce (r:1 w:1)
	// Storage: Signal Proposals (r:1 w:1)
	// Storage: Signal ProposalsCount (r:1 w:1)
	// Storage: Signal ProposalsByOrgCount (r:1 w:1)
	// Storage: Signal ProposalsByOwnerCount (r:1 w:1)
	// Storage: Signal ProposalsByOrg (r:1 w:1)
	// Storage: Signal ProposalsArray (r:0 w:1)
	// Storage: Signal ProposalsByOrgArray (r:0 w:1)
	// Storage: Signal ProposalsIndex (r:0 w:1)
	// Storage: Signal Owners (r:0 w:1)
	// Storage: Signal ProposalsByOwnerIndex (r:0 w:1)
	// Storage: Signal Metadata (r:0 w:1)
	// Storage: Signal ProposalsByOwnerArray (r:0 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	// Storage: Signal ProposalsByOrgIndex (r:0 w:1)
	fn general_proposal() -> Weight {
		(76_689_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(16 as Weight))
	}
	fn membership_proposal() -> Weight {
		(6_878_000 as Weight)
	}
	// Storage: Flow CampaignState (r:1 w:0)
	// Storage: Flow CampaignOwner (r:1 w:0)
	// Storage: Signal ProposalTimeLimit (r:1 w:0)
	// Storage: Signal CampaignBalanceUsed (r:1 w:0)
	// Storage: Flow CampaignBalance (r:1 w:0)
	// Storage: Signal ProposalsByBlock (r:1 w:1)
	// Storage: Signal Nonce (r:1 w:1)
	// Storage: Signal Proposals (r:1 w:1)
	// Storage: Flow CampaignOrg (r:1 w:0)
	// Storage: Signal ProposalsCount (r:1 w:1)
	// Storage: Signal ProposalsByOrgCount (r:1 w:1)
	// Storage: Signal ProposalsByOwnerCount (r:1 w:1)
	// Storage: Signal ProposalsByOrg (r:1 w:1)
	// Storage: Signal ProposalsArray (r:0 w:1)
	// Storage: Signal ProposalsByOrgArray (r:0 w:1)
	// Storage: Signal ProposalsIndex (r:0 w:1)
	// Storage: Signal Owners (r:0 w:1)
	// Storage: Signal ProposalsByOwnerIndex (r:0 w:1)
	// Storage: Signal Metadata (r:0 w:1)
	// Storage: Signal ProposalsByOwnerArray (r:0 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	// Storage: Signal ProposalsByOrgIndex (r:0 w:1)
	fn withdraw_proposal() -> Weight {
		(81_474_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(13 as Weight))
			.saturating_add(RocksDbWeight::get().writes(16 as Weight))
	}
	// Storage: Signal Proposals (r:1 w:0)
	// Storage: Signal ProposalStates (r:1 w:0)
	// Storage: Signal ProposalVotes (r:1 w:1)
	// Storage: Signal VotedBefore (r:1 w:1)
	// Storage: Signal ProposalSimpleVotes (r:1 w:1)
	// Storage: Signal ProposalApprovers (r:1 w:1)
	// Storage: Signal ProposalsByVoterCount (r:1 w:1)
	// Storage: Signal ProposalVotesByVoters (r:1 w:1)
	// Storage: Signal ProposalsByVoter (r:1 w:1)
	// Storage: Signal ProposalVoters (r:1 w:1)
	fn simple_vote_general(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((196_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: Signal Proposals (r:1 w:0)
	// Storage: Signal ProposalStates (r:1 w:0)
	// Storage: Signal ProposalVotes (r:1 w:1)
	// Storage: Signal VotedBefore (r:1 w:1)
	// Storage: Signal ProposalSimpleVotes (r:1 w:1)
	// Storage: Signal ProposalApprovers (r:1 w:1)
	// Storage: Flow CampaignContributorsCount (r:1 w:0)
	// Storage: Signal ProposalsByVoterCount (r:1 w:1)
	// Storage: Signal ProposalVotesByVoters (r:1 w:1)
	// Storage: Signal ProposalsByVoter (r:1 w:1)
	// Storage: Signal ProposalVoters (r:1 w:1)
	fn simple_vote_withdraw(b: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 1_000
			.saturating_add((345_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(11 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	// Storage: Signal Metadata (r:1 w:0)
	// Storage: Flow CampaignBalance (r:1 w:0)
	// Storage: Signal CampaignBalanceUsed (r:1 w:1)
	// Storage: Signal Owners (r:1 w:0)
	// Storage: Flow CampaignOrg (r:1 w:0)
	// Storage: Control OrgTreasury (r:1 w:0)
	// Storage: Tokens Accounts (r:1 w:1)
	// Storage: Signal ProposalStates (r:0 w:1)
	fn unlock_balance(b: u32, ) -> Weight {
		(79_958_000 as Weight)
			// Standard Error: 55_000
			.saturating_add((344_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
}
