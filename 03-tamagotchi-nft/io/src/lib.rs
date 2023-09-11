#![no_std]

use codec::{Decode, Encode};
use gmeta::Metadata;
use gstd::{ActorId, prelude::*};
use scale_info::TypeInfo;
use gmeta::{In, InOut};

pub const HUNGER_PER_BLOCK: u64 = 1;
pub const BOREDOM_PER_BLOCK: u64 = 2;
pub const ENERGY_PER_BLOCK: u64 = 2;
pub const FILL_PER_FEED: u64 = 1000;
pub const FILL_PER_ENTERTAINMENT: u64 = 1000;
pub const FILL_PER_SLEEP: u64 = 1000;

#[derive(Clone, Default, Encode, Decode, TypeInfo, Debug)]
pub struct Tamagotchi {
    pub name: String,
    pub date_of_birth: u64,
    pub owner: ActorId,
    pub fed: Mood,
    pub fed_block: u64,
    pub entertained: Mood,
    pub entertained_block: u64,
    pub slept: Mood,
    pub slept_block: u64,
    pub approved_account: Option<ActorId>,
}

#[derive(Clone, Encode, Decode, TypeInfo, Debug)]
pub struct Mood(u64);

impl Mood {
    pub fn try_add(&mut self, n: u64) -> Result<(), ()> {
        let tmp = self.0 + n;
        if tmp < 10_000 {
            self.0 = tmp;
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn try_sub(&mut self, n: u64) -> Result<(), ()> {
        if self.0 > n {
            self.0 -= n;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl TryFrom<u64> for Mood {
    type Error = ();
    fn try_from(n: u64) -> Result<Self, ()> {
        if n > 0 && n < 10_000 {
            Ok(Self(n))
        } else {
            Err(())
        }
    }
} 

impl Default for Mood {
    fn default() -> Self {
        Self(1)
    }
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgAction {
    Name,
    Age,
    Feed,
    Entertain,
    Sleep,
    Full,
    Transfer(ActorId),
    Approve(ActorId),
    RevokeApproval,
}

#[derive(Encode, Decode, TypeInfo)]
pub enum TmgEvent {
    Name(String),
    Age(u64),
    Fed,
    Entertained,
    Slept,
    Transferred,
    Approved,
    ApprovalRevoked,
    Full(Tamagotchi)
}

pub struct ProgramMetadata;

impl Metadata for ProgramMetadata {
    type Init = In<String>;
    type Handle = InOut<TmgAction, TmgEvent>;
    type State = Tamagotchi;
    type Reply = ();
    type Others = ();
    type Signal = ();
}
