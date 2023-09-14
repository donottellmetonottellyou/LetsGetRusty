#![allow(unused)]

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    io,
};

use anyhow::Context;
use thiserror::Error;

#[derive(Debug)]
struct ExpirationDate {
    year: u8,
    month: u8,
}

#[derive(Debug, Error)]
#[error("{msg}")]
struct ParsePaymentInfoError {
    source: Option<anyhow::Error>,
    msg: String,
}

#[derive(Debug, Error)]
enum CreditCardError {
    #[error("{0}")]
    InvalidInput(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

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
    ) -> Result<Self, CreditCardError> {
        let card_string = credit_cards
            .get(name)
            .ok_or(CreditCardError::InvalidInput(format!(
                "No credit card was found for {name}."
            )))?;

        Self::parse_card(card_string)
            .with_context(|| format!("{name}'s card could not be parsed."))
            .map_err(CreditCardError::Other)
    }

    fn parse_card(card_string: &str) -> Result<Self, ParsePaymentInfoError> {
        let numbers = Self::parse_card_numbers(card_string)?;

        let len = numbers.len();
        let expected_len = 4;
        if len != expected_len {
            return Err(ParsePaymentInfoError {
                source: None,
                msg: format!(
                    "Incorrect number of elements parsed. \
                    Expected {expected_len} but got {len}."
                ),
            });
        }

        let number = numbers[0];
        let month = numbers[1]
            .try_into()
            .with_context(|| format!("{} could not be converted to a u8.", numbers[1]))
            .map_err(|e| ParsePaymentInfoError {
                source: Some(e),
                msg: "Month was invalid.".into(),
            })?;
        let year = numbers[2]
            .try_into()
            .with_context(|| format!("{} could not be converted to a u8.", numbers[2]))
            .map_err(|e| ParsePaymentInfoError {
                source: Some(e),
                msg: "Year was invalid.".into(),
            })?;
        let cvv = numbers[3]
            .try_into()
            .with_context(|| format!("{} could not be converted to a u16.", numbers[3]))
            .map_err(|e| ParsePaymentInfoError {
                source: Some(e),
                msg: "CVV was invalid".into(),
            })?;

        Ok(Self {
            number,
            expiration: ExpirationDate { year, month },
            cvv,
        })
    }

    fn parse_card_numbers(card_string: &str) -> Result<Vec<u64>, ParsePaymentInfoError> {
        card_string
            .split(' ')
            .map(|s| {
                s.parse()
                    .with_context(|| format!("{s:?} could not be parsed as u64"))
            })
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|e| ParsePaymentInfoError {
                source: Some(e),
                msg: format!("Failed to parse input as numbers. Input: {card_string}"),
            })
    }
}

fn main() {
    env_logger::init();

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
        Err(e) => {
            match &e {
                CreditCardError::InvalidInput(msg) => println!("\n{msg}"),
                CreditCardError::Other(_) => println!("\nSomething went wrong! Try again."),
            };

            log::error!("\n{e:?}");
        }
    }
}
