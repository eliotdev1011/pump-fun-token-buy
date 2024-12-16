use anchor_lang::prelude::*;

#[account]
#[derive(Default)]
pub struct StateAccount {
    pub admin: Pubkey,
    pub fee_address: Pubkey,
    pub fee_amount: u64,      // In basis points (100 = 1%)
    pub max_fee_cap: u64,     // Fixed at 1000 (10%)
    pub paused: bool,
}

impl StateAccount {
    pub const SPACE: usize = 32 + // admin pubkey
                            32 + // fee_address pubkey
                            8 +  // fee_amount
                            8 +  // max_fee_cap
                            1;   // paused

    /// Calculate fee amount for a given purchase amount
    pub fn calculate_fee(&self, amount: u64) -> u64 {
        (amount * self.fee_amount) / 10_000
    }

    /// Calculate final purchase amount after fee deduction
    pub fn calculate_purchase_amount(&self, amount: u64) -> Option<u64> {
        let fee = self.calculate_fee(amount);
        amount.checked_sub(fee)
    }

    /// Check if a new fee is valid
    pub fn is_valid_fee(&self, new_fee: u64) -> bool {
        new_fee <= self.max_fee_cap
    }

    /// Get the current fee percentage as a float
    pub fn get_fee_percentage(&self) -> f64 {
        (self.fee_amount as f64) / 100.0
    }

    /// Check if contract is active (not paused)
    pub fn is_active(&self) -> bool {
        !self.paused
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fee_calculations() {
        let mut state = StateAccount::default();
        state.fee_amount = 100; // 1%

        // Test with 1 SOL (1_000_000_000 lamports)
        let amount = 1_000_000_000;
        let fee = state.calculate_fee(amount);
        assert_eq!(fee, 10_000_000); // 1% of 1 SOL

        let purchase_amount = state.calculate_purchase_amount(amount).unwrap();
        assert_eq!(purchase_amount, 990_000_000); // 99% of 1 SOL
    }

    #[test]
    fn test_fee_validation() {
        let mut state = StateAccount::default();
        state.max_fee_cap = 1000; // 10%

        assert!(state.is_valid_fee(1000));
        assert!(state.is_valid_fee(500));
        assert!(!state.is_valid_fee(1001));
    }

    #[test]
    fn test_fee_percentage() {
        let mut state = StateAccount::default();
        state.fee_amount = 100;
        assert_eq!(state.get_fee_percentage(), 1.0);

        state.fee_amount = 50;
        assert_eq!(state.get_fee_percentage(), 0.5);
    }

    #[test]
    fn test_contract_state() {
        let mut state = StateAccount::default();
        assert!(state.is_active()); // Default is not paused

        state.paused = true;
        assert!(!state.is_active());
    }
}