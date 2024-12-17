#[cfg(test)]
mod tests {
    use super::*;

    fn setup() {
        let mut wallet = WALLET.lock().unwrap();
        wallet.balances.clear();
    }

    #[test]
    fn test_send_tokens() {
        setup();
        receive_tokens("wallet1".to_string(), 100).unwrap();

        // Attempt to send tokens
        assert_eq!(send_tokens("wallet1".to_string(), "wallet2".to_string(), 50), Ok(()));
        assert_eq!(get_balance("wallet1".to_string()), 50);
        assert_eq!(get_balance("wallet2".to_string()), 50);
    }

    #[test]
    fn test_receive_tokens() {
        setup();

        // Receive tokens
        assert_eq!(receive_tokens("wallet1".to_string(), 100), Ok(()));
        assert_eq!(get_balance("wallet1".to_string()), 100);
    }

    #[test]
    fn test_invalid_address() {
        setup();

        // Test invalid address
        assert_eq!(receive_tokens("".to_string(), 100), Err("Invalid wallet address".to_string()));
        assert_eq!(send_tokens("wallet1".to_string(), "".to_string(), 50), Err("Invalid wallet address".to_string()));
    }

    #[test]
    fn test_insufficient_balance() {
        setup();
        receive_tokens("wallet1".to_string(), 30).unwrap();

        // Attempt to send more tokens than available
        assert_eq!(send_tokens("wallet1".to_string(), "wallet2".to_string(), 50), Err("Insufficient balance".to_string()));
        assert_eq!(get_balance("wallet1".to_string()), 30); // Balance should remain unchanged
    }
}
