#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as BPass;

use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_runtime::{DispatchError, traits::SaturatedConversion};
use sp_std::vec;

const DEPOSIT_AMOUNT: u128 = 10_000_000_000_000_000_000;

/// Fund account with tokens, needed for org and campaign interactions
fn fund_account<T: Config>(account_id: &T::AccountId) -> Result<(), DispatchError> {
    let balance_amount: T::Balance = DEPOSIT_AMOUNT.saturated_into();
	<T as pallet::Config>::Currency::deposit(T::NativeTokenId::get(), &account_id, balance_amount)?;
	<T as pallet::Config>::Currency::deposit(T::ProtocolTokenId::get(), &account_id, balance_amount)?;
	Ok(())
}

/// Fund account with tokens, needed for org and battlepass interactions
fn get_funded_caller<T: Config>() -> Result<T::AccountId, DispatchError> {
    let caller: T::AccountId = whitelisted_caller();
    fund_account::<T>(&caller)?;

    Ok(caller)
}


fn get_org<T: Config>(caller: T::AccountId) -> T::Hash {
    let org_id = T::ControlBenchmarkHelper::create_org(caller.clone()).unwrap();
    let treasury_id = T::Control::org_treasury_account(&org_id).unwrap();
	fund_account::<T>(&treasury_id).unwrap();

    org_id
}

fn get_battlepass<T: Config>(caller: T::AccountId, org_id: T::Hash) -> T::Hash {
    let str = BoundedVec::truncate_from(vec![1,2]);
    let battlepass = types::Battlepass {
        creator: caller.clone(),
        org_id,
        name: str.clone(),
        cid: str.clone(),
        season: 1,
        price: 10,
        collection_id: 0
    };

    let _ = BPass::<T>::create_battlepass(RawOrigin::Signed(caller.clone()).into(), org_id, battlepass.cid.clone(), battlepass.name.clone(), battlepass.price.clone());

    <T as frame_system::Config>::Hashing::hash_of(&battlepass)
}

fn activate_bpass<T: Config>(caller: T::AccountId, battlepass_id: T::Hash) {
    let _ = BPass::<T>::activate_battlepass(RawOrigin::Signed(caller).into(), battlepass_id);
}

fn claim_bpass<T: Config>(caller: T::AccountId, battlepass_id: T::Hash) {
    let _ = BPass::<T>::claim_battlepass(RawOrigin::Signed(caller.clone()).into(), battlepass_id, caller);
}

fn set_bpass_points<T: Config>(caller: T::AccountId, battlepass_id: T::Hash) {
    let _ = BPass::<T>::set_points(RawOrigin::Signed(caller.clone()).into(), battlepass_id, caller, 10);
}

fn set_bpass_level<T: Config>(caller: T::AccountId, battlepass_id: T::Hash) {
    let _ = BPass::<T>::add_level(RawOrigin::Signed(caller.clone()).into(), battlepass_id, 1, 10);
}

fn get_reward<T: Config>(caller: T::AccountId, battlepass_id: T::Hash) -> T::Hash {
    let str = BoundedVec::truncate_from(vec![1,2]);
    let reward = Reward{
        battlepass_id,
        name: str.clone(),
        cid: str.clone(),
        level: 1,
        transferable: true,
        collection_id: 1
    };

    let _ = BPass::<T>::create_reward(RawOrigin::Signed(caller).into(), battlepass_id, reward.name.clone(), reward.cid.clone(), Some(10), reward.level.clone(), reward.transferable.clone());

    <T as frame_system::Config>::Hashing::hash_of(&reward)
}


benchmarks! {

	create_battlepass {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let str = BoundedVec::truncate_from(vec![1,2]);
    }: _(RawOrigin::Signed(caller), org_id, str.clone(), str.clone(), 10)

    claim_battlepass {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        activate_bpass::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller.clone()), battlepass_id, caller.clone())

    activate_battlepass {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
    }: _(RawOrigin::Signed(caller), battlepass_id)

    conclude_battlepass {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        activate_bpass::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller), battlepass_id)

    set_points {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        activate_bpass::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller.clone()), battlepass_id, caller.clone(), 10)

    create_reward {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        let str = BoundedVec::truncate_from(vec![1,2]);
        activate_bpass::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller), battlepass_id, str.clone(), str.clone(), Some(10), 1, false)

    disable_reward {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        let reward_id = get_reward::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller), reward_id)

    claim_reward {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        activate_bpass::<T>(caller.clone(), battlepass_id);
        claim_bpass::<T>(caller.clone(), battlepass_id);
        set_bpass_points::<T>(caller.clone(), battlepass_id);
        set_bpass_level::<T>(caller.clone(), battlepass_id);
        let reward_id = get_reward::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller), reward_id)

    add_level {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
    }: _(RawOrigin::Signed(caller.clone()), battlepass_id, 1, 10)

    remove_level {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
        set_bpass_level::<T>(caller.clone(), battlepass_id);
    }: _(RawOrigin::Signed(caller.clone()), battlepass_id, 1)

    add_bot {
        let caller: T::AccountId = get_funded_caller::<T>()?;
        let org_id = get_org::<T>(caller.clone());
        let battlepass_id = get_battlepass::<T>(caller.clone(), org_id);
    }: _(RawOrigin::Signed(caller.clone()), battlepass_id, caller.clone())

    impl_benchmark_test_suite!(BPass, crate::mock::new_test_ext(), crate::mock::Test);
}

