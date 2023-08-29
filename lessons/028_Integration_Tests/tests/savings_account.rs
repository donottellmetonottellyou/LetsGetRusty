#![allow(unused)]

use integration_tests::SavingsAccount;

mod utils;

#[test]
fn should_have_a_default_balance_of_zero() {
    let savings = SavingsAccount::default();
    assert_eq!(0, savings.get_balance());
}
