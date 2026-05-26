#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, token, vec};

fn setup_test() -> (Env, DisasterReliefDaoClient, Address, Address, Address, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();
    
    let contract_id = env.register_contract(None, DisasterReliefDao);
    let client = DisasterReliefDaoClient::new(&env, &contract_id);
    
    let signer1 = Address::generate(&env);
    let signer2 = Address::generate(&env);
    let signer3 = Address::generate(&env);
    
    let donor = Address::generate(&env);
    let merchant = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract(token_admin.clone());
    let token_admin_client = token::StellarAssetClient::new(&env, &token_id);
    
    token_admin_client.mint(&donor, &1000);
    
    client.init(&vec![&env, signer1.clone(), signer2.clone(), signer3.clone()]);
    
    (env, client, signer1, signer2, signer3, donor, merchant, token_id)
}

// Test 1 (Happy path): 2 signers approve, funds disbursed
#[test]
fn test_happy_path_end_to_end() {
    let (env, client, signer1, signer2, _signer3, donor, merchant, token) = setup_test();
    let token_client = token::Client::new(&env, &token);
    
    client.donate(&donor, &token, &500);
    
    client.approve_disbursement(&signer1, &1, &merchant, &token, &300);
    assert_eq!(token_client.balance(&merchant), 0); // 1 approval, not disbursed yet
    
    client.approve_disbursement(&signer2, &1, &merchant, &token, &300);
    assert_eq!(token_client.balance(&merchant), 300); // 2 approvals, disbursed!
}

// Test 2 (Edge case): Unauthorized signer attempts approval
#[test]
#[should_panic(expected = "Unauthorized signer")]
fn test_edge_case_unauthorized_signer() {
    let (env, client, _signer1, _signer2, _signer3, _donor, merchant, token) = setup_test();
    let imposter = Address::generate(&env);
    
    client.approve_disbursement(&imposter, &1, &merchant, &token, &300);
}

// Test 3 (State verification): Verify pool balance
#[test]
fn test_state_verification() {
    let (env, client, signer1, signer2, _signer3, donor, merchant, token) = setup_test();
    
    client.donate(&donor, &token, &500);
    assert_eq!(client.get_pool_balance(), 500);
    
    client.approve_disbursement(&signer1, &1, &merchant, &token, &300);
    assert_eq!(client.get_pool_balance(), 500);
    
    client.approve_disbursement(&signer2, &1, &merchant, &token, &300);
    assert_eq!(client.get_pool_balance(), 200); // 500 - 300 = 200
}

// Test 4: 1 approval does not disburse
#[test]
fn test_single_approval_no_disbursement() {
    let (env, client, signer1, _signer2, _signer3, donor, merchant, token) = setup_test();
    let token_client = token::Client::new(&env, &token);
    
    client.donate(&donor, &token, &500);
    client.approve_disbursement(&signer1, &1, &merchant, &token, &300);
    
    assert_eq!(token_client.balance(&merchant), 0);
}

// Test 5: Cannot execute already executed disbursement
#[test]
#[should_panic(expected = "Disbursement already executed")]
fn test_cannot_execute_twice() {
    let (env, client, signer1, signer2, signer3, donor, merchant, token) = setup_test();
    
    client.donate(&donor, &token, &500);
    client.approve_disbursement(&signer1, &1, &merchant, &token, &300);
    client.approve_disbursement(&signer2, &1, &merchant, &token, &300);
    
    // 3rd signer tries to approve an already executed disbursement
    client.approve_disbursement(&signer3, &1, &merchant, &token, &300);
}
