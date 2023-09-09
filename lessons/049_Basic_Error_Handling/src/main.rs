#![allow(unused)]

use std::{collections::HashMap, io, num::ParseIntError};

#[derive(Debug)]
struct CreditCard {
    number: u64,
    expiration: ExpirationDate,
    cvv: u16,
}
impl CreditCard {
    fn get_credit_card_info(
        credit_cards: &HashMap<&str, &str>,
        name: &str,
    ) -> Result<Self, String> {
        let card_string = credit_cards
            .get(name)
            .ok_or(format!("No credit card was found for {name}."))?;

        Self::parse_card(card_string)
    }

    fn parse_card(card_string: &str) -> Result<Self, String> {
        let numbers = Self::parse_card_numbers(card_string).map_err(|e| e.to_string())?;

        let len = numbers.len();
        let expected_len = 4;
        if len != expected_len {
            return Err(format!(
                "Incorrect number of elements parsed. \
                    Expected {expected_len}, but got {len} elements."
            ));
        }

        let number = numbers[0];
        let month = numbers[1]
            .try_into()
            .map_err(|e| format!("Could not convert month: {e}"))?;
        let year = numbers[2]
            .try_into()
            .map_err(|e| format!("Could not convert year: {e}"))?;
        let cvv = numbers[3]
            .try_into()
            .map_err(|e| format!("Could not convert cvv: {e}"))?;

        Ok(Self {
            number,
            expiration: ExpirationDate { year, month },
            cvv,
        })
    }

    fn parse_card_numbers(card_string: &str) -> Result<Vec<u64>, ParseIntError> {
        card_string.split(' ').map(|s| s.parse()).collect()
    }
}

#[derive(Debug)]
struct ExpirationDate {
    year: u8,
    month: u8,
}

fn main() {
    let credit_cards = HashMap::from([
        ("Amy", "1234567891234567 04 25 123"),
        ("Tim", "1234567891234567 06 27"),
        ("Bob", "1234567891234567 Dec 27 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    match CreditCard::get_credit_card_info(&credit_cards, name.trim()) {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(e) => println!("{e}"),
    }
}
