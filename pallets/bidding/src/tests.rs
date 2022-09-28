use super::*;
use crate::{mock::*};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

#[test]
fn convert_u64_to_balance_option_should_succeed() {
	new_test_ext().execute_with(|| {
		let amount: u64 = 100;
		let converted_amount: crate::Housing_Fund::BalanceOf<Test> = 100;

		assert_eq!(
			type_of(BiddingModule::u64_to_balance_option(amount)),
			type_of(Some(converted_amount))
		);

		assert_eq!(
			BiddingModule::u64_to_balance_option(amount),
			Some(converted_amount)
		);
	});
}

#[test]
fn convert_balance_should_succeed() {
	new_test_ext().execute_with(|| {
		let amount: crate::Onboarding::BalanceOf<Test> = 100;
		let converted_amount: crate::Housing_Fund::BalanceOf<Test> = 100;

		assert_eq!(
			type_of(BiddingModule::convert_balance(amount)),
			type_of(Some(converted_amount))
		);

		assert_eq!(
			BiddingModule::convert_balance(amount),
			Some(converted_amount)
		);
	});
}

#[test]
fn get_amount_percentage_should_succeed() {
	new_test_ext().execute_with(|| {
		let amount: crate::Housing_Fund::BalanceOf<Test> = 1000;

		assert_eq!(
			BiddingModule::get_amount_percentage(amount, 20),
			200
		);

		assert_eq!(
			BiddingModule::get_amount_percentage(amount, 36),
			360
		);
	});
}


#[test]
fn get_investor_share_should_succeed() {
	new_test_ext().execute_with(|| {
		let amount: crate::Housing_Fund::BalanceOf<Test> = 100;

		let mut contribution: crate::Housing_Fund::Contribution<Test> = crate::Housing_Fund::Contribution {
			account_id: 1,
			available_balance: HousingFund::u64_to_balance_option(25).unwrap(),
			reserved_balance: HousingFund::u64_to_balance_option(0).unwrap(),
			contributed_balance: HousingFund::u64_to_balance_option(0).unwrap(),
			has_withdrawn: false,
			block_number: 1,
			contributions: vec![crate::Housing_Fund::ContributionLog {
				amount: HousingFund::u64_to_balance_option(25).unwrap(),
				block_number: 1
			}],
			withdraws: Vec::new()
		};

		assert_eq!(
			BiddingModule::get_investor_share(amount.clone(), contribution.clone()),
			20
		);

		contribution.reserve_amount(10);

		assert_eq!(
			BiddingModule::get_investor_share(amount, contribution),
			15
		);
	});
}

#[test]
fn get_oldest_contribution_should_succeed() {
	new_test_ext().execute_with(|| {
		let ordered_list = Vec::new();

		let contribution: crate::Housing_Fund::Contribution<Test> = crate::Housing_Fund::Contribution {
			account_id: 1,
			available_balance: HousingFund::u64_to_balance_option(25).unwrap(),
			reserved_balance: HousingFund::u64_to_balance_option(0).unwrap(),
			contributed_balance: HousingFund::u64_to_balance_option(0).unwrap(),
			has_withdrawn: false,
			block_number: 1,
			contributions: vec![crate::Housing_Fund::ContributionLog {
				amount: HousingFund::u64_to_balance_option(25).unwrap(),
				block_number: 1
			}],
			withdraws: Vec::new()
		};

		let contributions = vec![
			(1, contribution.clone()),
			(2, crate::Housing_Fund::Contribution {
				account_id: 1,
				available_balance: HousingFund::u64_to_balance_option(30).unwrap(),
				reserved_balance: HousingFund::u64_to_balance_option(0).unwrap(),
				contributed_balance: HousingFund::u64_to_balance_option(0).unwrap(),
				has_withdrawn: false,
				block_number: 2,
				contributions: vec![crate::Housing_Fund::ContributionLog {
					amount: HousingFund::u64_to_balance_option(25).unwrap(),
					block_number: 1
				}],
				withdraws: Vec::new()
			})
		];

		assert_eq!(
			BiddingModule::get_oldest_contribution(ordered_list, contributions),
			(1, contribution)
		);
	});
}

#[test]
fn create_investor_list_should_succeed() {
	new_test_ext().execute_with(|| {
		
		let mut block_number = System::block_number();
		let mut amount = 20;

		for account_id in 1..7 {
			assert_ok!(RoleModule::set_role(
				Origin::signed(account_id.clone()),
				account_id,
				crate::Onboarding::HousingFund::ROLES::Accounts::INVESTOR
			));

			if account_id > 4 {
				amount = 10;
			}
			// test contribute with sufficient contribution and free balance
			assert_ok!(HousingFund::contribute_to_fund(Origin::signed(account_id), amount));

			let contribution = HousingFund::contributions(account_id).unwrap();

			assert_eq!(contribution.block_number, block_number);

			block_number = block_number.saturating_add(1);
			System::set_block_number(block_number.clone());
		}

		let investor_list = BiddingModule::create_investor_list(100);

		assert_eq!(investor_list.contains(&(1, 20)), true);
		assert_eq!(investor_list.contains(&(2, 20)), true);
		assert_eq!(investor_list.contains(&(3, 20)), true);
		assert_eq!(investor_list.contains(&(4, 20)), true);
		assert_eq!(investor_list.contains(&(5, 10)), true);
		assert_eq!(investor_list.contains(&(6, 10)), true);
	});
}

#[test]
fn create_investor_list_should_fail() {
	new_test_ext().execute_with(|| {
		
		let mut block_number = System::block_number();
		let mut amount = 20;

		for account_id in 1..7 {
			assert_ok!(RoleModule::set_role(
				Origin::signed(account_id.clone()),
				account_id,
				crate::Onboarding::HousingFund::ROLES::Accounts::INVESTOR
			));

			if account_id > 2 {
				if account_id == 6 {
					amount = 30;
				}
				else {
					amount = 10;
				}			
			}
			
			// test contribute with sufficient contribution and free balance
			assert_ok!(HousingFund::contribute_to_fund(Origin::signed(account_id), amount));

			let contribution = HousingFund::contributions(account_id).unwrap();

			assert_eq!(contribution.block_number, block_number);

			block_number = block_number.saturating_add(1);
			System::set_block_number(block_number.clone());
		}

		let investor_list = BiddingModule::create_investor_list(100);

		assert_eq!(investor_list.len(), 0);
	});
}

#[test]
fn process_asset_not_enough_fund_should_fail() {
	new_test_ext().execute_with(|| {
		
		let mut block_number = System::block_number();
		let amount = 20;

		for account_id in 1..6 {
			assert_ok!(RoleModule::set_role(
				Origin::signed(account_id.clone()),
				account_id,
				crate::Onboarding::HousingFund::ROLES::Accounts::INVESTOR
			));
			
			// test contribute with sufficient contribution and free balance
			assert_ok!(HousingFund::contribute_to_fund(Origin::signed(account_id), amount));

			let contribution = HousingFund::contributions(account_id).unwrap();

			assert_eq!(contribution.block_number, block_number);

			block_number = block_number.saturating_add(1);
			System::set_block_number(block_number.clone());
		}

		assert_ok!(RoleModule::set_role(Origin::signed(KEZIA).clone(), KEZIA, crate::Onboarding::HousingFund::ROLES::Accounts::SERVICER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), KEZIA));
		assert_ok!(RoleModule::set_role(Origin::signed(AMANI).clone(), AMANI, crate::Onboarding::HousingFund::ROLES::Accounts::SELLER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), AMANI));

		let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> = b"metadata0".to_vec().try_into().unwrap();

		assert_ok!(NftModule::create_collection(
			Origin::signed(KEZIA),
			NftCollection::OFFICESTEST,
			metadata.clone()
		));

		assert_ok!(OnboardingModule::create_and_submit_proposal(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,
			Some(100),
			metadata,
			false
		));

		let collection_id = NftCollection::OFFICESTEST.value();
		let item_id = pallet_nft::ItemsCount::<Test>::get()[collection_id as usize] - 1;

		assert_ok!(OnboardingModule::change_status(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,item_id.clone(),
			crate::Onboarding::AssetStatus::ONBOARDED)
		);

		assert_ok!(BiddingModule::process_asset());

		let event = <frame_system::Pallet<Test>>::events()
			.pop()
			.expect("Expected at least one EventRecord to be found")
			.event;

		// check that the event has been raised
		assert_eq!(
			event,
			mock::Event::BiddingModule(crate::Event::HousingFundNotEnough(collection_id, item_id, 100, block_number))
		);
	});
}

#[test]
fn process_asset_cannot_assemble_investor_should_fail() {
	new_test_ext().execute_with(|| {
		
		let mut block_number = System::block_number();
		let amount = 100;

		for account_id in 1..6 {
			assert_ok!(RoleModule::set_role(
				Origin::signed(account_id.clone()),
				account_id,
				crate::Onboarding::HousingFund::ROLES::Accounts::INVESTOR
			));
			
			// test contribute with sufficient contribution and free balance
			assert_ok!(HousingFund::contribute_to_fund(Origin::signed(account_id), amount));

			let contribution = HousingFund::contributions(account_id).unwrap();

			assert_eq!(contribution.block_number, block_number);

			block_number = block_number.saturating_add(1);
			System::set_block_number(block_number.clone());
		}

		assert_ok!(HousingFund::withdraw_fund(Origin::signed(EVE), 90));

		assert_ok!(RoleModule::set_role(Origin::signed(KEZIA).clone(), KEZIA, crate::Onboarding::HousingFund::ROLES::Accounts::SERVICER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), KEZIA));
		assert_ok!(RoleModule::set_role(Origin::signed(AMANI).clone(), AMANI, crate::Onboarding::HousingFund::ROLES::Accounts::SELLER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), AMANI));

		let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> = b"metadata0".to_vec().try_into().unwrap();

		assert_ok!(NftModule::create_collection(
			Origin::signed(KEZIA),
			NftCollection::OFFICESTEST,
			metadata.clone()
		));

		assert_ok!(OnboardingModule::create_and_submit_proposal(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,
			Some(100),
			metadata,
			false
		));

		let collection_id = NftCollection::OFFICESTEST.value();
		let item_id = pallet_nft::ItemsCount::<Test>::get()[collection_id as usize] - 1;

		assert_ok!(OnboardingModule::change_status(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,item_id.clone(),
			crate::Onboarding::AssetStatus::ONBOARDED)
		);

		assert_ok!(BiddingModule::process_asset());

		let event = <frame_system::Pallet<Test>>::events()
			.pop()
			.expect("Expected at least one EventRecord to be found")
			.event;

		// check that the event has been raised
		assert_eq!(
			event,
			mock::Event::BiddingModule(crate::Event::FailedToAssembleInvestor(collection_id, item_id, 100, block_number))
		);
	});
}

#[test]
fn process_asset_should_succeed() {
	new_test_ext().execute_with(|| {
		
		let mut block_number = System::block_number();
		let amount = 100;

		for account_id in 1..6 {
			assert_ok!(RoleModule::set_role(
				Origin::signed(account_id.clone()),
				account_id,
				crate::Onboarding::HousingFund::ROLES::Accounts::INVESTOR
			));
			
			// test contribute with sufficient contribution and free balance
			assert_ok!(HousingFund::contribute_to_fund(Origin::signed(account_id), amount));

			let contribution = HousingFund::contributions(account_id).unwrap();

			assert_eq!(contribution.block_number, block_number);

			block_number = block_number.saturating_add(1);
			System::set_block_number(block_number.clone());
		}

		assert_ok!(RoleModule::set_role(Origin::signed(KEZIA).clone(), KEZIA, crate::Onboarding::HousingFund::ROLES::Accounts::SERVICER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), KEZIA));
		assert_ok!(RoleModule::set_role(Origin::signed(AMANI).clone(), AMANI, crate::Onboarding::HousingFund::ROLES::Accounts::SELLER));
		assert_ok!(RoleModule::account_approval(Origin::signed(ALICE), AMANI));

		let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> = b"metadata0".to_vec().try_into().unwrap();

		assert_ok!(NftModule::create_collection(
			Origin::signed(KEZIA),
			NftCollection::OFFICESTEST,
			metadata.clone()
		));

		assert_ok!(OnboardingModule::create_and_submit_proposal(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,
			Some(100),
			metadata,
			false
		));

		let collection_id = NftCollection::OFFICESTEST.value();
		let item_id = pallet_nft::ItemsCount::<Test>::get()[collection_id as usize] - 1;

		assert_ok!(OnboardingModule::change_status(
			Origin::signed(AMANI),
			NftCollection::OFFICESTEST,item_id.clone(),
			crate::Onboarding::AssetStatus::ONBOARDED)
		);

		assert_ok!(BiddingModule::process_asset());

		let event = <frame_system::Pallet<Test>>::events()
			.pop()
			.expect("Expected at least one EventRecord to be found")
			.event;

		// check that the event has been raised
		assert_eq!(
			event,
			mock::Event::BiddingModule(crate::Event::HouseBiddingSucceeded(collection_id, item_id, 100, block_number))
		);
	});
}
