// This (public) constant is only used by a test, so we tell
// the compiler to "see" it in test mode, but not otherwise - to
// avoid the const-unused warning.
#[cfg(test)]
pub const FIBBLE: i32 = 76;
