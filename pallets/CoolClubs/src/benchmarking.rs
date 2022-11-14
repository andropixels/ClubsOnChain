

use super::*;
// pub use crate::{mock::*, Error};


use crate::{Pallet as MyClub,*, AccountIdLookupOf};
use frame_benchmarking::{account,benchmarks, impl_benchmark_test_suite};
use frame_support::pallet_prelude::DispatchResult;
use frame_system::{RawOrigin};
use frame_support::{assert_ok, assert_noop};
use frame_support::traits::{Currency};

const BALANCE_FACTOR: u32 = 100_000_000;

/*
    As per the runtime 
    MILLISECS_PER_BLOCK = 6000
    SECS_PER_BLOCK = 6

    SECS_PER_YEAR = 31_557_600 [(365.25 * 24 * 60 * 60)]

    YEARS = SECS_PER_YEAR / SECS_PER_BLOCK


*/
pub const YEARS:u32 = 31_557_600_u32/6;
type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;
/// function for calculating the expiration period of member from membership_period
fn membership_expire_period_test_benchmark<T: Config>(membership_period_in_years:u32, block_number_now:BlockNumberOf<T>)->BlockNumberOf<T> {
    let  years:BlockNumberOf<T> = membership_period_in_years.into(); 
	let block_number_years = years* YEARS.into();

    
    let membership_expiration_time = block_number_now+ block_number_years; 

    membership_expiration_time

}

/// function for club creation it does not needs any checks as it is only for testing purpose
fn benchmark_create_club<T:Config>(owner_lookup:AccountIdLookupOf<T>,initial_annual_expence:BalanceOf<T>) -> DispatchResult {

    MyClub::<T>::create_club(RawOrigin::Root.into(),owner_lookup,initial_annual_expence)
}


/// grab new account with infinite balance.
fn endowed_account<T: Config>(name: &'static str, index: u32) -> T::AccountId {
	let account: T::AccountId = account(name, index, 23);
	
	let amount = BalanceOf::<T>::from(BALANCE_FACTOR);
	let _ = T::Currency::make_free_balance_be(&account, amount);

	T::Currency::issue(amount);

	account
}


/// Account to lookup type of system trait.
fn as_lookup<T: Config>(account: T::AccountId) -> AccountIdLookupOf<T> {
	T::Lookup::unlookup(account)
}


benchmarks!{

    create_club {
        
        // set up the owner and annual_expence
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
     }:{
         assert_ok!(
            MyClub::<T>::create_club(RawOrigin::Root.into(),owner_lookup,initial_annual_expences)
         );
     }
     verify {
        // we can read the clubs with the exact keys 
        let club =  MyClub::<T>::read_clubs(owner, club_id).unwrap(); 
        assert_eq!(club.annual_expences, initial_annual_expences); 
     }

      add_members{

        //set up the owner and annual_expence of the club
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
        assert_ok!(
            benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
         );

        // set the member to be added in club 
        let member_ship_period = 1_u32;
        let member = endowed_account::<T>("member",1);
        let member_lookup = as_lookup::<T>(member.clone());
    
    }:{
        assert_ok!(
            MyClub::<T>::add_members(RawOrigin::Signed(owner.clone()).into(),member_lookup,club_id,member_ship_period)

        );
     }
     verify{
        // verifying if the mebership exists after adding in the club
        let key = (member.clone(),club_id);
        let alice_membership_data = MyClub::<T>::read_membership(key).unwrap(); 
        assert_eq!(alice_membership_data.membership_period_in_years, member_ship_period);
     }
    
    transfer_ownership{

        //  set the owner and create the club    
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
        assert_ok!(
            benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
        );
      // set the new_owner to transfer ownership to
        let new_owner = endowed_account::<T>("new_owner",1);
        let new_owner_lookup = as_lookup::<T>(new_owner.clone());
        let fees = 1000_u32; 

        let fees_amount = BalanceOf::<T>::from(fees);

     }:{
        assert_ok!(
            MyClub::<T>::transfer_ownership(RawOrigin::Signed(owner.clone()).into(),new_owner_lookup.clone(),fees_amount,club_id )

        );
     } verify{
        // verifying that new owner can add member to the club
        // as the ownership transferred
        assert_noop!(
            MyClub::<T>::add_members(RawOrigin::Signed(owner).into(), new_owner_lookup, club_id, 1),
            Error::<T>::ClubDoesNotExistORWrongOwner

        );
     }

     set_the_annual_expences{
        
        // setting the owner                  
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
        assert_ok!(
            benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
        );
       // new annual_expences     
        let new_annual_expence =10000_u32.into();
     }:{
        assert_ok!(
            MyClub::<T>::set_the_annual_expences(RawOrigin::Signed(owner.clone()).into(),new_annual_expence,club_id)

        );
     }
     verify{
        // verifying by reading club if the new_annual_expence has been set or not 
        let club =  MyClub::<T>::read_clubs(owner, club_id).unwrap(); 
        assert_eq!(club.annual_expences, new_annual_expence); 
     }

     cancel_membership{

        //owner of the club
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        // annual expences of the club
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
        // create club
        assert_ok!(
            benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
        );

        // member to be added 
         let member = endowed_account::<T>("member",1);
         let member_ship_period = 1_u32;
         let member_lookup = as_lookup::<T>(member.clone());
         
        assert_ok!(
            MyClub::<T>::add_members(RawOrigin::Signed(owner.clone()).into(),member_lookup.clone(),club_id,member_ship_period)

        );
        // member added 
        let key = (member.clone(),club_id);
        // reading membership to check if the membership has been added
        // and membership_period is correct
        let charlie_membership_data = MyClub::<T>::read_membership(key).unwrap(); 
        assert_eq!(charlie_membership_data.membership_period_in_years, member_ship_period);

     }:{
      assert_ok!(
            MyClub::<T>::cancel_membership(RawOrigin::Signed(owner.clone()).into(),member_lookup,club_id)
         );
     }
      verify{
        // after membership has been cancelled there will be no key for that member
        let key = (member,club_id);
        assert_eq!( MyClub::<T>::read_membership(key).is_err(), true); 
     }

    on_initialize {

        // Club Owner
        let owner = endowed_account::<T>("owner",0);
        let owner_lookup = as_lookup::<T>(owner.clone());
        let initial_annual_expences = 1000_u32.into();
        let club_id = 1;
        // create the club
        assert_ok!(
            benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
        );
         // member to be added 
         let member_ship_period = 1_u32;
         let member = endowed_account::<T>("member",1);
         let member_lookup = as_lookup::<T>(member.clone());
         // we set the blocknumber as 10 
         // inorder to calculate the expiration period of member
         let block_number_10 = BlockNumberOf::<T>::from(10_u32);
         frame_system::Pallet::<T>::set_block_number(block_number_10);
         let expiration_period_of_member = membership_expire_period_test_benchmark::<T>(member_ship_period, block_number_10);

         // add the member 
        assert_ok!(
            MyClub::<T>::add_members(RawOrigin::Signed(owner.clone()).into(),member_lookup.clone(),club_id,member_ship_period)

        );

        // now lets set the block_number to the expiration period_of_member 
        frame_system::Pallet::<T>::set_block_number(expiration_period_of_member);
        
        // now the membership  will be expired 
    }:{
         assert_ok!(
            MyClub::<T>::membership_expired(expiration_period_of_member)
        );
      
    }

    verify{

        // the membership period of member will be updated after expiration
       let new_test_expiration_time = membership_expire_period_test_benchmark::<T>(member_ship_period, expiration_period_of_member);

        let expired_on =  MyClub::<T>::read_expired_on(new_test_expiration_time).unwrap();

        assert_eq!(expired_on.contains(&(member,club_id,owner)), true);

    }

    update_membership_period{
    // Club Owner
    let owner = endowed_account::<T>("owner",0);
    let owner_lookup = as_lookup::<T>(owner.clone());
    let initial_annual_expences = 1000_u32.into();
    let club_id = 1;
    // create the club
    assert_ok!(
        benchmark_create_club::<T>(owner_lookup.clone(),initial_annual_expences)
    );
    // member to be added 
    let member_ship_period = 1_u32;
    let member = endowed_account::<T>("member",1);
    let member_lookup = as_lookup::<T>(member.clone());
    // add member
    assert_ok!(
        MyClub::<T>::add_members(RawOrigin::Signed(owner.clone()).into(),member_lookup.clone(),club_id,member_ship_period)

    );
        let key = (member.clone(),club_id);
        let membership_data = MyClub::<T>::read_membership(key).unwrap();
        assert_eq!(membership_data.membership_period_in_years, member_ship_period);
    // now member will update the membership period

    let new_membership_period:u32 = 2;
   
    }:{

        assert_ok!(
            MyClub::<T>::update_membership_period(RawOrigin::Signed(member.clone()).into(),club_id,new_membership_period)
        );
    }
    verify{
        // reading memnber ship data checking if the membership period is updated
        let key = (member,club_id);
        let membership_data = MyClub::<T>::read_membership(key).unwrap();
        assert_eq!(membership_data.membership_period_in_years, new_membership_period)

     }
}



impl_benchmark_test_suite!(
    MyClub,
    crate::mock::Extuilder::default().build(),
    crate::mock::Test,
);

