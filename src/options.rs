#[cfg(test)]
mod tests {

    #[test]
    fn demo_options_basic_idea() {
        // An Option is Rust's bullet-proof alternative take on a nullable value.
        // It is an enum.
        // It is parameterised to a type T - in the cases below i32.
        // It can hold either a concrete value of type T, or None.

        let _foo = Some(42); // It holds an i32 (implicitly typed)
        let _bar: Option<i32> = None; // Instead of an i32, it holds None (requires explicit Type)
    }

    #[test]
    fn using_option_to_signal_function_failure_in_band() {
        fn divide(a: f64, b: f64) -> Option<f64> {
            if b == 0.0 {
                None
            } else {
                Some(a / b)
            }
        }
        assert_eq!(divide(18.0, 6.0), Some(3.0));
        assert_eq!(divide(18.0, 0.0), None);
    }

    #[test]
    fn use_question_mark_as_null_return_value_shortcut() {
        // If you write a ? after an expression that evaluates to an
        // Option, inside a funtion that returns an Option, then the
        // first such evaluated will shortcut-return the function with None, if
        // said Option is None.
        fn nested_fn(a: Option<i32>, b: Option<i32>) -> Option<i32> {
            Some(a? + b?)
        }

        assert_eq!(nested_fn(Some(2), Some(3)), Some(5));
        assert_eq!(nested_fn(Some(2), None), None);
    }

    #[test]
    fn facilitating_nullable_pointer() {
        // Recall that Box<T> is a smart pointer wrapper around a (Copy-able) type T,
        // thus moving the instance of T onto the heap.

        // Thus Option<Box<i32>> , being able to hold None or the Boxed item, may be thought of as a pointer that
        // the compiler knows might not point to anything.

        // Nb. Useful for self-referential structures like say a graph.
        let _foo: Option<Box<i32>> = None;
        let _foo = Some(Box::new(9000));
    }

    #[test]
    fn working_with_references_and_illustrate_map() {
        // There are a set of adapter methods that let you do "stuff" with Options that
        // provide concise ways to convert to and from references - avoiding where possible
        // taking ownership of the inner object.
        //
        // This illustrates one use case and the to_ref() and map() methods.
        //
        // Say we have an Option over a String type and we want to know the
        // length of the underlying string in bytes, but want to avoid transferring
        // ownership of the original String from here.

        let a_string = "Hello, world!".to_string();
        let optional_string: Option<String> = Some(a_string);

        // This first uses the as_ref() Option method to get an Option<&String>, i.e. it
        // borrows the underlying String.
        //
        // The Option's map method provides a new Option over a new type <U> by applying a function
        // to it that must return an Option over type <U>. In this case <usize>.

        // This produces a new Option over <&String> rather than the original <String>,
        // so that when we pass it into map() and thence len() the original string is
        // borrowed, not moved.
        let bar = optional_string.as_ref();

        // This then uses map() to apply a length-calculating function and provide an Option over said length.
        let text_len = bar.map(|s| s.len());
        assert_eq!(text_len, Some(13));

        // Show that the original string is still in scope.
        _ = a_string;
    }

    #[test]
    fn extracting_the_inner_value() {
        // Calling expect() on an Option either returns the inner value or
        // panics if the Option has the None value. For when the only meaningful
        // thing to do is panic immediately.
        #[allow(clippy::unnecessary_literal_unwrap)]
        let _foo = Some(42).expect("msg to panic with if None");

        // unwrap() is the same but with a default panic message.
        #[allow(clippy::unnecessary_literal_unwrap)]
        let _bar = Some(42).unwrap();

        // unwrap_or(), instead of panicking substitutes the given in-band value.
        #[allow(clippy::unnecessary_literal_unwrap)]
        let _baz = Some(42).unwrap_or(-1);
        //
        // See also unwrap_or_else() - which has the same semantics, but differs in
        // when the fallback value is evaluated. (Eager vs. lazy). I cannot yet see any
        // point in this distinction apart from the performance hit of the fallback closure execution.
        //
        // See also unwrap_or_default() - which falls back to default value for a <T>.

        // inspect() is a closure despatcher on an Option, that despatches the
        // closure IFF the Option is not None. It returns the original option.
        let _foo = Some(42);
        _foo.inspect(|_x| { /* can consume _x here */ });
        //
        // Note inspect returns the original Option - so could chain them.

        // See also map_or() etc.
    }

    #[test]
    fn filtering() {
        // Filtering passes or rejects the contained value, coping with None.

        fn pred(v: &i32) -> bool {
            v % 2 == 0
        }

        // filter() should return None if the Option is already None.
        let already_none = None;
        assert_eq!(already_none.filter(pred), None);

        // filter() should return Some(n) if the filter passes the Option's value.
        let two = Some(2);
        assert_eq!(two.filter(pred), Some(2));

        // filter() shouild return None if the filter rejects the Option's value.
        let three = Some(3);
        assert_eq!(three.filter(pred), None);
    }

    // See also or(), or_else(), take(), replace(), zip(), copied(), cloned(),
    // See also "boolean" operators - in which Some() is true and None is false.

    // See also iterating over an Option - which yields either an iterable over
    // either one Some() or no None's. I.e. the latter one does not iterate.o

    // See also the insertion and getter methods.
}
