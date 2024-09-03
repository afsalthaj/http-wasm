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
    fn process(actions: Actions, user_id: String) -> CustomResult {
        match actions {
            Actions::Follow(user_id) => CustomResult::Success(format!("follow {}", user_id)),
            Actions::PostTweet(user_id) => CustomResult::Success(format!("tweet {}", user_id)),
            Actions::Register(user_id) => CustomResult::Success(format!("register {}", user_id)),
            Actions::Unfollow(user_id) => CustomResult::Failure("unfollow".to_string()),
            Actions::Noop => CustomResult::Success("noop action".to_string()),
        }
    }

    fn remote(input: String) -> Result<String, String> {
        worker_to_worker(input)
    }
}

bindings::export!(Component with_types_in bindings);
