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
    token_client.mint(&seller, &1);
    token_client.mint(&bidder_a, &1_000_000_000);
    token_client.mint(&bidder_b, &1_000_000_000);

    let contract_id = env.register(SubRosaAuctionContract, ());
    let client = SubRosaAuctionContractClient::new(&env, &contract_id);
    client.initialize(&admin, &token);

    env.ledger().with_mut(|li| li.timestamp = 100);
    let auction_id = client.create_auction(&seller, &token, &1, &1_000, &2_000);
    assert_eq!(auction_id, 1);

    let salt_a = BytesN::from_array(&env, &[1u8; 32]);
    let salt_b = BytesN::from_array(&env, &[2u8; 32]);
    let mut payload_a = [0u8; 56];
    payload_a[..16].copy_from_slice(&750_000i128.to_be_bytes());
    payload_a[16..48].copy_from_slice(&salt_a.to_array());
    payload_a[48..].copy_from_slice(&auction_id.to_be_bytes());
    let commitment_a = BytesN::from_array(
        &env,
        &env.crypto()
            .sha256(&soroban_sdk::Bytes::from_array(&env, &payload_a))
            .to_array(),
    );

    let mut payload_b = [0u8; 56];
    payload_b[..16].copy_from_slice(&950_000i128.to_be_bytes());
    payload_b[16..48].copy_from_slice(&salt_b.to_array());
    payload_b[48..].copy_from_slice(&auction_id.to_be_bytes());
    let commitment_b = BytesN::from_array(
        &env,
        &env.crypto()
            .sha256(&soroban_sdk::Bytes::from_array(&env, &payload_b))
            .to_array(),
    );

    env.ledger().with_mut(|li| li.timestamp = 250);
    client.submit_sealed_bid(&bidder_a, &auction_id, &commitment_a, &800_000);
    client.submit_sealed_bid(&bidder_b, &auction_id, &commitment_b, &1_000_000);

    env.ledger().with_mut(|li| li.timestamp = 1_500);
    client.reveal_bid(&bidder_a, &auction_id, &750_000, &salt_a);
    client.reveal_bid(&bidder_b, &auction_id, &950_000, &salt_b);

    let auction = client.get_auction(&auction_id);
    assert_eq!(auction.highest_bid, 950_000);
    assert_eq!(auction.highest_bidder, Some(bidder_b));

    env.ledger().with_mut(|li| li.timestamp = 2_000);
    client.finalize_auction(&auction_id);
    client.claim_refund(&auction_id, &bidder_a);

    let settled = client.get_auction(&auction_id);
    assert_eq!(settled.status, crate::types::AuctionStatus::Settled);
    assert_eq!(token_client.balance(&bidder_a), 1_000_000_000);
}
