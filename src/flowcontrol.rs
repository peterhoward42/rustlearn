#[cfg(test)]

mod tests {

    #[test]
    fn ternary_operator_equivalent() {
        // Recall that a block evaluates to an expression - i.e. the final expression inside the block.
        // By definition, an if-statement is also an expression, and can be used
        // as the RHS of a "let" statement.
        let n = 43;
        let b = if n % 2 == 0 { true } else { false };
        assert_eq!(b, false);
    }

    #[test]
    fn infinite_loop() {
        let mut count = 0;
        loop {
            count += 1;
            if count == 23 {
                break;
            }
        }
    }

    #[test]
    fn while_loop_plain() {
        let mut count = 0;
        while count < 23 {
            count += 1;
        }
    }

    #[test]
    fn while_let() {
        let mut s = "hello".to_string();
        while let Some(_c) = s.pop() {
            // consume _c here
        }
    }

    #[test]
    fn for_in_loop_over_array_iterable() {
        // Recall, this is a reference to an array[3] of &str.
        // Which implements std::iter::IntoIterator - a requirement for a for-in loop.
        let strings = &["apples", "cake", "coffee"];
        for _s in strings {
            // consume _s here
        }
    }

    #[test]
    fn for_in_loop_over_range_iterable() {
        for _n in 1..11 {
            // consume _n
        }
    }

    #[test]
    fn break_out_multiple_levels() {
        'a: for _i in 1..4 {
            for _j in 1..4 {
                if _j == 2 {
                    break 'a; // Breaks outer loop.
                              // Same applies to continue
                }
            }
        }
    }

    #[test]
    fn loops_can_return_a_value_via_break() {
        let mut i = 0;
        let result = loop {
            i += 1;
            if i == 3 {
                break "boo";
            }
        }; // The let statement requires a semicolon.
        assert_eq!(result, "boo");
    }
}
