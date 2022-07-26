use frame_support::{assert_noop, assert_ok, traits::tokens::nonfungibles::*};

use super::*;
use mock::*;
use std::convert::TryInto;

type NFTPallet = Pallet<Test>;
pub fn prep_roles(){
    RoleModule::set_role(Origin::signed(CHARLIE).clone(), Acc::SERVICER).ok();
    RoleModule::account_approval(Origin::signed(ALICE),CHARLIE).ok();
    RoleModule::set_role(Origin::signed(EVE).clone(), Acc::SERVICER).ok();
    RoleModule::account_approval(Origin::signed(ALICE),EVE).ok();
    RoleModule::set_role(Origin::signed(BOB).clone(), Acc::SELLER).ok();
    RoleModule::account_approval(Origin::signed(ALICE),BOB).ok();
    RoleModule::set_role(Origin::signed(DAVE).clone(), Acc::INVESTOR).ok();
    RoleModule::set_role(Origin::signed(ACCOUNT_WITH_NO_BALANCE).clone(), Acc::SERVICER).ok();
    RoleModule::account_approval(Origin::signed(ALICE),ACCOUNT_WITH_NO_BALANCE).ok();
}

#[test]
fn create_collection_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
        prep_roles();
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_eq!(
            NFTPallet::collections(COLLECTION_ID_0).unwrap(),
            CollectionInfo {
                created_by: Acc::SERVICER,
                metadata: metadata.clone()
            }
        );

        expect_events(vec![crate::Event::CollectionCreated {
            owner: CHARLIE,
            collection_id: COLLECTION_ID_0,
            created_by: Acc::SERVICER,
        }
        .into()]);

        // not allowed in Permissions
        assert_noop!(
            NFTPallet::create_collection(Origin::signed(BOB), PossibleCollections::OFFICESTEST, metadata.clone()),
            Error::<Test>::NotPermitted
        );

        // existing collection ID
        assert_noop!(
            NFTPallet::create_collection(
                Origin::signed(CHARLIE),
                PossibleCollections::HOUSESTEST,
                metadata.clone()
            ),
            pallet_uniques::Error::<Test>::InUse
        );

        // reserved collection ID
        assert_noop!(
            NFTPallet::create_collection(
                Origin::signed(CHARLIE),
                PossibleCollections::APPARTMENTS,
                metadata
            ),
            Error::<Test>::IdReserved
        );

    })
}

#[test]
fn mint_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
            prep_roles();

        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::OFFICESTEST,
            metadata.clone()
        ));

        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_0,
            metadata.clone()
        ));
        assert_eq!(
            NFTPallet::items(COLLECTION_ID_0, ITEM_ID_0).unwrap(),
            ItemInfo {
                metadata: metadata.clone()
            }
        );

        expect_events(vec![crate::Event::ItemMinted {
            owner: BOB,
            collection_id: COLLECTION_ID_0,
            item_id: ITEM_ID_0,
        }
        .into()]);

        // duplicate item
        assert_noop!(
            NFTPallet::mint(Origin::signed(BOB), PossibleCollections::HOUSESTEST, ITEM_ID_0, metadata.clone()),
            pallet_uniques::Error::<Test>::AlreadyExists
        );

        // not allowed in Permissions
        assert_noop!(
            NFTPallet::mint(Origin::signed(DAVE), PossibleCollections::OFFICESTEST, ITEM_ID_0, metadata.clone()),
            Error::<Test>::NotPermitted
        );


        // invalid collection ID
        assert_noop!(
            NFTPallet::mint(Origin::signed(BOB), PossibleCollections::NONEXISTING, ITEM_ID_0, metadata),
            Error::<Test>::CollectionUnknown
        );
    });
}

#[test]
fn transfer_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
            prep_roles();
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::OFFICESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_0,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::OFFICESTEST,
            ITEM_ID_0,
            metadata
        ));

        // not existing
        assert_noop!(
            NFTPallet::transfer(Origin::signed(CHARLIE), PossibleCollections::APPARTMENTSTEST, ITEM_ID_0, BOB),
            pallet_uniques::Error::<Test>::UnknownCollection
        );

        // not owner
        assert_noop!(
            NFTPallet::transfer(Origin::signed(BOB), PossibleCollections::HOUSESTEST, ITEM_ID_0, DAVE),
            Error::<Test>::NotPermitted
        );

        // not allowed in Permissions
        assert_noop!(
            NFTPallet::transfer(Origin::signed(BOB), PossibleCollections::OFFICESTEST, ITEM_ID_0, DAVE),
            Error::<Test>::NotPermitted
        );

        assert_ok!(NFTPallet::transfer(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST ,
            ITEM_ID_0,
            EVE
        ));
        assert_eq!(NFTPallet::owner(COLLECTION_ID_0, ITEM_ID_0).unwrap(), EVE);

        assert_ok!(NFTPallet::transfer(
            Origin::signed(EVE),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_0,
            BOB
        ));
        assert_eq!(NFTPallet::owner(COLLECTION_ID_0, ITEM_ID_0).unwrap(), BOB);

        expect_events(vec![crate::Event::ItemTransferred {
            from: EVE,
            to: BOB,
            collection_id: COLLECTION_ID_0,
            item_id: ITEM_ID_0,
        }
        .into()]);
    });
}

#[test]
fn burn_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
            prep_roles();

        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::OFFICESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_0,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_1,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::OFFICESTEST,
            ITEM_ID_0,
            metadata
        ));

        
        // not allowed in Permissions
        assert_noop!(
            NFTPallet::burn(Origin::signed(BOB), PossibleCollections::OFFICESTEST, ITEM_ID_0),
            Error::<Test>::NotPermitted
        );

        assert_ok!(NFTPallet::burn(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST , ITEM_ID_0));
        assert!(!<Items<Test>>::contains_key(COLLECTION_ID_0, ITEM_ID_0));

        expect_events(vec![crate::Event::ItemBurned {
            owner: CHARLIE,
            collection_id: COLLECTION_ID_0,
            item_id: ITEM_ID_0,
        }
        .into()]);

        // not existing
        assert_noop!(
            NFTPallet::burn(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST , ITEM_ID_0),
            pallet_uniques::Error::<Test>::UnknownCollection
        );
    });
}

#[test]
fn destroy_collection_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
        prep_roles();
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::OFFICESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST ,
            ITEM_ID_0,
            metadata
        ));

        // existing item
        assert_noop!(
            NFTPallet::destroy_collection(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST ),
            Error::<Test>::TokenCollectionNotEmpty
        );
        assert_ok!(NFTPallet::burn(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST , ITEM_ID_0));

        // not allowed in Permissions
        assert_noop!(
            NFTPallet::destroy_collection(Origin::signed(BOB), PossibleCollections::OFFICESTEST),
            Error::<Test>::NotPermitted
        );

        assert_ok!(NFTPallet::destroy_collection(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST));
        assert_eq!(NFTPallet::collections(COLLECTION_ID_0), None);

        expect_events(vec![crate::Event::CollectionDestroyed {
            owner: CHARLIE,
            collection_id: COLLECTION_ID_0,
        }
        .into()]);

        // not existing
        assert_noop!(
            NFTPallet::destroy_collection(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST),
            Error::<Test>::CollectionUnknown
        );
    });
}

#[test]
fn deposit_works() {
    ExtBuilder::default().build().execute_with(|| {
        let metadata: BoundedVec<u8, <Test as pallet_uniques::Config>::StringLimit> =
            b"metadata".to_vec().try_into().unwrap();
        
        prep_roles();
        let collection_deposit = <Test as pallet_uniques::Config>::CollectionDeposit::get();
        let initial_balance = <Test as pallet_uniques::Config>::Currency::free_balance(&CHARLIE);
        // has deposit
        assert_eq!(<Test as pallet_uniques::Config>::Currency::reserved_balance(&CHARLIE), 0);
        
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_eq!(
            <Test as pallet_uniques::Config>::Currency::free_balance(&CHARLIE),
            initial_balance - collection_deposit
        );
        assert_eq!(
            <Test as pallet_uniques::Config>::Currency::reserved_balance(&CHARLIE),
            collection_deposit
        );

        assert_ok!(NFTPallet::destroy_collection(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST));
        assert_eq!(
            <Test as pallet_uniques::Config>::Currency::free_balance(&CHARLIE),
            initial_balance
        );
        assert_eq!(<Test as pallet_uniques::Config>::Currency::reserved_balance(&CHARLIE), 0);

        // no deposit
        assert_ok!(NFTPallet::create_collection(
            Origin::signed(CHARLIE),
            PossibleCollections::HOUSESTEST,
            metadata.clone()
        ));
        assert_ok!(NFTPallet::mint(
            Origin::signed(BOB),
            PossibleCollections::HOUSESTEST,
            ITEM_ID_0,
            metadata
        ));
        assert_eq!(
            <Test as pallet_uniques::Config>::Currency::free_balance(&BOB),
            initial_balance
        );
        assert_eq!(<Test as pallet_uniques::Config>::Currency::reserved_balance(&BOB), 0);

        assert_ok!(NFTPallet::burn(Origin::signed(CHARLIE), PossibleCollections::HOUSESTEST, ITEM_ID_0));
        assert_eq!(
            <Test as pallet_uniques::Config>::Currency::free_balance(&BOB),
            initial_balance
        );
        assert_eq!(<Test as pallet_uniques::Config>::Currency::reserved_balance(&BOB), 0);

        
    })
}

#[test]
fn create_typed_collection_should_not_work_without_deposit_when_deposit_is_required() {
    ExtBuilder::default().build().execute_with(|| {
        prep_roles();
        assert_noop!(
            NFTPallet::create_typed_collection(ACCOUNT_WITH_NO_BALANCE, COLLECTION_ID_0),
            pallet_balances::Error::<Test>::InsufficientBalance
        );
    });
}

#[test]
fn create_typed_collection_should_not_work_when_not_permitted() {
    ExtBuilder::default().build().execute_with(|| {
        prep_roles();
        assert_noop!(
            NFTPallet::create_typed_collection(DAVE, COLLECTION_ID_0),
            Error::<Test>::NotPermitted
        );
    });
}

