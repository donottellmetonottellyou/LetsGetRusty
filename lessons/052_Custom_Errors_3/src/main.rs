#![allow(unused)]

use std::{
    collections::HashMap,
    error::Error,
    fmt::{Debug, Display},
    io,
};

#[derive(Debug)]
struct ExpirationDate {
    year: u8,
    month: u8,
}

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String,
}
impl Debug for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self}\n\t{}", self.msg)?;

        if let Some(e) = self.source.as_ref() {
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}
impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}
impl Error for ParsePaymentInfoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref()
    }
}

enum CreditCardError {
    InvalidInput(String),
    Other(Box<dyn Error>, String),
}
impl Debug for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(msg) => write!(f, "{self}\n{msg}"),
            Self::Other(e, msg) => write!(f, "{self}\n{msg}\n\nCaused by:\n\t{e:?}"),
        }
    }
}
impl Display for CreditCardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}
impl Error for CreditCardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidInput(_) => None,
            Self::Other(e, _) => Some(e.as_ref()),
        }
    }
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

        Self::parse_card(card_string).map_err(|e| {
            CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
        })
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
        let month = numbers[1].try_into().map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: "Month could not be converted to a u8.".into(),
        })?;
        let year = numbers[2].try_into().map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: "Year could not be converted to a u8.".into(),
        })?;
        let cvv = numbers[3].try_into().map_err(|e| ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: "CVV could not be converted to a u16.".into(),
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
                s.parse().map_err(|_| ParsePaymentInfoError {
                    source: None,
                    msg: format!("{s:?} could not be parsed as u64"),
                })
            })
            .collect::<Result<Vec<u64>, _>>()
            .map_err(|e| ParsePaymentInfoError {
                source: Some(Box::new(e)),
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
                CreditCardError::Other(_, _) => println!("\nSomething went wrong! Try again."),
            };

            log::error!("\n{e:?}");
        }
    }
}
