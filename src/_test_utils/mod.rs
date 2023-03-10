use std::sync::Once;

static INIT: Once = Once::new();

/// Setup function that is only run once, even if called multiple times.
#[allow(dead_code)]
pub fn setup() {
    INIT.call_once(|| {
        env_logger::init();
    });
}
