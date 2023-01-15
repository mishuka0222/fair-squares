pub use super::*;
pub use frame_support::{
	assert_ok,
	dispatch::{DispatchResult,DispatchResultWithPostInfo, EncodeLike},
	inherent::Vec,fail,require_transactional,
	pallet_prelude::*,
	sp_runtime::{
		traits::{AccountIdConversion, Hash, One, Saturating, StaticLookup, Zero},
		FixedU128, PerThing, Percent,
	},
	storage::{child,bounded_btree_map::BoundedBTreeMap},
	traits::{
		Contains, Currency, ExistenceRequirement, Get, LockableCurrency, ReservableCurrency,
		UnfilteredDispatchable, WithdrawReasons,
		tokens::{BalanceStatus,ExistenceRequirement::KeepAlive},
	},
	weights::GetDispatchInfo,
	PalletId,
};
use codec::{Decode, Encode, HasCompact, MaxEncodedLen};
pub use frame_system::{ensure_signed, pallet_prelude::*, RawOrigin};
pub use scale_info::{prelude::vec, TypeInfo};
pub use sp_runtime::{
	traits::{BadOrigin, BlakeTwo256, IdentityLookup,CheckedAdd},

};
pub use sp_std::boxed::Box;

pub type BalanceOf<T> =
	<<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
pub type HashOf<T> = <T as frame_system::Config>::Hash;
pub type BoundedDataOf<T> = BoundedVec<u8, <T as Config>::MaxRemarkLength>;
pub type ScheduledTaskOf<T> = ScheduledTask<<T as frame_system::Config>::BlockNumber>;
/// list of ScheduledTasks, stored as a BoundedBTreeMap
pub type ScheduledTaskList<T> = BoundedBTreeMap<
(
	<T as frame_system::Config>::AccountId,
	<T as frame_system::Config>::AccountId,
),
ScheduledTaskOf<T>,
<T as Config>::MaxRemarkLength,
>;

/// The PaymentDetail struct stores information about the payment/escrow
/// A "payment" in virto network is similar to an escrow, it is used to
/// guarantee proof of funds and can be released once an agreed upon condition
/// has reached between the payment creator and recipient. The payment lifecycle
/// is tracked using the state field.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PaymentDetail<T: pallet::Config> {
	/// amount used for payment
	#[codec(compact)]
	pub amount: BalanceOf<T>,
	/// incentive amount that is credited to creator for resolving
	#[codec(compact)]
	pub incentive_amount: BalanceOf<T>,
	/// enum to track payment lifecycle [Created, NeedsReview, RefundRequested,
	/// Requested]
	pub state: PaymentState<T>,
	/// account that can settle any disputes created in the payment
	pub resolver_account: T::AccountId,
	/// fee charged and recipient account details
	pub fee_detail: Option<(T::AccountId, BalanceOf<T>)>,
}

/// The `PaymentState` enum tracks the possible states that a payment can be in.
/// When a payment is 'completed' or 'cancelled' it is removed from storage and
/// hence not tracked by a state.
#[derive(Encode, Decode, Debug, Clone, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound(T: pallet::Config))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PaymentState<T: pallet::Config> {
	/// Amounts have been reserved and waiting for release/cancel
	Created,
	/// A judge needs to review and release manually
	NeedsReview,
	/// The user has requested refund and will be processed by `BlockNumber`
	RefundRequested { cancel_block: T::BlockNumber },
	/// The recipient of this transaction has created a request
	PaymentRequested,
}

/// trait that defines how to create/release payments for users
pub trait PaymentHandler<T: pallet::Config> {
	/// Create a PaymentDetail from the given payment details
	/// Calculate the fee amount and store PaymentDetail in storage
	/// Possible reasons for failure include:
	/// - Payment already exists and cannot be overwritten
	fn create_payment(
		from: &T::AccountId,
		to: &T::AccountId,
		amount: BalanceOf<T>,
		payment_state: PaymentState<T>,
		incentive_percentage: Percent,
		remark: Option<&[u8]>,
	) -> Result<PaymentDetail<T>, sp_runtime::DispatchError>;

	/// Attempt to reserve the amount from the caller
	/// If not possible then return Error. Possible reasons for failure include:
	/// - User does not have enough balance.
	fn reserve_payment_amount(from: &T::AccountId, to: &T::AccountId, payment: PaymentDetail<T>) -> DispatchResult;

	// Settle a payment of `from` to `to`. To release a payment, the
	// recipient_share=100, to cancel a payment recipient_share=0
	// Possible reasonse for failure include
	///
	/// - The payment does not exist
	/// - The unreserve operation fails
	/// - The transfer operation fails
	fn settle_payment(from: &T::AccountId, to: &T::AccountId, recipient_share: Percent) -> DispatchResult;

	/// Attempt to fetch the details of a payment from the given payment_id
	/// Possible reasons for failure include:
	/// - The payment does not exist
	fn get_payment_details(from: &T::AccountId, to: &T::AccountId) -> Option<PaymentDetail<T>>;
}

/// DisputeResolver trait defines how to create/assign judges for solving
/// payment disputes
pub trait DisputeResolver<Account> {
	/// Returns an `Account`
	fn get_resolver_account() -> Account;
}

/// Fee Handler trait that defines how to handle marketplace fees to every
/// payment/swap
pub trait FeeHandler<T: pallet::Config> {
	/// Get the distribution of fees to marketplace participants
	fn apply_fees(
		from: &T::AccountId,
		to: &T::AccountId,
		detail: &PaymentDetail<T>,
		remark: Option<&[u8]>,
	) -> (T::AccountId, Percent);
}

/// Types of Tasks that can be scheduled in the pallet
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub enum Task {
	// payment `from` to `to` has to be cancelled
	Cancel,
}

/// The details of a scheduled task
#[derive(PartialEq, Eq, Clone, Encode, Decode, Debug, TypeInfo, MaxEncodedLen)]
pub struct ScheduledTask<Time: HasCompact> {
	/// the type of scheduled task
	pub task: Task,
	/// the 'time' at which the task should be executed
	pub when: Time,
}
