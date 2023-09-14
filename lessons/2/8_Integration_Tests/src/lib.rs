#[derive(Default)]
pub struct SavingsAccount {
    balance: i32,
}
impl SavingsAccount {
    pub fn new(balance: i32) -> Self {
        Self { balance }
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount.is_negative() {
            panic!("Can not deposit a negative amount!");
        }
        self.balance += amount;
    }

    pub fn transfer(&self, account_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {account_number}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_should_have_a_starting_balance_of_zero() {
        let savings = SavingsAccount::default();
        assert_eq!(savings.get_balance(), 0);
    }

    #[test]
    fn should_be_able_to_create_account_with_balance() {
        let balance = 32;
        let savings = SavingsAccount::new(balance);
        assert_eq!(savings.get_balance(), balance);
    }

    #[test]
    fn should_be_able_to_deposit() {
        let deposit = 100;
        let mut savings = SavingsAccount::new(0);
        savings.deposit(deposit);
        assert_eq!(savings.get_balance(), deposit, "Balance should be 100!");
        assert_ne!(savings.get_balance(), 0, "Balance should not be 0!");
        assert!(savings.get_balance() == deposit, "Balance should be 100!");
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut savings = SavingsAccount::default();
        savings.deposit(-1);
    }

    #[test]
    fn should_transfer_money() -> Result<(), String> {
        let account = SavingsAccount::new(100);
        account.transfer(123456, 100)?;
        Ok(())
    }
}
