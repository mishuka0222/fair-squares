
pub use super::*;
pub use frame_support::pallet_prelude::*;
pub use scale_info::prelude::boxed::Box;
pub use sp_core::H256;
use sp_runtime::{
	traits::{StaticLookup, Zero},
};
impl<T: Config> Pallet<T> {
	pub fn approve_representative(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
		let caller = ensure_signed(origin.clone())?;
		let mut representative = Roles::Pallet::<T>::get_pending_representatives(&who).unwrap();
		representative.activated = true;
		representative.assets_accounts.clear();
		representative.assets_accounts.push(caller);
		//get Rep number
		let mut index = Roles::Pallet::<T>::rep_num();
		//Update Rep index
		representative.index = index;

		Roles::RepresentativeLog::<T>::insert(&who, representative);
		Roles::RepApprovalList::<T>::remove(&who);
		Roles::AccountsRolesLog::<T>::insert(&who, Roles::Accounts::REPRESENTATIVE);
		let who2 = T::Lookup::unlookup(who.clone());

		//Check that the Representative is not already a Registrar
		//If a Representative is revoked from a given asset, and approved
		//for another asset, we don't want to repeat the registrar settings

		let mut check0 = false;
		let v = Ident::Pallet::<T>::registrars();
		for i in v {
			let reg = i.unwrap();
			if reg.account == who.clone(){
				check0 = true;
			}
		}

		if check0 == false {
			//Set the representative as a registrar
		Ident::Pallet::<T>::add_registrar(origin, who2).ok();

		//Set registrar fields
		let origin2: OriginFor<T> = RawOrigin::Signed(who).into();
		Ident::Pallet::<T>::set_fields(origin2.clone(), index, Default::default()).ok();

		//Set registrar fees
		let fee0 = Self::balance_to_u128_option1(T::RepFees::get()).unwrap();
		let fees = Self::u128_to_balance_option1(fee0).unwrap();
		Ident::Pallet::<T>::set_fee(origin2, index, fees).ok();
		
		//Update Rep number
		index += 1;
		Roles::RepNumber::<T>::put(index);
		}
		

		

		Ok(())
	}

	pub fn revoke_representative(who: T::AccountId) -> DispatchResult {
		Roles::RepresentativeLog::<T>::mutate(&who, |val| {
			let mut val0 = val.clone().unwrap();
			val0.activated = false;
			*val = Some(val0);
		});
		Roles::AccountsRolesLog::<T>::remove(&who);

		Ok(())
	}

	pub fn guaranty_payment(
		origin: OriginFor<T>,
		from: T::AccountId,
		collection: T::NftCollectionId,
		item: T::NftItemId,
	) -> DispatchResult {

		let creator = ensure_signed(origin.clone())?;

		//Calculate guaranty deposit using Return On Rent and guaranty coefficients found in runtime 
		let coeff = T::Guaranty::get() as u128;
		let ror = T::RoR::get() as u64;
		let price0 = Onboarding::Pallet::<T>::houses(collection,item).unwrap().price.unwrap();
		let price1 = Onboarding::Pallet::<T>::balance_to_u64_option(price0).unwrap();
		let rent:u128 = ((ror*price1)/1200).into();
		let amount:u128 = coeff * rent;
		//convert amount to payment_pallet compatible balance
		let amount1 = Payment::Pallet::<T>::u128_to_balance_option(amount).unwrap();

		//create payment_request
		Payment::Pallet::<T>::request_payment(origin,from.clone(),amount1).ok();

		//Store payment details
		let details = Payment::Pallet::<T>::get_payment_details(&from,&creator).unwrap();
		GuarantyPayment::<T>::insert(from.clone(),creator.clone(),details);

		
		
		
		

		Ok(())
	}

	pub fn tenant_link_asset(
		tenant: T::AccountId,
		collection: T::NftCollectionId,
		item: T::NftItemId,
		asset_account: T::AccountId,
	) -> DispatchResult {
		// Update tenant info
		let coeff = T::RoR::get() as u64;
		Roles::TenantLog::<T>::mutate(&tenant, |val| {
			let mut val0 = val.clone().unwrap();
			// get asset price
			let price0 = Onboarding::Pallet::<T>::houses(collection,item).unwrap().price.unwrap();
			let price1 = Onboarding::Pallet::<T>::balance_to_u64_option(price0).unwrap();
			//update rent in tenant infos added
			let rent0:u128 = ((coeff*price1)/1200).into();
			let rent = Roles::Pallet::<T>::u128_to_balance_option(rent0).unwrap();
			val0.rent = rent.into();
			val0.asset_account = Some(asset_account);
			*val = Some(val0);
		});

		// Update asset info
		Onboarding::Houses::<T>::mutate(collection, item, |house| {
			let mut house0 = house.clone().unwrap();
			house0.tenants.push(tenant);
			*house = Some(house0);
		});

		Ok(())
	}

	pub fn tenant_unlink_asset(
		tenant: T::AccountId,
		collection: T::NftCollectionId,
		item: T::NftItemId,
	) -> DispatchResult {
		// Update tenant info
		Roles::TenantLog::<T>::mutate(&tenant, |val| {
			let mut val0 = val.clone().unwrap();
			val0.asset_account = None;
			*val = Some(val0);
		});

		// Update asset info
		Onboarding::Houses::<T>::mutate(collection, item, |house| {
			let mut house0 = house.clone().unwrap();
			house0.tenants.retain(|t| *t != tenant);
			*house = Some(house0);
		});

		Ok(())
	}

	pub fn create_proposal_hash_and_note(
		caller: T::AccountId,
		proposal_call: pallet::Call<T>,
	) -> T::Hash {
		let origin: <T as frame_system::Config>::Origin = RawOrigin::Signed(caller.clone()).into();
		let proposal = Box::new(Self::get_formatted_call(proposal_call.into()));

		let call = Call::<T>::execute_call_dispatch { account_id: caller, proposal };
		let call_formatted = Self::get_formatted_call(call.into());
		let call_dispatch = Box::new(call_formatted);

		let proposal_hash = T::Hashing::hash_of(&call_dispatch);
		let proposal_encoded: Vec<u8> = call_dispatch.encode();
		match Dem::Pallet::<T>::note_preimage(origin, proposal_encoded) {
			Ok(_) => (),
			Err(x) if x == Error::<T>::DuplicatePreimage.into() => (),
			Err(x) => panic!("{:?}", x),
		}
		proposal_hash
	}

	pub fn caller_can_vote(caller: &T::AccountId, ownership: Share::Ownership<T>) -> bool {
		let owners = ownership.owners;
		owners.contains(caller)
	}

	pub fn balance_to_u128_option(input: <T as Assetss::Config>::Balance) -> Option<u128> {
		input.try_into().ok()
	}
	pub fn u128_to_balance_option(input: u128) -> Option<DemoBalanceOf<T>> {
		input.try_into().ok()
	}

	pub fn balance_to_u128_option1(input: BalanceOf<T>) -> Option<u128> {
		input.try_into().ok()
	}

	pub fn u128_to_balance_option1(input: u128) -> Option<IdentBalanceOf<T>> {
		input.try_into().ok()
	}

	pub fn get_formatted_call(call: <T as Config>::Call) -> <T as Config>::Call {
		call
	}

	pub fn begin_block(now: T::BlockNumber) -> Weight {
		let max_block_weight = Weight::from_ref_time(1000_u64);
		if (now % <T as Config>::CheckPeriod::get()).is_zero() {
			let indexes = ProposalsIndexes::<T>::iter();
			for index in indexes {
				//check if the status is Finished
				let ref_infos: RefInfos<T> = Dem::Pallet::<T>::referendum_info(index.1).unwrap();
				let b = match ref_infos {
					pallet_democracy::ReferendumInfo::Finished { approved, end: _ } =>
						(1, approved),
					_ => (0, false),
				};
				if b.0 == 1 {
					//get the local prop_infos and update vote result if referendum ended
					ProposalsLog::<T>::mutate(index.1, |val| {
						let mut val0 = val.clone().unwrap();
						if b.1 {
							val0.vote_result = VoteResult::ACCEPTED
						} else {
							val0.vote_result = VoteResult::REJECTED
						}
						*val = Some(val0)
					});
				}
			}
		}
		max_block_weight
	}
}
