use ic_cdk::export::{
    candid::{CandidType, Deserialize},
    Principal,
};
use ic_cdk_macros::*;
use std::cell::RefCell;

// Define a struct to hold the greeting message
#[derive(CandidType, Deserialize, Clone, Default)]
struct GreetingState {
    message: String,
    history: Vec<String>,
}

// Create a thread-local variable to store the state
thread_local! {
    static STATE: RefCell<GreetingState> = RefCell::new(GreetingState {
        message: "Hello, World!".to_string(),
        history: vec!["Hello, World!".to_string()],
    });
}

#[init]
fn init() {
    // Initialize the canister state with a default greeting
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.message = "Hello, World!".to_string();
        state.history = vec!["Hello, World!".to_string()];
    });
}

#[update]
fn update_greeting(new_greeting: String) -> String {
    // Update the greeting and return the new greeting
    STATE.with(|state| {
        let mut state = state.borrow_mut();
        state.message = new_greeting.clone();
        state.history.push(new_greeting.clone());
        new_greeting
    })
}

#[query]
fn get_greeting() -> String {
    // Return the current greeting
    STATE.with(|state| {
        state.borrow().message.clone()
    })
}

// Additional challenge: Add a method to get the greeting history
#[query]
fn get_greeting_history() -> Vec<String> {
    // Return the greeting history
    STATE.with(|state| {
        state.borrow().history.clone()
    })
}

// Candid service definition for interface description
ic_cdk::export::candid::export_service!();

#[query(name = "__did_you_deploy_me")]
fn did_you_deploy_me() -> String {
    ic_cdk::export::candid::export_service!()
} 