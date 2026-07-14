/// Application-wide configuration.
pub mod mod_inner {
    pub const APP_NAME: &str = "HISSEC* Hospital System";
    pub const VERSION:  &str = "1.0.0";
    pub const HOST:     &str = "127.0.0.1";
    pub const PORT:     u16  = 8080;
    pub const LOG_LEVEL: &str = "info";
}

pub use mod_inner::*;
