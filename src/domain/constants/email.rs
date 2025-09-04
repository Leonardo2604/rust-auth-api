use once_cell::sync::Lazy;
use regex::Regex;

pub static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").expect("failed to compile email regex")
});

pub const MAX_EMAIL_LENGTH: usize = 255;
