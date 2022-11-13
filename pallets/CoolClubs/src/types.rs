
use codec::{Encode, Decode};
use scale_info::TypeInfo;

//ClubID 
pub type ClubId = u64; 

#[derive(Encode, Decode, Default, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct Club<Expences,ListOfMembers,BlockNumber> {
   
   //annual_expences set by the owner
   pub(crate)  annual_expences:Expences,
   //When Club was Created
   pub(crate) created:BlockNumber,
   /// there can be multiple members in a signle club
   pub (crate) members:ListOfMembers
}

#[derive(Encode, Decode, Default, PartialEq, Eq,TypeInfo)]
#[cfg_attr(feature = "std", derive(Debug))]
pub struct MemberShipData<BlockNumber> {

    // membership expired on 
    pub(crate) expired_on:BlockNumber,
    // membership period in years  storing it  as member may wants to change it further
    pub membership_period_in_years:u32
}

