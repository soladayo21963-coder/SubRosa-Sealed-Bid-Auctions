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
    sac_client.mint(&seller, &1);
    sac_client.mint(&bidder, &100_000_000);

    let contract_id = env.register(SubRosaAuctionContract, ());
    let client = SubRosaAuctionContractClient::new(&env, &contract_id);

    client.initialize(&admin, &token_address);

    env.ledger().with_mut(|li| li.timestamp = 100);
    let auction_id = client.create_auction(&seller, &token_address, &1, &1000, &2000);
    assert_eq!(auction_id, 1);

    let salt = BytesN::from_array(&env, &[5u8; 32]);
    let mut payload = [0u8; 56];
    payload[..16].copy_from_slice(&75_000_000i128.to_be_bytes());
    payload[16..48].copy_from_slice(&salt.to_array());
    payload[48..].copy_from_slice(&auction_id.to_be_bytes());
    let commitment_hash = BytesN::from_array(
        &env,
        &env.crypto()
            .sha256(&soroban_sdk::Bytes::from_array(&env, &payload))
            .to_array(),
    );

    env.ledger().with_mut(|li| li.timestamp = 500);
    client.submit_sealed_bid(&bidder, &auction_id, &commitment_hash, &80_000_000);

    env.ledger().with_mut(|li| li.timestamp = 1500);
    client.reveal_bid(&bidder, &auction_id, &75_000_000, &salt);

    let auction = client.get_auction(&auction_id);

    assert_eq!(auction.highest_bid, 75_000_000);
    assert_eq!(auction.highest_bidder, Some(bidder));
}

#[test]
#[should_panic]
fn cannot_create_auction_before_initialization() {
    let env = Env::default();
    env.mock_all_auths();

    let seller = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin);
    let contract_id = env.register(SubRosaAuctionContract, ());
    let client = SubRosaAuctionContractClient::new(&env, &contract_id);

    client.create_auction(&seller, &token_contract.address(), &1, &10, &20);
}
