#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    BytesN, Env,
};

#[test]
fn test_sealed_bid_auction_lifecycle() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let bidder = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token_address = token_contract.address();
    let sac_client = token::StellarAssetClient::new(&env, &token_address);
    sac_client.mint(&bidder, &100_000_000);

    let contract_id = env.register(SubRosaAuctionContract, ());
    let client = SubRosaAuctionContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_address);

    env.ledger().with_mut(|li| li.timestamp = 100);
    let auction_id = client.create_auction(&seller, &token_address, &1000, &2000);
    assert_eq!(auction_id, 1);

    let salt = BytesN::from_array(&env, &[5u8; 32]);
    let commitment_hash = BytesN::from_array(&env, &[9u8; 32]);

    env.ledger().with_mut(|li| li.timestamp = 500);
    client.submit_sealed_bid(&bidder, &auction_id, &commitment_hash, &80_000_000);

    env.ledger().with_mut(|li| li.timestamp = 1500);
    client.reveal_bid(&bidder, &auction_id, &75_000_000, &salt);

    let auction: Auction = env
        .storage()
        .persistent()
        .get(&DataKey::Auction(auction_id))
        .unwrap();

    assert_eq!(auction.highest_bid, 75_000_000);
    assert_eq!(auction.highest_bidder, Some(bidder));
}
