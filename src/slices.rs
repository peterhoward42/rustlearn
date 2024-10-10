#[cfg(test)]

mod tests {

    #[test]
    fn string_slices_basics() {
        // The str type is a slice of bytes.
        // String literals are by definition references to a str type.
        //
        let literal = "Hello, World!"; // is type &str
        _ = literal;

        // It is more common to work with &str rather than plan str - so as to only borrow them.
        // E.g. ...

        let a_string = String::from("hello world");
        // Method to get entire slice.
        let _underlying_slice_from_as_string = a_string.as_str();
        // Take a slice to get a partial slice.
        let __partial_slice = &a_string[0..3];

        // Note you can omit the first and or second index as per Go.

        // If you use %str as a function argument type, you can pass
        // in either &str or &String - it is an implicit coercion.
    }

    #[test]
    fn prefer_slice_function_params() {
        // If you use %str as a function argument type, you can pass
        // in either &str or &String - it is an implicit coercion.
        //
        // A &String is a special case of &str - all of it as a slice.
        fn receive_a_string(_s: &str) {}
        receive_a_string(&String::from("foo"));
        receive_a_string("foo");

        // TODO - I saw a passing reference to implicit coercion but haven't
        // encountered the actual documentation yet.
    }
    #[test]
    fn slicing_into_an_array() {
        let an_array = [3, 4, 5];
        let _a_slice = &an_array[1..];
    }
}
