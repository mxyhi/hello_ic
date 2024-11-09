use ic_cdk_macros::{query, update};
use std::cell::RefCell;

thread_local! {
    static SUBMITTED_NAMES: RefCell<Vec<String>> = RefCell::new(Vec::new());
}

#[update]
fn greet(name: String) -> String {
    SUBMITTED_NAMES.with(|names| names.borrow_mut().push(name.clone()));
    format!("Hello, {}!", name)
}

#[query]
fn get_submitted_names() -> Vec<String> {
    SUBMITTED_NAMES.with(|names| names.borrow().clone())
}
