#![no_std]

mod test;

use codec::{Decode, Encode};
use gmeta::{In, InOut, Metadata, Out};
use gstd::prelude::*;

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct PebblesInit {
    pub difficulty: DifficultyLevel,
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum DifficultyLevel {
    #[default]
    Easy,
    Hard,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesAction {
    Turn(u32),
    GiveUp,
    Restart {
        difficulty: DifficultyLevel,
        pebbles_count: u32,
        max_pebbles_per_turn: u32,
    },
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum PebblesEvent {
    CounterTurn(u32),
    Won(Player),
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub enum Player {
    #[default]
    User,
    Program,
}

#[derive(Debug, Default, Clone, Encode, Decode, TypeInfo)]
pub struct IoGameState {
    pub pebbles_count: u32,
    pub max_pebbles_per_turn: u32,
    pub pebbles_remaining: u32,
    pub program_lastmove: u32,
    pub difficulty: DifficultyLevel,
    pub first_player: Player,
    pub winner: Option<Player>,
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<PebblesInit>;
    type Handle = InOut<PebblesAction, PebblesEvent>;
    type State = Out<IoGameState>;
    type Reply = ();
    type Others = ();
    type Signal = ();
}

#[cfg(not(test))]
pub fn get_random_u32() -> u32 {
    use gstd::{exec, msg};

    let salt = msg::id();
    let (hash, _num) = exec::random(salt.into()).expect("internal error: random call failed");
    u32::from_le_bytes([hash[0], hash[1], hash[2], hash[3]])
}

// mock for test
#[cfg(test)]
pub fn get_random_u32() -> u32 {
    use getrandom::getrandom;
    let mut buffer = [0u8; 4];
    getrandom(&mut buffer).expect("Failed to generate random number");
    u32::from_ne_bytes(buffer)
}

// 程序生成数
pub fn program_turn_gen(difficulty: DifficultyLevel, remaining: u32, max_per_turn: u32) -> u32 {
    match difficulty {
        DifficultyLevel::Easy => {
            let mut count;
            if remaining <= max_per_turn {
                return remaining;
            }
            loop {
                count = get_random_u32() % max_per_turn;
                count += 1;
                if count <= remaining {
                    break;
                }
            }
            count
        }
        DifficultyLevel::Hard => {
            let mut count: u32;
            if remaining <= max_per_turn {
                return remaining;
            }
            loop {
                count = get_random_u32() % max_per_turn;
                count += 1;

                if count > remaining {
                    continue;
                }
                let next_remaining = remaining - count;
                // 让程序选择后剩下的为n*k+1, 最后一次用户选时(k+1), 用户难度加大
                if next_remaining % max_per_turn == 1 {
                    break;
                }
            }
            count
        }
    }
}
