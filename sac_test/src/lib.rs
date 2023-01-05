#![no_std]

use soroban_sdk::{contractimpl, BytesN, Env};
use token::Signature;

mod token {
    soroban_sdk::contractimport!(file = "../soroban_token_spec.wasm");
}

use token::{Identifier};

pub struct SACTest;

#[contractimpl]
impl SACTest {
    pub fn burn_self(env: Env, token: BytesN<32>, nonce: i128, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.burn(&Signature::Invoker, &nonce, &amount);
    }

    pub fn xfer(env: Env, token: BytesN<32>, to: Identifier, amount: i128) {
        let client = token::Client::new(&env, &token);
        client.xfer(&Signature::Invoker, &0, &to, &amount);
    }
}
