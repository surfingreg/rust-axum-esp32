//!
//! main.rs
//!
//!

/// main
fn main() {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    log::info!("Hello, world!");
}
