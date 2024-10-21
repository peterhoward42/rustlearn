#[cfg(test)]

mod tests {

    #[test]
    fn illustrate_a_generic_function() {
        // This is more demanding to get to work that at first sight.
        //
        // The function body will imply that certain operations are
        // valid when receiving instances of type T. In our case
        // the implication arises from the ">" operator.
        //
        // The compiler cannot enforce this unless it has more information
        // than simply that there is a generic type T.
        //
        // Specifically we must constrain T to be a type that implements the PartiaOrd trait.
        fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
            let mut big = &list[0];

            for x in list {
                if x > big {
                    big = x;
                }
            }
            big
        }

        assert_eq!(largest(&[5, 4, 3, 99, 1]), &99);

        // todo - I tried to make a generic function parameterised with two params <U,V> - but
        // got bogged down in trying to constrain their types to allow any interactions that
        // involved both types. Hopefully when I study traits more - this will be clearer.
    }

    #[test]
    fn illustrate_a_generic_struct() {
        struct Thing<T> {
            plain_field: i32,
            generic_field: T,
        }

        let thing = Thing {
            plain_field: 42,
            generic_field: "boo",
        };
        // The compiler will now insist that the type of the "thing" variable is qualified thus
        // Thing<&str>. I.e. the instance does NOT have the type Thing, or Thing<T>.

        _ = thing.plain_field;
        _ = thing.generic_field;
    }

    // todo - generic enum
    //
    // tried to make an enum with one variant of type <T> and the other variant an explicit type.
    // But could not assign a value to the explicit type variant (i32).
    //
    // I.e. same pattern as Option.
    //
    // It couldn't work out how to specify the type for <T> when I was setting the value to be
    // the i32 variant. If I gave it () to suggest the unit type - it thought that the
    // i32 variant of the enum was never used.
}
