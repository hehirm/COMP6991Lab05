use rand::prelude::*;
use rand_chacha::ChaCha20Rng;

/// Takes in a max value and returns a random number between 1 and max
// DO NOT MODIFY
fn get_random_value(max: u8) -> u8 {
    let mut rng = ChaCha20Rng::seed_from_u64(2);
    rng.gen_range(1..=max)
}

pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
}

pub struct Coin;

impl Roll for Coin {
    fn roll(&self) -> u8 {
        get_random_value(3)
    }
}

impl Roll for Die {
    fn roll(&self) -> u8 {
        let max_val = match self {
            Die::D4 => 5,
            Die::D6 => 7,
            Die::D8 => 9,
            Die::D10 => 11,
            Die::D12 => 13,
            Die::D20 => 21,
        };
        get_random_value(max_val)
    }
}

// MODIFY/ADD BELOW HERE ONLY

pub trait Roll {
    fn roll(&self) -> u8;
}
pub fn roll<T: Roll>(item: T) -> u8 {
    item.roll() 
}
