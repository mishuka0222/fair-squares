pub use super::*;
pub use frame_support::{
	assert_ok,
	dispatch::{DispatchResult, EncodeLike},
	inherent::Vec,
	pallet_prelude::*,
	sp_runtime::{
		traits::{AccountIdConversion, Hash, One, Saturating, StaticLookup, Zero},
		PerThing, Percent,
	},
	storage::child,
	traits::{
		Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency, WithdrawReasons,
	},
	PalletId,
};

pub use frame_system::{ensure_signed, pallet_prelude::*, RawOrigin};
pub use scale_info::{
	prelude::{boxed::Box, format, vec},
	TypeInfo,
};
pub use serde::{Deserialize, Serialize};
pub use Ident::IdentityInfo;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type BalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct RegisteredTenant<T: Config> {
	/// account id
	pub account_id: T::AccountId,
	///infos
	pub infos: Box<IdentityInfo<T::MaxAdditionalFields>>,
	///Creation Blocknumber
	pub registered_at_block: BlockNumberOf<T>,
	///Asset requested by the tenant
	pub asset_requested: Option<T::AccountId>,
}

impl<T: Config> RegisteredTenant<T> {
	pub fn new(
		tenant_id: T::AccountId,
		infos: Box<IdentityInfo<T::MaxAdditionalFields>>,
		asset_requested: Option<T::AccountId>,
	) -> DispatchResult {
		let registered_at_block = <frame_system::Pallet<T>>::block_number();
		let tenant = RegisteredTenant::<T> {
			account_id: tenant_id.clone(),
			infos,
			registered_at_block,
			asset_requested,
		};
		Tenants::<T>::insert(tenant_id.clone(), tenant);
		Roles::TenantLog::<T>::mutate(tenant_id, |val| {
			let mut val0 = val.clone().unwrap();
			val0.registered = true;
			*val = Some(val0);
		});
		Ok(())
	}
}
