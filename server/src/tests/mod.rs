use crate::http::AppState;

mod request_handler;
mod utils;

pub(super) struct TestEnvironment {
    state: &AppState,
}

impl TestEnvironment {
    fn new() {}
}
