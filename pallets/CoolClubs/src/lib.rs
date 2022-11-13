#![cfg_attr(not(feature = "std"), no_std)]

pub  use pallet::*; 


use sp_runtime::traits::StaticLookup;

#[cfg(test)]
mod mock;

#[cfg(test)]
pub mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

mod types;

#[frame_support::pallet] 

pub mod pallet {

use super::*; 
	use frame_support::{pallet_prelude::{*, DispatchResult, ValueQuery, OptionQuery}, Blake2_128Concat,PalletId, Twox64Concat, traits::EnsureOrigin}; 
	use frame_system::{pallet_prelude::{*, OriginFor}}; 
	use frame_support::{traits::{Currency, ExistenceRequirement, Get, ReservableCurrency},
	
		};
		use sp_runtime::traits::AccountIdConversion;
use sp_std::{vec::Vec};
use sp_runtime::traits::Zero;

use weights::WeightInfo;
use crate::types::*;	

#[pallet::config]
	pub trait Config:frame_system::Config {
		
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency:ReservableCurrency<Self::AccountId>;
	
		/// Fees For Creating a Club
		type FeesToCreateClub:Get<BalanceOf<Self>>;

		/*AS IT IS NOT CLEARLY MENTION IN PROBLEM STATEMENT WHO SHOULD GET PAID THE FEES FOR CREATING A CLUB
		 BY DEFAULT FeeCollector WILL COLLECT FEES FROM OWNERS FOR CREATING A CLUB
		*/
		#[pallet::constant]
		///The Account id to pay fees to for creating the Club
		type FeeCollector: Get<PalletId>;

		/*AS PER THE PROBLEM STATEMENT MAX MEMBER CAN PAY IS FOR THE PERIOD OF 100 YEARS  */
		/// Maximum Member Ship Period will be 100 years 
		type MaxMembershipPeriod: Get<Self::BlockNumber>;

		
		/*WE CAN EASILY SET THE ROOT ROOT IN Runtime   */
		type Root:EnsureOrigin<Self::RuntimeOrigin>;

		///Unit MemberShipPeriod will be set to one year IN terms of blocknumber
		type UnitMembeShipPeriod:Get<Self::BlockNumber>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
		

	}



	pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
	pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountIdOf<T>>>::Balance;
	pub type BlockNumberOf<T> = <T as frame_system::Config>::BlockNumber;

	/*AS THERE CAN BE MULTIPLE MEMBERS IN CLUB */
	///Club Members are Vec<T::AccountId>
    type ClubMembers<T> = Vec<AccountIdOf<T>>;
	/// Club 
	type ClubsOF<T> = Club<BalanceOf<T>,ClubMembers<T>,BlockNumberOf<T>>; 
	/// MemberShip Data Storing Expiration Period
    type MemberShipDataOf<T> = MemberShipData<BlockNumberOf<T>>;



	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		///ClubCreated(Owner, ClubId) when the club is created
		ClubCreated(AccountIdOf<T>,ClubId,BalanceOf<T>),
		/// AnnualExpenceChanged(ClubId, NewAnnualExpence) when the new annual_expence is set
		AnnualExpenceChanged( ClubId, BalanceOf<T>),
		/// MemberAdded(member,clubId) when the new member added in the club of clubId
		MemberAdded(AccountIdOf<T>, ClubId,BlockNumberOf<T>),
		/// OwnershipOfClubTransfered(,club_id) when the owner of club is changed 
		OwnershipOfClubTransfered( ClubId),
		/// MemberShipExpired(Member, club_id) when the member is removed or the membership expired 
		MemebrShipExpired(AccountIdOf<T>, ClubId),
		/// membershipRenewed(member,club_id) when it is renewed after expiration
		MemberShipRenewed(AccountIdOf<T>,ClubId),
		/// MembershipPeriodUpdated(member,clubId) as member can change the membership period
		MembershipPeriodUpdated(AccountIdOf<T>,ClubId)

	}



	#[pallet::error]
	pub enum Error<T> {


		/// If club does not exists with given Owner and club_id as keys
		ClubDoesNotExistORWrongOwner,
		/// Club Id Does not Exits
		ClubIdDoesNotExist,
		/// if the given BlockNumber is not a Membership Expiry time
		NotAExpirationTime,
		/// for a given MemberAddress and clubId MemberShip Does not Exist
		MemberShipDoesNotExist,
		/// for a given MemberAddress and clubId MemberShip Already  Exist
		MemberAlreadyExist,
		/// if the membership period exceeds the given limit
		MemeberShipPeriodExceeds,
		/// if owner tries to create the same club again 
		ClubAlreadyExists,
		/// if a non-owner  tried to access the club 
		ClubDoesNotExists
		
	}



	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_); 

	/*THERE CAN BE MULTIPLE CLUBS WE NEED TO GIVE ID TO EVERY CLUB */
	///Tracking Club ID's
	#[pallet::storage]
	#[pallet::getter(fn next_club_id)]
	pub type NextClubId<T:Config> = StorageValue<_, u64,ValueQuery>;

	
	/*AS THERE CAN BE MULTIPLE CLUB OWNED BY THE SAME OWNER THIS MAPPING WILL HELP TO TRACK OWNERSHIP WITH ClubID*/
	/// Club Storage Mapping Between Owner->ClubId => Club
	#[pallet::storage]
	pub type Clubs<T:Config> = StorageDoubleMap<_, Blake2_128Concat, AccountIdOf<T>, Twox64Concat, ClubId, ClubsOF<T>, ValueQuery>; 

	/*THIS MAPPING WILL HELP TO EASILY RENEW THE MEMBESHIP IF THE MENERSHIP PERIOD IS EXPIRED */
	/// Expiration Period Storage Mapping of T::BlockNumber(ExpirationTime) -> Vec((AccountIdOf<T>[Member(will pay for renewal as well))], ClubId, AccountIdOf<T>[ClubOwner(to be paid to)])
	#[pallet::storage]
	pub type ExpiredOn<T:Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<(AccountIdOf<T>,ClubId,AccountIdOf<T>)>, ValueQuery>;


	/*THIS MAPPING WILL HELP TO TRACK THE MEMBERSHIP DATA FOR SPECIFIC CLUB */
	/// MemberShip Storage Mappin of (Member,ClubId) => MemberShipData
	#[pallet::storage]
	pub type MemberShip<T:Config> = StorageMap<_, Blake2_128Concat, (AccountIdOf<T>,ClubId), MemberShipDataOf<T>, OptionQuery>; 



	
	///  to check if at present block the memnership of some memberfrom a specific Club is expired or not
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		
		fn on_initialize(n: T::BlockNumber) -> Weight {
		// trigger the membership_expire and return the weight calculated for it else return weight zero
			if n > T::BlockNumber::zero() {
				
				_= Self::membership_expired(n);
				<T as pallet::Config>::WeightInfo::on_initialize()
			}else {

				Weight::zero()
			}
		}
	}



	#[pallet::call]
	impl<T:Config> Pallet <T> {


	#[pallet::weight(<T as pallet::Config>::WeightInfo::create_club())]
	/// Root can create the Club
	pub fn create_club(origin:OriginFor<T>,owner:AccountIdLookupOf<T>,initial_annual_expences:BalanceOf<T>) -> DispatchResult {

	//root can create the club
		ensure_root(origin)?;


	let owner = T::Lookup::lookup(owner)?;
	let prev_club_id = Self::next_club_id();
	let club_id = prev_club_id.checked_add(1).unwrap(); 
	// update the NextClubId with New Club ID 
	NextClubId::<T>::put(club_id);
	ensure!(!Clubs::<T>::contains_key(&owner,club_id), Error::<T>::ClubAlreadyExists);

	// owner need to pay  to the palletId for creation of the club 
	T::Currency::transfer(
		&owner,
		&Self::fee_collector_id(),
		T::FeesToCreateClub::get(),
		ExistenceRequirement::AllowDeath
	)?;



	// getting the blocknumber when  club_created
	let club_creation_time = <frame_system::Pallet<T>>::block_number();

	// there will be no memebers in the club while creating the club
	let mut  club_members:ClubMembers<T> = Vec::new(); 

   // member have to pay based on  initial_annual_expences as long as owner has not set the new_annual_expence 
	let  mut my_club = ClubsOF::<T>{

		annual_expences:initial_annual_expences,
		created:club_creation_time,
		members:club_members

	}; 


	
	//inserting club on chain with the mapping as owner->id => Club
	Clubs::<T>::insert(owner.clone(),club_id, my_club);


	//Emiting event ClubCreated(owner, club_id) After Creating Club
	Self::deposit_event(Event::ClubCreated(owner, club_id,initial_annual_expences));

	Ok(())


	}


	#[pallet::weight(<T as pallet::Config>::WeightInfo::set_the_annual_expences())]
	/// Owner can set/ chnage the annual_expence of club
	pub	fn set_the_annual_expences(origin:OriginFor<T>, new_annual_expence:BalanceOf<T>,club_id:u64) -> DispatchResult {


		
        // owner can set annual expence
		let owner = ensure_signed(origin)?; 
		// check if the Club exists and mutate the annual_expence
		ensure!(Clubs::<T>::contains_key(&owner, club_id), Error::<T>::ClubDoesNotExistORWrongOwner); 
 		
		
		//Mutate the value under the given keys when the closure returns Ok.   
		Clubs::<T>::try_mutate(
			owner.clone(), 
			club_id,
			|maybe_club| -> DispatchResult {
				
				//setting up the new annual_expence
				maybe_club.annual_expences = new_annual_expence; 

				Self::deposit_event(Event::AnnualExpenceChanged(club_id,new_annual_expence));
				Ok(())
			}
		)
	}

	#[pallet::weight(<T as pallet::Config>::WeightInfo::transfer_ownership())]
	/// Ownership can Be Transfered
	pub	fn transfer_ownership(origin:OriginFor<T>, new_owner:AccountIdLookupOf<T>,fees:BalanceOf<T>,club_id:u64) -> DispatchResult {
       
		let old_owner = ensure_signed(origin)?; 
		let new_owner = T::Lookup::lookup(new_owner)?;

		// check if the club exists 
		ensure!(Clubs::<T>::contains_key(&old_owner, &club_id), Error::<T>::ClubDoesNotExistORWrongOwner);
		
		// new_owner will pay the fees for ownership to the old_owner
        T::Currency::transfer(
			&new_owner,
			&old_owner,
			fees,
			ExistenceRequirement::AllowDeath
		)?;

		// take the club 
		let club = Clubs::<T>::take(&old_owner,club_id);
		// change the owner of club i.e insert it with new_owner
		Clubs::<T>::insert(new_owner.clone(),club_id,club) ; 

		//emit event for tranfered_ownership
		/*
		Not Exposing the Old_Owner and the New_Owner in the emiting event 
		as they are Crucial Keys 

		*/
		Self::deposit_event(Event::<T>::OwnershipOfClubTransfered( club_id));

		Ok(())
	
	}


	#[pallet::weight(<T as pallet::Config>::WeightInfo::add_members())]
	/// Club Owner can add member to their club
	pub fn add_members(owner:OriginFor<T>, wanna_be_member:AccountIdLookupOf<T>, club_id:u64, membership_period_in_years:u32) -> DispatchResult {


		let owner = ensure_signed(owner) ?; 
        let wanna_be_member = T::Lookup::lookup(wanna_be_member)?;
		// check if the club exists
		ensure!(Clubs::<T>::contains_key(&owner, club_id), Error::<T>::ClubDoesNotExistORWrongOwner); 

		// check if the Member is Already a part of the same Club
		ensure!(!MemberShip::<T>::contains_key(&(wanna_be_member.clone(),club_id)), Error::<T>::MemberAlreadyExist);

		// Check if the MemberShip Period does not cross the max MemberShip Limit
		let  years:BlockNumberOf<T> = membership_period_in_years.into(); 

		/*
			for e.g. 
			membership_period_in_years:u32 = 4 ; 
			years:BlockNumberOf<T> = 4 as well

			the block_number_years is then the actual no. of years in terms of BlockNumber after multiplying
			it with T::UnitMemberShipPeriod::get() which is a 1 year in terms of block number as set up in Runtime

		*/
		let block_number_years = years*T::UnitMembeShipPeriod::get();
		ensure!(block_number_years >= T::UnitMembeShipPeriod::get() && block_number_years <= T::MaxMembershipPeriod::get(), Error::<T>::MemeberShipPeriodExceeds );
	
       //Mutate the value under the given keys when the closure returns Ok.            
		Clubs::<T>::try_mutate(
			owner.clone(),
			club_id,
			|maybe_club| -> DispatchResult {


				/*
				AS NO MECHANISM FOR FEE CALCULATION SPECIFIED IN PROBLEM STATEMENT USING THIS SIMPLE FORMULA FOR THE SAME
				AS CALCULATING THE FEES BASED ON THE ACTUAL BLOCK NUMBER 
				CAN RESULT IN OVERFLOW AND 	
				So, using the Simple Formul for calculating the fees of membership
				total_fees = Time * Fees_For_Unit_Time
				THIS LOGIC CAN BE CHANGED WITH FEW MORE SPECIFICATIONS IN PROBLEM STATEMENT
				*/ 

				let calculated_fees = maybe_club.annual_expences *membership_period_in_years.into();

				T::Currency::transfer(
					&wanna_be_member,
					&owner,
					calculated_fees,
					ExistenceRequirement::AllowDeath
				)?;

				let now = <frame_system::Pallet<T>>::block_number();

				let membership_expiration_time = now+ block_number_years; 


				let key = (wanna_be_member.clone(),club_id);
				let membership_data = MemberShipDataOf::<T>{
							
					expired_on:membership_expiration_time,
					membership_period_in_years

				};
				// update Membership First
				Self::set_membership_data(key, membership_data);

				//  update the ExpiredOn
				let mut expired_on_data:Vec<(AccountIdOf<T>,ClubId,AccountIdOf<T>)>= Vec::new(); 
				let value =(wanna_be_member.clone(),club_id,owner);
				expired_on_data.push(value);
				ExpiredOn::<T>::insert(membership_expiration_time.clone(),expired_on_data);

				// update the Club 
				maybe_club.members.push(wanna_be_member.clone()); 

				// emit the event for New Member Added 
				Self::deposit_event(Event::<T>::MemberAdded(wanna_be_member,club_id,membership_expiration_time));

				Ok(())
			}
		)

	}


	#[pallet::weight(<T as pallet::Config>::WeightInfo::update_membership_period())]
	/// member can update their membership period(as how many years) in specific club_id
	pub fn update_membership_period(member:OriginFor<T>, club_id:u64,new_membership_period_in_years:u32) -> DispatchResult {

			let member = ensure_signed(member)?; 

			// check if the membership exists
			let key = (member.clone(),club_id);

			ensure!(MemberShip::<T>::contains_key(&key), Error::<T>::MemberShipDoesNotExist);

			// using try_mutate exists for additional secuirty and mutating the data


			MemberShip::<T>::try_mutate(
				key.clone(),
				|maybe_membership_data| -> DispatchResult {

					let   membership_data  = maybe_membership_data.as_mut().ok_or(Error::<T>::MemberShipDoesNotExist)?;

					// update the new expiration period 
					membership_data.membership_period_in_years = new_membership_period_in_years;

					Self::deposit_event(Event::<T>::MembershipPeriodUpdated(key.0,key.1));



					Ok(())
				}
			)


		
	}



	#[pallet::weight(<T as pallet::Config>::WeightInfo::cancel_membership())]
	/// when member wants to cancel the membership
	pub fn cancel_membership(owner:OriginFor<T>, member:AccountIdLookupOf<T>,club_id:u64) -> DispatchResult {

		let owner = ensure_signed(owner)?;
		let member = T::Lookup::lookup(member)?;

		// check if the club exists
		ensure!(Clubs::<T>::contains_key(&owner, club_id), Error::<T>::ClubDoesNotExistORWrongOwner); 

		// check if membership exists 
		ensure!(MemberShip::<T>::contains_key(&(member.clone(),club_id)), Error::<T>::MemberShipDoesNotExist);
		let membership_data = MemberShip::<T>::take(&(member.clone(),club_id)).unwrap();

		let expired_on = membership_data.expired_on; 
        let value = (member.clone(),club_id,owner.clone()) ;
		// remove from expiredOn
	_=	ExpiredOn::<T>::try_mutate(
			expired_on,
			|member_data| -> DispatchResult {

				ensure!(member_data.contains(&value),Error::<T>::NotAExpirationTime);

				let index = member_data.binary_search(&value).unwrap(); 
				member_data.remove(index);




				Ok(())

			}

		);

      // remove from clubs
	_= 	Clubs::<T>::try_mutate(
			owner.clone(),
			club_id,
			|club| -> DispatchResult {

				ensure!(club.members.contains(&member),Error::<T>::MemberShipDoesNotExist);

					 let index = club.members.binary_search(&member).unwrap();

					 club.members.remove(index);

				Ok(())

			}

		);

		// emit the event
		Self::deposit_event(Event::<T>::MemebrShipExpired(member,club_id));

		Ok(())

		


		
	}


	}

	impl <T:Config> Pallet<T> {

		/// convert the pallet id to account id
		pub fn fee_collector_id() -> T::AccountId {
			T::FeeCollector::get().into_account_truncating()
		}
		/// read clubs if exists
		pub fn read_clubs(owner:T::AccountId, club_id:u64)  -> Result<ClubsOF<T>, Error<T>>
		{	
				if Clubs::<T>::contains_key(&owner,club_id) {

					Ok(Clubs::<T>::get(&owner,club_id))
				}else {
					Err(Error::<T>::ClubDoesNotExistORWrongOwner)
				}
		}

		///	membership_expired will get triggered for blocknumbers with the help of on_initialize		
	 	pub fn membership_expired(block_number:T::BlockNumber) -> DispatchResult {

		// check if the expire_on period exists 	
		ensure!(!ExpiredOn::<T>::try_get(&block_number).is_err(), Error::<T>::NotAExpirationTime);
		 
			for(member_id,club_id,club_owner) in ExpiredOn::<T>::take(block_number){

				let key = (member_id.clone(),club_id); 

				/*
				  additional check if the membership exists 
				  this will ensure the that the club evantually exists 
				  and will save the further reading from Clubs
				*/
				
				ensure!(MemberShip::<T>::contains_key(&key), Error::<T>::MemberShipDoesNotExist);

				
				if let Some( membership_data) = Self::get_membership_data((member_id.clone(),club_id)){
					if membership_data.expired_on == block_number {

						let value = (member_id, club_id,club_owner);
						/*BY DEFAULT THE MEMBERSHIP WILL GET RENEW AFTER EXPIRED
						  BASED TO THE MEMBERSHIP PERIOD(in terms of years) MEMBER SETS THE NEW EXPIRATION
						  PERIOD WILL BE CALCULATED 	
						*/
						 _= Self::renew_membership(value, membership_data.membership_period_in_years);

					}

				}

			}

			Ok(())

		}




		/// Must Be Called after the check that key exists In Membership
		fn get_membership_data(key: (AccountIdOf<T>, u64)) -> Option< MemberShipDataOf<T>> {

			
			MemberShip::<T>::try_get(key).ok()

		}

		/// Must Be Called after the check that key does not exists In Membership
		fn set_membership_data(key: (AccountIdOf<T>, u64),membership_data:MemberShipDataOf<T>) {

			MemberShip::<T>::insert(key,membership_data);

		}
		/// read membership if exists
		pub	fn read_membership(key: (AccountIdOf<T>, u64)) -> Result<MemberShipDataOf<T>, Error<T>> {


			if MemberShip::<T>::contains_key(&key) {
				Ok(MemberShip::<T>::get(&key).unwrap())
			}else {
				Err(Error::<T>::MemberShipDoesNotExist)
			}

		}
		/// read expired on if exists
		pub fn read_expired_on(block_number:T::BlockNumber) -> Result<Vec<(AccountIdOf<T>,ClubId,AccountIdOf<T>)>, Error<T> > {

            
		if ExpiredOn::<T>::contains_key(&block_number){

					Ok(ExpiredOn::<T>::get(&block_number))

		}else {

			Err(Error::<T>::NotAExpirationTime)

		}


		}
		/// renew membership will get triggered if the members membership is expired
		fn renew_membership(member:(AccountIdOf<T>, ClubId,AccountIdOf<T>),membership_period_in_year:u32) -> DispatchResult  {
			let  years:BlockNumberOf<T> = membership_period_in_year.into(); 
			/*
				As the default renewal period will be same as membership_period registered 
				at first
			*/
			let block_number_years = years*T::UnitMembeShipPeriod::get();
			let now = frame_system::pallet::Pallet::<T>::block_number();
            let new_expire_on = now + block_number_years;
			
			// Member need to pay to the Owner for renewing the membership 

			let club_owner = &member.2;
            let member_id = &member.0; 
			let club_id = &member.1;

			let club = Clubs::<T>::try_get(&club_owner,club_id).unwrap(); 
			let annual_fees = club.annual_expences; 

			
			/*
				calculating fees based on default renewal period 
				AGAIN WITH THE SAME LOGIC AND FORMULA AS STATED IN THE add_member
			*/
			let calculated_fees = annual_fees *membership_period_in_year.into(); 


              
			// member needs to pay to the owner for renewal 

				T::Currency::transfer(
					&member_id,
					&club_owner,
					calculated_fees,
					ExistenceRequirement::AllowDeath
				)?;

			

			// update the Expired_on 	
			ExpiredOn::<T>::append(new_expire_on, member.clone());
			// mutate Membership for new expire period of member	
			Self::mutate_membership((member.0,member.1), new_expire_on)


		}


		/// check the balance of fee collector
		pub fn check_balance_of_fee_collector() -> BalanceOf<T> {
			let fee_collector= Self::fee_collector_id();
			T::Currency::total_balance(&fee_collector)	
				
		}

		/// check the balance of who
	    pub fn check_balance(who:&AccountIdOf<T>) -> BalanceOf<T> {
			T::Currency::total_balance(&who)	
		}
		/// will get trigered for the renewal for membership after the membership has beem expired 
		fn mutate_membership(key:(AccountIdOf<T>, ClubId), new_expiration_period:T::BlockNumber) -> DispatchResult {

			MemberShip::<T>::try_mutate(
				key.clone(),
				|maybe_membership_data| -> DispatchResult {

					// let mut membership_data = maybe_membership_data.take().ok_or(Error::<T>::MemberShipDoesNotExist)?;
                         
					let   membership_data  = maybe_membership_data.as_mut().ok_or(Error::<T>::MemberShipDoesNotExist)?;

					// update the new expiration period 
					 membership_data.expired_on = new_expiration_period;

					// emiting the event fot renewal of membership
					Self::deposit_event(Event::<T>::MemberShipRenewed(key.0,key.1));



					Ok(())
				}
			)

		}
	}

}
