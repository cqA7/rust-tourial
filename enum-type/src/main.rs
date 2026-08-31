mod operate;
mod poker;

use operate::{print_poker, print_poker_param};
use poker::{PokerCard, PokerSuit, PokerSuitParam};

fn main() {
    let clubs = PokerSuit::Clubs;
    let spades = PokerSuit::Spades;
    let diamonds = PokerSuit::Diamonds;
    let hearts = PokerSuit::Hearts;

    print_poker(clubs);
    print_poker(hearts);
    print_poker(diamonds);
    print_poker(spades);

    let card1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 3,
    };

    println!("card1 is {:#?}", card1);
    println!("card1's type is {:#?}", card1.suit);
    println!("card1's value is {:#?}", card1.value);

    let card2 = PokerSuitParam::Clubs(10);
    let card3 = PokerSuitParam::Diamonds('A');

    match card2 {
        PokerSuitParam::Clubs(value) => {
            println!("card2's value is {}", value);
        }
        PokerSuitParam::Diamonds(_) => {}
    }

    match card3 {
        PokerSuitParam::Clubs(_) => {}
        PokerSuitParam::Diamonds(char_value) => {
            println!("card3's value is {}", char_value);
        }
    }

    print_poker_param(card2);
    print_poker_param(card3);

    let some_number = Some(5);
    let some_str = Some("String");
    let absent_number: Option<i32> = None;

    match some_number {
        Some(value) => {
            println!("some_number is {}", value);
        }
        None => {}
    }

    match some_str {
        Some(value) => {
            println!("some_str is {}", value);
        }
        None => {}
    }

    match absent_number {
        Some(value) => {
            println!("absent_number is {}", value);
        }
        None => {
            println!("absent_number is None");
        }
    }
}
