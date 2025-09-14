// Playing card lib
mod card;
use crate::card::*;

fn main() {
    let mut deck = [PlayingCard {
        suit: Suit::Club,
        value: 0,
    }; 52];
    // four suits
    for i in 0..4 {
        let current_suit = match i as u8 {
            0 => Suit::Club,
            1 => Suit::Spade,
            2 => Suit::Heart,
            _ => Suit::Diamond,
        };
        // Ace - 10 + J,Q,K == 13 cards per suit
        for j in 1..14 {
            deck[(i * 13) + (j - 1)] = PlayingCard::new(&current_suit, &(j as u8));
        }
    }
    for card in deck.iter() {
        println!("{card}");
    }
}
