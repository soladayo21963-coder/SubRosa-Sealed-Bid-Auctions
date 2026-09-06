#![no_std]

use soroban_sdk::{
    contract, contracterror, contractimpl, token, Address, BytesN, Env,
};

mod types;
use types::*;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotInitialized = 1,
    AlreadyInitialized = 2,
    Unauthorized = 3,
    InvalidAmount = 4,
    InvalidTimeRange = 5,
    AuctionNotFound = 6,
    BiddingClosed = 7,
    RevealNotOpen = 8,
    RevealClosed = 9,
    InvalidCommitment = 10,
    BidAlreadyRevealed = 11,
    ContractPaused = 12,
    MissingBid = 13,
    InvalidBidder = 14,
    AlreadySettled = 15,
    SettlementNotOpen = 16,
    CannotRefundWinner = 17,
    AlreadyRefunded = 18,
}

const THIRTY_DAYS_IN_LEDGERS: u32 = 518_400;

#[contract]
pub struct SubRosaAuctionContract;

#[contractimpl]
impl SubRosaAuctionContract {
    pub fn initialize(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(Error::AlreadyInitialized);
        }

        admin.require_auth();

        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::NextAuctionId, &1u64);
        env.storage().instance().set(&DataKey::Paused, &false);
        env.storage().instance().extend_ttl(THIRTY_DAYS_IN_LEDGERS, THIRTY_DAYS_IN_LEDGERS);

        Ok(())
    }

    pub fn create_auction(
        env: Env,
        seller: Address,
        asset_token: Address,
        asset_amount: i128,
        bid_deadline: u64,
        reveal_deadline: u64,
    ) -> Result<u64, Error> {
        Self::ensure_not_paused(&env)?;
        seller.require_auth();

        let now = env.ledger().timestamp();
        if asset_amount <= 0 || bid_deadline <= now || reveal_deadline <= bid_deadline {
            return Err(Error::InvalidTimeRange);
        }

        let asset_client = token::Client::new(&env, &asset_token);
        asset_client.transfer(&seller, &env.current_contract_address(), &asset_amount);

        let auction_id: u64 = env
            .storage()
            .instance()
            .get(&DataKey::NextAuctionId)
            .unwrap_or(1u64);

        let auction = Auction {
            id: auction_id,
            seller,
            asset_token,
            asset_amount,
            bid_deadline,
            reveal_deadline,
            highest_bidder: None,
            highest_bid: 0,
            status: AuctionStatus::Bidding,
        };

        env.storage().persistent().set(&DataKey::Auction(auction_id), &auction);
        env.storage().persistent().extend_ttl(
            &DataKey::Auction(auction_id),
            THIRTY_DAYS_IN_LEDGERS,
            THIRTY_DAYS_IN_LEDGERS,
        );

        env.storage().instance().set(&DataKey::NextAuctionId, &(auction_id + 1));
        Ok(auction_id)
    }

    pub fn submit_sealed_bid(
        env: Env,
        bidder: Address,
        auction_id: u64,
        commitment_hash: BytesN<32>,
        collateral_amount: i128,
    ) -> Result<(), Error> {
        Self::ensure_not_paused(&env)?;
        bidder.require_auth();

        if collateral_amount <= 0 {
            return Err(Error::InvalidAmount);
        }

        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .ok_or(Error::AuctionNotFound)?;

        let now = env.ledger().timestamp();
        if now >= auction.bid_deadline {
            return Err(Error::BiddingClosed);
        }

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;

        let token_client = token::Client::new(&env, &token_addr);
        token_client.transfer(&bidder, &env.current_contract_address(), &collateral_amount);

        let bid = SealedBid {
            bidder: bidder.clone(),
            commitment_hash,
            collateral_locked: collateral_amount,
            is_revealed: false,
            is_refunded: false,
        };

        env.storage().persistent().set(&DataKey::Bid(auction_id, bidder.clone()), &bid);
        env.storage().persistent().extend_ttl(
            &DataKey::Bid(auction_id, bidder),
            THIRTY_DAYS_IN_LEDGERS,
            THIRTY_DAYS_IN_LEDGERS,
        );

        Ok(())
    }

    pub fn reveal_bid(
        env: Env,
        bidder: Address,
        auction_id: u64,
        secret_bid_amount: i128,
        salt: BytesN<32>,
    ) -> Result<(), Error> {
        Self::ensure_not_paused(&env)?;
        bidder.require_auth();

        let mut auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .ok_or(Error::AuctionNotFound)?;

        let now = env.ledger().timestamp();
        if now < auction.bid_deadline {
            return Err(Error::RevealNotOpen);
        }
        if now >= auction.reveal_deadline {
            return Err(Error::RevealClosed);
        }

        let mut bid: SealedBid = env
            .storage()
            .persistent()
            .get(&DataKey::Bid(auction_id, bidder.clone()))
            .ok_or(Error::MissingBid)?;

        if bid.bidder != bidder {
            return Err(Error::InvalidBidder);
        }

        if bid.is_revealed {
            return Err(Error::BidAlreadyRevealed);
        }

        if secret_bid_amount <= 0 || secret_bid_amount > bid.collateral_locked {
            return Err(Error::InvalidAmount);
        }

        if salt.len() != 32 {
            return Err(Error::InvalidCommitment);
        }

        let commitment_matches = Self::validate_commitment(&env, &bid.commitment_hash, &salt, secret_bid_amount);
        if !commitment_matches {
            return Err(Error::InvalidCommitment);
        }

        bid.is_revealed = true;
        env.storage().persistent().set(&DataKey::Bid(auction_id, bidder.clone()), &bid);

        if secret_bid_amount > auction.highest_bid {
            auction.highest_bid = secret_bid_amount;
            auction.highest_bidder = Some(bidder.clone());
            auction.status = AuctionStatus::Reveal;
            env.storage().persistent().set(&DataKey::Auction(auction_id), &auction);
        }

        Ok(())
    }

    pub fn get_auction(env: Env, auction_id: u64) -> Result<Auction, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .ok_or(Error::AuctionNotFound)
    }

    pub fn get_bid(env: Env, auction_id: u64, bidder: Address) -> Result<SealedBid, Error> {
        env.storage()
            .persistent()
            .get(&DataKey::Bid(auction_id, bidder))
            .ok_or(Error::MissingBid)
    }

    pub fn finalize_auction(env: Env, auction_id: u64) -> Result<(), Error> {
        Self::ensure_not_paused(&env)?;

        let mut auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .ok_or(Error::AuctionNotFound)?;

        if env.ledger().timestamp() < auction.reveal_deadline {
            return Err(Error::SettlementNotOpen);
        }
        if auction.status == AuctionStatus::Settled {
            return Err(Error::AlreadySettled);
        }

        let payment_token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        let payment_client = token::Client::new(&env, &payment_token);
        let asset_client = token::Client::new(&env, &auction.asset_token);

        if let Some(winner) = auction.highest_bidder.clone() {
            payment_client.transfer(
                &env.current_contract_address(),
                &auction.seller,
                &auction.highest_bid,
            );
            asset_client.transfer(
                &env.current_contract_address(),
                &winner,
                &auction.asset_amount,
            );
        } else {
            asset_client.transfer(
                &env.current_contract_address(),
                &auction.seller,
                &auction.asset_amount,
            );
        }

        auction.status = AuctionStatus::Settled;
        env.storage().persistent().set(&DataKey::Auction(auction_id), &auction);
        Ok(())
    }

    pub fn claim_refund(env: Env, auction_id: u64, bidder: Address) -> Result<(), Error> {
        Self::ensure_not_paused(&env)?;
        bidder.require_auth();

        let auction: Auction = env
            .storage()
            .persistent()
            .get(&DataKey::Auction(auction_id))
            .ok_or(Error::AuctionNotFound)?;
        if auction.status != AuctionStatus::Settled {
            return Err(Error::SettlementNotOpen);
        }
        if auction.highest_bidder == Some(bidder.clone()) {
            return Err(Error::CannotRefundWinner);
        }

        let mut bid: SealedBid = env
            .storage()
            .persistent()
            .get(&DataKey::Bid(auction_id, bidder.clone()))
            .ok_or(Error::MissingBid)?;
        if bid.is_refunded {
            return Err(Error::AlreadyRefunded);
        }

        let payment_token: Address = env
            .storage()
            .instance()
            .get(&DataKey::Token)
            .ok_or(Error::NotInitialized)?;
        token::Client::new(&env, &payment_token).transfer(
            &env.current_contract_address(),
            &bidder,
            &bid.collateral_locked,
        );
        bid.is_refunded = true;
        env.storage().persistent().set(&DataKey::Bid(auction_id, bidder), &bid);
        Ok(())
    }

    pub fn set_paused(env: Env, admin: Address, paused: bool) -> Result<(), Error> {
        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(Error::NotInitialized)?;

        if admin != stored_admin {
            return Err(Error::Unauthorized);
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Paused, &paused);
        Ok(())
    }

    fn ensure_not_paused(env: &Env) -> Result<(), Error> {
        let paused: bool = env
            .storage()
            .instance()
            .get(&DataKey::Paused)
            .unwrap_or(false);

        if paused {
            Err(Error::ContractPaused)
        } else {
            Ok(())
        }
    }

    fn validate_commitment(
        env: &Env,
        commitment_hash: &BytesN<32>,
        salt: &BytesN<32>,
        bid_amount: i128,
    ) -> bool {
        let mut payload = [0u8; 32 + 16];
        let amount_bytes = bid_amount.to_be_bytes();
        let salt_bytes = salt.to_array();

        let mut idx = 0;
        for b in amount_bytes {
            payload[idx] = b;
            idx += 1;
        }
        for b in salt_bytes {
            payload[idx] = b;
            idx += 1;
        }

        let payload_bytes = soroban_sdk::Bytes::from_array(&env, &payload);
        let computed_hash = env.crypto().sha256(&payload_bytes);
        let computed_bytes: [u8; 32] = computed_hash.to_array();
        let commitment_bytes: [u8; 32] = commitment_hash.to_array();

        computed_bytes == commitment_bytes
    }
}

#[cfg(test)]
mod test;

#[cfg(test)]
mod integration_test;
