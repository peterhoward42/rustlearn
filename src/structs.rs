pub fn sub(left: u64, right: u64) -> u64 {
    left - right
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    struct FooStruct {
        count: i32,
        ratio: f64,
    }

    #[test]
    fn sub_works() {
        let result = sub(3, 1);
        assert_eq!(result, 2);
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
}
