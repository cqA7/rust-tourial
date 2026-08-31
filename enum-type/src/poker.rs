#[derive(Debug)]
pub enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}

#[derive(Debug)]
pub struct PokerCard {
    pub suit: PokerSuit,
    pub value: u8,
}

#[derive(Debug)]
pub enum PokerSuitParam {
    Clubs(u8),
    Diamonds(char),
}
