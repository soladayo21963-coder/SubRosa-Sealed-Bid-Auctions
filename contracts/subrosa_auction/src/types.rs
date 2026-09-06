use soroban_sdk::{contracttype, Address, BytesN};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    Admin,
    Token,
    Auction(u64),
    Bid(u64, Address),
    NextAuctionId,
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuctionStatus {
    Bidding,
    Reveal,
    Settled,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Auction {
    pub id: u64,
    pub seller: Address,
    pub asset_token: Address,
    pub bid_deadline: u64,
    pub reveal_deadline: u64,
    pub highest_bidder: Option<Address>,
    pub highest_bid: i128,
    pub status: AuctionStatus,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SealedBid {
    pub bidder: Address,
    pub commitment_hash: BytesN<32>,
    pub collateral_locked: i128,
    pub is_revealed: bool,
}

impl Default for AuctionStatus {
    fn default() -> Self {
        AuctionStatus::Bidding
    }
}
