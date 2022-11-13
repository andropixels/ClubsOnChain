pub use crate::{mock::*, Error};
pub use frame_support::{assert_noop, assert_ok};
pub use frame_system::RawOrigin;

/// function for calculating the membership_period in test
fn membership_expire_period_test(membership_period_in_years:u32, block_number_now:BlockNumber)->BlockNumber {
    let  years:BlockNumber = membership_period_in_years.into(); 
	let block_number_years = years* YEARS;
    let membership_expiration_time = block_number_now+ block_number_years; 

    membership_expiration_time

}


#[test]
fn create_club_works() {
    
    Extuilder::default().build().execute_with(

        || {

                // BOB is the owner of club 
                let annual_expences_of_club_1:u128 = 1000;

                assert_ok!(
                    ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
                );
                // get the club of BOB 
                let bob_club_1 = ClubsOnChain::read_clubs(BOB, 1).unwrap();
                // There will be no members in the Club initially or unitl owner adds one     
                assert_eq!(bob_club_1.members.len(), 0);
                //annual_expence of the Bob's Club should be equal to 1000
                assert_eq!(bob_club_1.annual_expences, 1000);

                // the balance of FeeCollector will be update to 100000000 as it is fee to 
                // create a club
                assert_eq!(ClubsOnChain::check_balance_of_fee_collector(), 100000000);

                /*
                 As per the problem statement the owner has to pay for creating a club 
                 that means one owner can create multiple clubs 

                */    

                // Bob creating another club 
                let annual_expences_of_club_2:u128 = 2000; 
                assert_ok!(
                    ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_2)
                );

                // now the balance of fee collector will be 200000000
                // as two clubs got created 
                assert_eq!(ClubsOnChain::check_balance_of_fee_collector(), 200000000);

                  // get the Second club of BOB 
                  let bob_club_2 = ClubsOnChain::read_clubs(BOB, 2).unwrap();
                 // There will be no members in the Club initially or unitl owner adds one  
                  assert_eq!(bob_club_2.members.len(), 0);
                  //annual_expence of the Bob's Club should be equal to 2000
                  assert_eq!(bob_club_2.annual_expences, 2000);

                  // Now Charlie is Creating a club 
                  let charlie_club_annual_expence = 3000 ;
                  assert_ok!(
                    ClubsOnChain::create_club(RawOrigin::Root.into(), CHARLIE, charlie_club_annual_expence)
                );  

                // now the balance of fee collector will be 300000000
                // as three clubs got created 
                assert_eq!(ClubsOnChain::check_balance_of_fee_collector(), 300000000);

                 // get the Second club of BOB 
                 let charlie_club_1 = ClubsOnChain::read_clubs(CHARLIE, 3).unwrap();
                 // There will be no members in the Club initially or unitl owner adds one   
                 assert_eq!(charlie_club_1.members.len(), 0);
                 //annual_expence of the Bob's Club should be equal to 2000
                 assert_eq!(charlie_club_1.annual_expences, 3000);   
        }
    )

}


#[test]
fn add_member_works() {

        Extuilder::default().build().execute_with(
            
                || {
        

        // Create the club
         let annual_expences_of_club_1:u128 = 1000;

         assert_ok!(
             ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
         );

        let alice_membership_period:u32 = 1; 

         // Lets add alice add bloc_number 10
         System::set_block_number(10);
         let alices_membership_expiration_period = membership_expire_period_test(alice_membership_period, 10);
        
        // reading the BOB's Club now
        let _bobclub = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
        
         let club_id:u64 =1;
        // Now Bob will add Alice as A member in his club 
        assert_ok!(
            ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), ALICE, club_id, alice_membership_period)
        );

        // The Memership will be updated as (ALICE, club_id)
        let key = (ALICE,club_id);
        let alice_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        assert_eq!(alice_membership_data.membership_period_in_years, alice_membership_period);
        // ALICE's Memebership will be expired on this blocknumber 
        assert_eq!(alice_membership_data.expired_on,alices_membership_expiration_period); 
        let expired_on = ClubsOnChain::read_expired_on(alices_membership_expiration_period).unwrap();

        // Expired On is updated with the ALICE's Expiration pair(member,clubid,clubOwner)
        assert_eq!(expired_on.contains(&(ALICE,club_id,BOB)), true);

        let bobclub = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
       
       // member count of BOB's Club will be updated to 1 as ALICE got in 
        assert_eq!(bobclub.members.len(), 1);     
        // Check if the ALICE is the first member
        assert_eq!(bobclub.members[0], ALICE);     

        //  if BOB tries to add ALICE again as a member in the same club it will fail 
        assert_noop!(
                ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), ALICE, 1, alice_membership_period),
                Error::<Test>::MemberAlreadyExist
        );
        

        let charlies_membership_period:u32 = 2; 

        // lets add charlie at block_number 20
        System::set_block_number(20);
        let charlies_expiration_period = membership_expire_period_test(charlies_membership_period, 20);
        // BOB adding CHARLIE as a next member

        assert_ok!(
            ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), CHARLIE, club_id, charlies_membership_period)
        );

        // check if the CHARLIE membership is updated with BOB's club_id
        let key = (CHARLIE,club_id);
        let charlie_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        assert_eq!(charlie_membership_data.membership_period_in_years, charlies_membership_period);
         
        // check the membership expiration period of CHARLIE
        assert_eq!(charlie_membership_data.expired_on,charlies_expiration_period); 

         // reading the BOB's Club now
        let bobclub = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
        
        // member count of BOB's Club will be updated to 1 as ALICE got in 
        assert_eq!(bobclub.members.len(), 2);     
        // Check if the ALICE is the first member and CHARLIE is the second member
        assert_eq!(bobclub.members[1], CHARLIE);   
        assert_eq!(bobclub.members[0], ALICE);     
           
        }

    )

}



#[test]
fn transfer_ownership_works() {

    Extuilder::default().build().execute_with(

        || {

            // BOB has a club with club_id = 1
            let annual_expences_of_club_1:u128 = 1000;
            let club_id =1;
            assert_ok!(
                ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
            );
            // Now Alice wants the ownership for that she needs to pay fees to bob 
            let fees:u128 = 100000;
            
            assert_ok!(
                ClubsOnChain::transfer_ownership(RuntimeOrigin::signed(BOB), ALICE, fees, club_id),

            );

            // After ownership transfer you cannot read the club with BOB 
          assert_eq!(
                ClubsOnChain::read_clubs(BOB, club_id).is_err(),
                true
            );
            
            // The Ownership transfered from BOB to ALICE 
            // Now BOB will not be able to mutate the club in any way 
            // for e.g BOB adding member to club will not work 
            assert_noop!(
                ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), CHARLIE, club_id, 1),
                Error::<Test>::ClubDoesNotExistORWrongOwner

            );

            // On the other hand ALICE adding new member to the club will work
            assert_ok!(
                ClubsOnChain::add_members(RuntimeOrigin::signed(ALICE), CHARLIE, club_id, 1)
            );

            // Or BOB can not transfer ownership as he is not the owner anymore
            assert_noop!(
                ClubsOnChain::transfer_ownership(RuntimeOrigin::signed(BOB), ALICE, fees, club_id),
                Error::<Test>::ClubDoesNotExistORWrongOwner
            );

            // or BOB will not be able to set the new annual_expences
            let new_annual_expence = 10000;
            assert_noop!(
                ClubsOnChain::set_the_annual_expences(RuntimeOrigin::signed(BOB), new_annual_expence, club_id),
                Error::<Test>::ClubDoesNotExistORWrongOwner
            );

           // but ALICE will be able to set the new_annual_expence
           assert_ok!(
            ClubsOnChain::set_the_annual_expences(RuntimeOrigin::signed(ALICE), new_annual_expence, club_id),
         
            );
             // reading club with ALICE will work as she is the new owner
            let alice_club = ClubsOnChain::read_clubs(ALICE, club_id).unwrap();    

            assert_eq!(alice_club.annual_expences, new_annual_expence);
        }

    )
}

#[test]
fn set_annual_expences_works() {


    Extuilder::default().build().execute_with( 

        || {

            // lets create a club with BOB 

        let annual_expences_of_club_1:u128 = 1000;
            let club_id =1;
            assert_ok!(
                ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
            );

            let club = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
            assert_eq!(club.annual_expences, annual_expences_of_club_1); 

            let new_annual_expence = 10000;

            assert_ok!(
               
                ClubsOnChain::set_the_annual_expences(RuntimeOrigin::signed(BOB), new_annual_expence, club_id),
             
            );

            let club = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
            assert_eq!(club.annual_expences, new_annual_expence); 
        
            // if anyone other than owner tries to set the annual_expences it will not work 
        assert_noop!(
                ClubsOnChain::set_the_annual_expences(RuntimeOrigin::signed(ALICE), new_annual_expence, club_id),
                Error::<Test>::ClubDoesNotExistORWrongOwner
            );


        }
    )
}



#[test]
fn renewal_of_membership_works() {

        /*
            WHEN THE MEMBERSHIP IS EXPIRED WE RENEW IT BY DEFAULT UNTIL MEMBER HAS CANCELED THE 
            MEMBERSHIP    

            BY DEFAULT THE RENEWAL PERIOD WILL BE  CALCULATED ON  THE 'membership_period'
            MEMBER HAS SET IT TO AND SHE WILL AGAIN HAVE TO PAY THAT AMOUNT
            TO THE OWNER 
        */


      Extuilder::default().build().execute_with(
        
        || {
        
        // Create the club
         let annual_expences_of_club_1:u128 = 3000;

         assert_ok!(
             ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
         );

        let alice_membership_period:u32 = 3; 

        // reading the BOB's Club now
        let _bobclub = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
        
         let club_id:u64 =1;

        /*
          for simple calculations lets set block Number to 10 
           so Now Bob will add Alice as A member in his club with blockNumber set to 10
        */
        System::set_block_number(10);
        let test_expiration_period = membership_expire_period_test(alice_membership_period, 10);
        assert_ok!(
            ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), ALICE, club_id, alice_membership_period)
        );
       
        // The Memership will be updated as (ALICE, club_id)
        let key = (ALICE,club_id);
        let alice_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        assert_eq!(alice_membership_data.membership_period_in_years, alice_membership_period);
        let expired_on = ClubsOnChain::read_expired_on(test_expiration_period).unwrap();
        
        // ALICE's Memebership will be expired on this blocknumber
        assert_eq!(alice_membership_data.expired_on,test_expiration_period); 
        // Expired On is updated with the ALICE's Expiration pair(member,clubid,clubOwner)
        assert_eq!(expired_on.contains(&(ALICE,club_id,BOB)), true);

        // now we set our blocknumber to the ALICE's expiration blocknumber
        System::set_block_number(test_expiration_period);

        assert_ok!(
                ClubsOnChain::membership_expired(test_expiration_period)
            );

        // Now the ALICE's Membership is renewd with the new expiration time 
       let new_test_expiration_time = membership_expire_period_test(alice_membership_data.membership_period_in_years, test_expiration_period);

          
        // Membership  of ALICE in BOB's club is successfully renewd with new expiration time 
        let expired_on = ClubsOnChain::read_expired_on(new_test_expiration_time).unwrap();

        assert_eq!(expired_on.contains(&(ALICE,club_id,BOB)), true);

   
        }
      )
    
}


#[test]
fn cancel_membership_works() {

    Extuilder::default().build().execute_with(
        

        || {


        // Create the club
         let club_id:u64 =1;

         let annual_expences_of_club_1:u128 = 1000;

         assert_ok!(
             ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
         );

        let alice_membership_period:u32 = 1; 

        
        // Now Bob will add Alice as A member in his club 
        assert_ok!(
            ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), ALICE, club_id, alice_membership_period)
        );

        // lets read mebership to ensure that ALICE became part of BOB's club
        let key = (ALICE,club_id);
        let alice_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        // read the ExpireOn
        let expired_on = ClubsOnChain::read_expired_on(alice_membership_data.expired_on).unwrap();
        assert_eq!(expired_on.contains(&(ALICE,club_id,BOB)), true);

        // Now ALICE wants to remove  her membership from this club 

        assert_ok!(
            ClubsOnChain::cancel_membership(RuntimeOrigin::signed(BOB), ALICE, club_id)
        );

        // now lets read membership  again if its error  then we can be sure that the  membership is removed 

        assert_eq!(ClubsOnChain::read_membership(key).is_err(), true); 


        let expired_on = ClubsOnChain::read_expired_on(alice_membership_data.expired_on).unwrap();

        // after membership cancelation there will be no pair for ALICE and BOB's Club id
        assert_eq!(expired_on.contains(&(ALICE,club_id,BOB)), false);
        }
    )       
}

#[test]
fn update_membership_period_works() {


    Extuilder::default().build().execute_with(
        
        || {
              // Create the club
         let annual_expences_of_club_1:u128 = 1000;

         assert_ok!(
             ClubsOnChain::create_club(RawOrigin::Root.into(), BOB, annual_expences_of_club_1)
         );

        let alice_membership_period:u32 = 1; 

        // reading the BOB's Club now
        let _bobclub = ClubsOnChain::read_clubs(BOB, 1).unwrap(); 
        
         let club_id:u64 =1;
        // Now Bob will add Alice as A member in his club 
        assert_ok!(
            ClubsOnChain::add_members(RuntimeOrigin::signed(BOB), ALICE, club_id, alice_membership_period)
        );

        // The Memership will be updated as (ALICE, club_id)
        let key = (ALICE,club_id);
        let alice_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        assert_eq!(alice_membership_data.membership_period_in_years,alice_membership_period);

        // Now ALICE will change her membership period 
        let new_membership_period = 3_u32; 
        assert_ok!(
            ClubsOnChain::update_membership_period(RuntimeOrigin::signed(ALICE), club_id, new_membership_period)
        );
        let key = (ALICE,club_id);
        let alice_membership_data = ClubsOnChain::read_membership(key).unwrap(); 
        // ALICE's membership will be updated with new_membership_period
        assert_eq!(alice_membership_data.membership_period_in_years,new_membership_period);


        }
    )
}

