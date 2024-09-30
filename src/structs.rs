#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    struct FooStruct {
        count: i32,
        ratio: f64,
    }

    #[test]
    fn can_define_and_instantiate() {
        let fibble = FooStruct {
            count: 42,
            ratio: PI,
        };
        assert_eq!(fibble.count, 42);
        assert_eq!(fibble.ratio, PI);
    }

    #[test]
    fn can_mutate_when_marked_mutable() {
        let mut fibble = FooStruct {
            count: 42,
            ratio: PI,
        };
        fibble.count += 1;
        assert_eq!(fibble.count, 43);
    }

    fn construct_with_some_defaults(count: i32) -> FooStruct {
        // Note name-matching shorthand to consume count argument.
        FooStruct { count, ratio: PI }
    }

    #[test]
    fn can_construct_with_some_defaults() {
        let fibble = construct_with_some_defaults(43);
        assert_eq!(fibble.count, 43);
        assert_eq!(fibble.ratio, PI);
    }

    #[test]
    fn can_use_update_copy_idiom() {
        let fibble = FooStruct {
            count: 42,
            ratio: PI,
        };

        let fabble = FooStruct {
            ratio: 1.414,
            ..fibble // Init remaining fields from fibble instance.
        };

        // Note that the ..fibble will cause fabble to take ownership of the
        // any of the original fields that do not implement the Copy trait.

        assert_eq!(fabble.count, 42);
        assert_eq!(fabble.ratio, 1.414);
    }

    // Merit of tuple-struct is that it defines a new type and thus provides
    // both readability, domain-specific type safety, and ability to have methods.
    //
    // Note members can be heterogeneous types as in this case.
    struct RankAndNumber(&'static str, i32);

    #[test]
    fn can_instantiate_a_tuple_struct() {
        let pike = RankAndNumber("Captain", 43);
        assert_eq!(pike.0, "Captain");
        assert_eq!(pike.1, 43);
    }

    // A unit-like struct is a struct that has no state.
    // They provide a simple type on which you can implement a trait.
    struct MyUnitLikeStruct;
    #[test]
    fn can_instantiate_a_unit_like_struct() {
        let _not_a_thing = MyUnitLikeStruct;
    }

    // A trivial struct with one method.
    struct HasAMethod {
        n: i32,
    }

    impl HasAMethod {
        fn squared(&self) -> i32 {
            return self.n * self.n;
        }
    }

    #[test]
    fn can_call_a_method() {
        assert_eq!(HasAMethod { n: 3 }.squared(), 9);
    }
}
