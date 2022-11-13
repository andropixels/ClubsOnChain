## pallet clubs
  ## clone the repo
    https git@github.com:andropixels/ClubsOnChain.git
    ssh  https://github.com/andropixels/ClubsOnChain.git

### Test the pallet-clubs
     cargo test -p pallet-clubs 

### Build the pallet
   cargo build -p pallet-clubs

### Build the node
   cargo build --release

### Asumptions based on given problem statement
    1-Root can create the club
    2-owner needs to pay some token to FeeCollector for creating the club
    3-There can be multiple clubs and multiple clubs can have multiple members
    4-Owner can add members to the club
        A-member needs to pay some token to the owner based on the annual expences of club and their membership period  
        B- member can set/change the membership period in a club
        C- membership will get expired and by default will get renewd unitl the member cancel's membership
        D- The MaximumMembership Period is 100 years
    5-Club Ownerhship can be transfered for that new owner will pay to the oldOwner
    6-Club Owner can set/Change the Annual Expences of club    



           
