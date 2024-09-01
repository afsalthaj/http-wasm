#[allow(warnings)]
mod bindings;
mod remote;

use crate::bindings::exports::golem::demo::api::*;
use std::cell::RefCell;
use crate::remote::worker_to_worker;

/// This is one of any number of data types that our application
/// uses. Golem will take care to persist all application state,
/// whether that state is local to a function being executed or
/// global across the entire program.
struct State {
    total: u64,
}

thread_local! {
    /// This holds the state of our application.
    static STATE: RefCell<State> = RefCell::new(State {
        total: 0,
    });
}

struct Component;

impl Guest for Component {
    /// Updates the component's state by adding the given value to the total.
    fn add(value: u64) {
        STATE.with_borrow_mut(|state| state.total += value);
    }

    /// Returns the current total.
    fn get() -> u64 {
        STATE.with_borrow(|state| state.total)
    }

    fn remote(input: String) -> Result<String, String> {
        worker_to_worker()
    }
}

bindings::export!(Component with_types_in bindings);
