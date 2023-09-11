#![no_std]

use gstd::{exec, msg, prelude::*};
use tmg3_io::{
    Tamagotchi, TmgAction, TmgEvent, 
    HUNGER_PER_BLOCK, BOREDOM_PER_BLOCK, ENERGY_PER_BLOCK, 
    FILL_PER_FEED, FILL_PER_ENTERTAINMENT, FILL_PER_SLEEP  
};

static mut TAMAGOTCHI: Option<Tamagotchi> = None;
static mut PREV_HANDLED_BLOCK: u64 = 0;

#[no_mangle]
extern "C" fn init() {
    let name: String = msg::load().expect("Can't load init message");
    let current_block = exec::block_height() as u64;
    unsafe { PREV_HANDLED_BLOCK = current_block };

    unsafe { TAMAGOTCHI = Some(
        Tamagotchi {
            name,
            date_of_birth: exec::block_timestamp(),
            owner: msg::source(),
            fed: Default::default(),
            fed_block: current_block,
            entertained: Default::default(),
            entertained_block: current_block,
            slept: Default::default(),
            slept_block: current_block, 
            approved_account: Default::default(),
        })
    };
}

#[no_mangle]
extern "C" fn handle() {
    let action: TmgAction = msg::load().expect("Error on loading Tamagotchi Action");
    let mut tmg: &mut Tamagotchi = unsafe { TAMAGOTCHI.as_mut().expect("Contract not initialized") };
    let prev_handled_block = unsafe { PREV_HANDLED_BLOCK };
    let current_block = exec::block_height() as u64;

    // do not fail on trying subtraction, rather leave it at default
    let _ = tmg.fed.try_sub( (current_block - prev_handled_block) * HUNGER_PER_BLOCK );
    let _ = tmg.entertained.try_sub( (current_block - prev_handled_block) * BOREDOM_PER_BLOCK );
    let _ = tmg.slept.try_sub( (current_block - prev_handled_block) * ENERGY_PER_BLOCK );

    match action {
        TmgAction::Name => msg::reply(TmgEvent::Name(tmg.name.clone()), 0),
        TmgAction::Age => msg::reply(
            TmgEvent::Age(exec::block_timestamp() - tmg.date_of_birth), 0
        ),
        TmgAction::Feed => {
            tmg.fed.try_add(FILL_PER_FEED).expect("Fed value is out of range");
            tmg.fed_block = current_block;
            msg::reply(TmgEvent::Fed, 0)
        },
        TmgAction::Entertain => {
            tmg.entertained.try_add(FILL_PER_ENTERTAINMENT).expect("Entertained value is out of range");
            tmg.entertained_block = current_block;
            msg::reply(TmgEvent::Entertained, 0)
        },
        TmgAction::Sleep => {
            tmg.slept.try_add(FILL_PER_SLEEP).expect("Slept value is out of range");
            tmg.slept_block = current_block;
            msg::reply(TmgEvent::Slept, 0)
        },
        TmgAction::Transfer(new_owner) => {
            assert!(msg::source() == tmg.owner);
            tmg.owner = new_owner;
            // I conjecture that the new owner prefers to not inhere approved account from the previous owner
            tmg.approved_account = None; 
            msg::reply(TmgEvent::Transferred, 0)
        },
        TmgAction::Approve(account) => {
            // supposing the owner doesn't mean to approve themselves
            assert!(msg::source() == tmg.owner && account != tmg.owner);
            tmg.approved_account = Some(account);
            msg::reply(TmgEvent::Approved, 0)
        },
        TmgAction::RevokeApproval => {
            assert!(msg::source() == tmg.owner);
            tmg.approved_account = None;
            msg::reply(TmgEvent::ApprovalRevoked, 0)
        },
        TmgAction::Full => msg::reply(TmgEvent::Full(tmg.clone()), 0),
    }
    .expect("Failed replying to sender");

    unsafe { PREV_HANDLED_BLOCK = current_block as u64 };
}

#[no_mangle]
extern "C" fn state() {
    msg::reply(
        unsafe { TAMAGOTCHI.as_ref().expect("Contract not initialized") }, 0
    ).expect("Unable to return Tamagotchi instance");
}
