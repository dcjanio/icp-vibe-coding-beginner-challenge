use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;

#[derive(Default, CandidType, Deserialize)]
struct GreetingState {
    greeting: String,
}

type State = RefCell<GreetingState>;
thread_local! {
    static STATE: State = RefCell::new(GreetingState {
        greeting: "Hello, Internet Computer!".to_string(),
    });
}

#[init]
fn init() {
    STATE.with(|state| {
        state.borrow_mut().greeting = "Hello, Internet Computer!".to_string();
    });
}

#[update]
fn update_greeting(new_greeting: String) -> String {
    STATE.with(|state| {
        state.borrow_mut().greeting = new_greeting.clone();
        new_greeting
    })
}

#[query]
fn get_greeting() -> String {
    STATE.with(|state| state.borrow().greeting.clone())
}

// Additional challenge: Add a method to get the greeting history 