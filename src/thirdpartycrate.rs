#[cfg(test)]

mod tests {

    #[test]
    fn show_use_of_crate_from_rust_dot_io() {
        // See also the dependency entry for fastrand in Cargo.toml
        let num = fastrand::i32(0..10);
        assert_eq!(num >= 0 && num < 10, true);
    }
}