// Playing card lib
mod card;
use crate::card::*;

fn main() {
    // Deck of cards to draw from :)
    let mut deck = Vec::new();

    // Init deck of cards
    for i in 0..4 {
        let current_suit = match i as u8 {
            0 => Suit::Club,
            1 => Suit::Spade,
            2 => Suit::Heart,
            _ => Suit::Diamond,
        };
        // Ace - 10 + J,Q,K == 13 cards per suit
        for j in 1..14 {
            deck.push(PlayingCard {
                suit: current_suit,
                value: j as u8,
            });
        }
    }

    for card in deck.iter() {
        println!("{card}");
    }
}
