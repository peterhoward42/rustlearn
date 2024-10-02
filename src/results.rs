#[cfg(test)]

mod tests {
    use std::fs::File;

    #[test]
    fn reason_over_a_std_io_result_with_match() {
        let res = File::open("no such file");
        match res {
            Ok(_file) => (), // consume _file in this expression
            Err(_err) => (), // consume _err in this expression
        };
    }

    // See also unwrap() and similar unpacking methods.
}
