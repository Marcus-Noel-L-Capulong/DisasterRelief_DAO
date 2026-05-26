#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, token, Vec};

#[contract]
pub struct DisasterReliefDao;

#[contracttype]
pub enum DataKey {
    PoolBalance,
    Signers, // Vec of authorized signers
    Approvals(u64), // Disbursement ID to number of approvals
    Executed(u64), // Disbursement ID execution status
}

#[contractimpl]
impl DisasterReliefDao {
    /// Initializes the DAO with 3 designated signers (e.g., NGO heads).
    pub fn init(env: Env, signers: Vec<Address>) {
        assert!(!env.storage().instance().has(&DataKey::Signers), "Already initialized");
        env.storage().instance().set(&DataKey::Signers, &signers);
    }

    /// Donors contribute USDC to the relief pool.
    pub fn donate(env: Env, donor: Address, token: Address, amount: i128) {
        donor.require_auth();
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&donor, &env.current_contract_address(), &amount);
        
        let mut pool: i128 = env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0);
        pool += amount;
        env.storage().instance().set(&DataKey::PoolBalance, &pool);
    }

    /// A signer approves a specific disbursement ID to a merchant. 
    /// If 2 approvals are reached, the funds are disbursed automatically.
    pub fn approve_disbursement(
        env: Env, 
        signer: Address, 
        disbursement_id: u64, 
        merchant: Address, 
        token: Address, 
        amount: i128
    ) {
        signer.require_auth();
        
        let signers: Vec<Address> = env.storage().instance().get(&DataKey::Signers).expect("Not initialized");
        assert!(signers.contains(signer.clone()), "Unauthorized signer");
        
        let executed: bool = env.storage().instance().get(&DataKey::Executed(disbursement_id)).unwrap_or(false);
        assert!(!executed, "Disbursement already executed");

        let mut approvals: u32 = env.storage().instance().get(&DataKey::Approvals(disbursement_id)).unwrap_or(0);
        approvals += 1; 
        
        if approvals >= 2 {
            let mut pool: i128 = env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0);
            assert!(pool >= amount, "Insufficient pool funds");
            
            pool -= amount;
            env.storage().instance().set(&DataKey::PoolBalance, &pool);
            env.storage().instance().set(&DataKey::Executed(disbursement_id), &true);
            
            let token_client = token::Client::new(&env, &token);
            token_client.transfer(&env.current_contract_address(), &merchant, &amount);
        } else {
            env.storage().instance().set(&DataKey::Approvals(disbursement_id), &approvals);
        }
    }
    
    pub fn get_pool_balance(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::PoolBalance).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests;
