## pallet clubs
  ## clone the repo
    https git@github.com:andropixels/ClubsOnChain.git
    ssh  https://github.com/andropixels/ClubsOnChain.git

### test the pallet-clubs
   cargo test -p pallet-clubs 
### build the pallet
   cargo build -p pallet-clubs
### build the node
   cargo build --release

### Asumptions Based on given problem statement
    1-Root can create the club
    2-owner needs to pay some token to FeeCollector for creating the club
    3-There can be multiple clubs and multiple clubs can have multiple members
    4-Owner can add members to the club
        A-member needs to pay some token based on the annual expences of club and their membership period to the owner based on their membership period
        B- member can set/change the membership period in a club
        C- membership will get expired and by default will get renewd unitl the member cancels memnership
        D- The MaximumMembership Period is 100 years
    5-Club Ownerhsip can be transfered for that new owner will pay to the oldOwner
    6-Club Owner can set/Change the Annual Expences of club    

    

           
