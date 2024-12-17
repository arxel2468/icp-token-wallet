use candid::Principal;
use ic_cdk_macros::{query, update};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};

#[derive(Default)]
struct Wallet {
    balances: HashMap<String, u64>, // Maps addresses to balances
}

static WALLET: LazyLock<Mutex<Wallet>> = LazyLock::new(|| Mutex::new(Wallet::default()));

fn is_valid_address(address: &str) -> bool {
    !address.trim().is_empty() && address.chars().all(|c| c.is_alphanumeric())
}

fn is_authorized() -> bool {
    let caller = ic_cdk::caller();
    // Replace "authorized_principal" with the actual Principal string of authorized user
    let authorized_principal = Principal::from_text("bnz7o-iuaaa-aaaaa-qaaaa-cai").unwrap();
    caller == authorized_principal
}


/// Add tokens to a wallet (receive tokens)
#[update]
fn receive_tokens(address: String, amount: u64) -> Result<(), String> {
    if !is_valid_address(&address) {
        return Err("Invalid wallet address".to_string());
    }

    let mut wallet = WALLET.lock().unwrap();
    *wallet.balances.entry(address.clone()).or_insert(0) += amount;

    ic_cdk::println!("{} tokens received for address: {}", amount, address);
    Ok(())
}


/// Send tokens from one wallet to another
#[update]
fn send_tokens(from: String, to: String, amount: u64) -> Result<(), String> {
    if !is_valid_address(&from) || !is_valid_address(&to) {
        return Err("Invalid wallet address".to_string());
    }

    let mut wallet = WALLET.lock().unwrap();

    if let Some(balance) = wallet.balances.get_mut(&from) {
        if *balance >= amount {
            *balance -= amount;
            *wallet.balances.entry(to).or_insert(0) += amount;
            Ok(())
        } else {
            Err("Insufficient balance".to_string())
        }
    } else {
        Err("Sender wallet does not exist".to_string())
    }
}

/// Get the balance of a specific address
#[query]
fn get_balance(address: String) -> u64 {
    let wallet = WALLET.lock().unwrap();
    *wallet.balances.get(&address).unwrap_or(&0)
}


#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        let mut wallet = WALLET.lock().unwrap();
        wallet.balances.clear(); // Clear wallet balances before each test
    }

    #[test]
    fn test_receive_tokens() {
        setup();
        let address = "wallet1".to_string();
        let result = receive_tokens(address.clone(), 100);
        assert_eq!(result, Ok(()));
        let balance = get_balance(address);
        assert_eq!(balance, 100);
    }

    #[test]
    fn test_send_tokens() {
        setup();
        let address1 = "wallet1".to_string();
        let address2 = "wallet2".to_string();

        receive_tokens(address1.clone(), 100).unwrap();
        let result = send_tokens(address1.clone(), address2.clone(), 50);
        assert_eq!(result, Ok(()));
        assert_eq!(get_balance(address1), 50);
        assert_eq!(get_balance(address2), 50);
    }

    #[test]
    fn test_invalid_address() {
        setup();
        let result = receive_tokens("".to_string(), 100);
        assert_eq!(result, Err("Invalid wallet address".to_string()));
    }

    #[test]
    fn test_insufficient_balance() {
        setup();
        let address1 = "wallet1".to_string();
        let address2 = "wallet2".to_string();

        receive_tokens(address1.clone(), 50).unwrap();
        let result = send_tokens(address1.clone(), address2.clone(), 100);
        assert_eq!(result, Err("Insufficient balance".to_string()));
    }
}
