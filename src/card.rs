use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::Display;

#[derive(Clone, Debug)]
pub enum Card {
    Coeur(u8),
    Pique(u8),
    Carreau(u8),
    Trefle(u8),
}

impl Card {
    pub fn unwrap(&self) -> &u8 {
        match self {
            Card::Coeur(v) => v,
            Card::Pique(v) => v,
            Card::Carreau(v) => v,
            Card::Trefle(v) => v,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_str = match self {
            Card::Coeur(_) => "Coeur",
            Card::Pique(_) => "Pique",
            Card::Carreau(_) => "Carreau",
            Card::Trefle(_) => "Trefle",
        };
        write!(f, "{} {}", color_str, self.unwrap())
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Card) -> bool {
        match (self, other) {
            (Card::Coeur(a), Card::Coeur(b)) => a == b,
            (Card::Pique(a), Card::Pique(b)) => a == b,
            (Card::Carreau(a), Card::Carreau(b)) => a == b,
            (Card::Trefle(a), Card::Trefle(b)) => a == b,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PackOfCards {
    pack_cards: Vec<Card>,
}

impl PackOfCards {
    pub fn init() -> PackOfCards {
        let mut pack = PackOfCards {
            pack_cards: Vec::new(),
        };
        for _ in 1..4 {
            for n in 1..13 {
                if n >= 10 {
                    pack.pack_cards.push(Card::Coeur(10));
                    pack.pack_cards.push(Card::Pique(10));
                    pack.pack_cards.push(Card::Carreau(10));
                    pack.pack_cards.push(Card::Trefle(10));
                } else {
                    pack.pack_cards.push(Card::Coeur(n));
                    pack.pack_cards.push(Card::Pique(n));
                    pack.pack_cards.push(Card::Carreau(n));
                    pack.pack_cards.push(Card::Trefle(n));
                }
            }
        }
        pack.shuffle();
        pack
    }

    pub fn iterator(&self) -> &Vec<Card> {
        &self.pack_cards
    }

    pub fn get_card(&self, index: usize) -> Option<&Card> {
        self.pack_cards.get(index)
    }

    pub fn new() -> PackOfCards {
        PackOfCards {
            pack_cards: Vec::new(),
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.pack_cards.push(card);
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.pack_cards.shuffle(&mut rng);
    }

    pub fn pick(&mut self) -> Result<Card, &str> {
        match self.pack_cards.pop() {
            Some(card) => Ok(card),
            None => Err("No more cards"),
        }
    }

    pub fn sum(&self) -> u8 {
        let mut total = 0;
        for card in &self.pack_cards {
            match card.unwrap() {
                1 => {
                    if total + 11 > 21 {
                        total += 1;
                    } else {
                        total += 11;
                    }
                }
                value => total += value,
            }
        }
        total
    }
}
