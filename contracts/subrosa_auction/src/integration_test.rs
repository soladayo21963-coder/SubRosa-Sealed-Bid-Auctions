#![cfg(test)]

use soroban_sdk::{
    testutils::{Address as _, Ledger},
    Address, BytesN, Env,
};

use crate::{SubRosaAuctionContract, SubRosaAuctionContractClient};

#[test]
fn integration_reveal_and_highest_bid_tracking() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let seller = Address::generate(&env);
    let bidder_a = Address::generate(&env);
    let bidder_b = Address::generate(&env);

    let token_admin = Address::generate(&env);
    let token_contract = env.register_stellar_asset_contract_v2(token_admin.clone());
    let token = token_contract.address();
    let token_client = soroban_sdk::token::StellarAssetClient::new(&env, &token);
    token_client.mint(&bidder_a, &1_000_000_000);
    token_client.mint(&bidder_b, &1_000_000_000);

    let contract_id = env.register(SubRosaAuctionContract, ());
    let client = SubRosaAuctionContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token);

    env.ledger().with_mut(|li| li.timestamp = 100);
    let auction_id = client.create_auction(&seller, &token, &1_000, &2_000);
    assert_eq!(auction_id, 1);

    let salt_a = BytesN::from_array(&env, &[1u8; 32]);
    let salt_b = BytesN::from_array(&env, &[2u8; 32]);
    let commitment_a = BytesN::from_array(&env, &[10u8; 32]);
    let commitment_b = BytesN::from_array(&env, &[11u8; 32]);

    env.ledger().with_mut(|li| li.timestamp = 250);
    client.submit_sealed_bid(&bidder_a, &auction_id, &commitment_a, &800_000);
    client.submit_sealed_bid(&bidder_b, &auction_id, &commitment_b, &900_000);

    env.ledger().with_mut(|li| li.timestamp = 1_500);
    client.reveal_bid(&bidder_a, &auction_id, &750_000, &salt_a);
    client.reveal_bid(&bidder_b, &auction_id, &950_000, &salt_b);

    let auction = client.get_auction(&auction_id);
    assert_eq!(auction.highest_bid, 950_000);
    assert_eq!(auction.highest_bidder, Some(bidder_b));
}
