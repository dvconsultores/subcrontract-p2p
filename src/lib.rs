use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};

const INITIAL_BALANCE: Balance = 3_800_000_000_000_000_000_000_000; // 1e24yN, 3.8N
const CODE: &[u8] = include_bytes!("./wasm/rust_changeback.wasm");

#[near_bindgen]
pub struct Contract {}

#[near_bindgen]
impl Contract {
    //#[private]
    #[payable]
    pub fn create_child_contract(prefix: AccountId) -> Promise {
        let subaccount_id = AccountId::new_unchecked(
          format!("{}.{}", prefix, env::current_account_id())
        );
        Promise::new(subaccount_id)
            .create_account()
            //.add_full_access_key(env::signer_account_pk())
            .transfer(INITIAL_BALANCE)
            .deploy_contract(CODE.to_vec())
    }
}