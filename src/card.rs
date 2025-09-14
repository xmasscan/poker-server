/*
 * https://xmasscan.github.io
 *
 * Poker Server
 * Playing card definition
 *
 */

use std::fmt;
use std::fmt::Error;
use std::fmt::Formatter;

// Playing Card suits as an enum!
#[derive(Copy, Clone, Default)]
pub enum Suit {
    // When Default::default() is invoked, the Club variant will be used as the default Suit
    // variant.
    #[default]
    Club,
    Spade,
    Heart,
    Diamond,
}

// print macro support for Suit enum
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        match self {
            Suit::Heart => write!(f, "Heart"),
            Suit::Diamond => write!(f, "Diamond"),
            Suit::Spade => write!(f, "Spade"),
            Suit::Club => write!(f, "Club"),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct PlayingCard {
    pub suit: Suit,
    pub value: u8,
}

// print macro support for PlayingCard struct
impl fmt::Display for PlayingCard {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self.value {
            // Ace
            1 => write!(f, "Ace of {}s", self.suit),
            11 => write!(f, "Jack of {}s", self.suit),
            12 => write!(f, "Queen of {}s", self.suit),
            13 => write!(f, "King of {}s", self.suit),
            _ => write!(f, "{} of {}s", self.value, self.suit),
        }
    }
}

// PlayingCard functions
impl PlayingCard {
    // PlayingCard constructor
    // Takes enum Suit, u8 as input and returns a PlayingCard struct
    pub fn new(suit: Suit, value: u8) -> Self {
        Self {
            suit: suit,
            value: value,
        }
    }

    // getters for suit & value
    pub fn get_suit(&self) -> &Suit {
        &self.suit
    }

    pub fn get_value(&self) -> &u8 {
        &self.value
    }
}

// PlayingCard funcationality testing
#[cfg(test)]
mod tests {
    use super::*;

    // Special case formatting
    #[test]
    fn playingcard_print() {
        let card_ace = PlayingCard {
            suit: Suit::Club,
            value: 1,
        };
        let card_jack = PlayingCard {
            suit: Suit::Club,
            value: 11,
        };
        let card_queen = PlayingCard {
            suit: Suit::Club,
            value: 12,
        };
        let card_king = PlayingCard {
            suit: Suit::Club,
            value: 13,
        };

        // Testing for Ace case
        assert_eq!(card_ace.to_string(), String::from("Ace of Clubs"));
        // Jack case
        assert_eq!(card_jack.to_string(), String::from("Jack of Clubs"));
        // Queen case
        assert_eq!(card_queen.to_string(), String::from("Queen of Clubs"));
        // King case
        assert_eq!(card_king.to_string(), String::from("King of Clubs"));
    }

    #[test]
    // Constructor test
    fn playingcard_construct() {
        // Manually constructed PlayingCard
        let manual_card = PlayingCard {
            suit: Suit::Heart,
            value: 7,
        };
        let construct_card = PlayingCard::new(Suit::Heart, 7);

        assert!(matches!(manual_card.get_suit(), Suit::Heart));
        assert_eq!(manual_card.get_value(), construct_card.get_value());
    }
}
