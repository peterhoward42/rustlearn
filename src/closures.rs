#[cfg(test)]

mod tests {

    // This is not about closures in the general design paradigm sense of the word.
    // It is about how closures in Rust specifically interoperate with the ownership
    // and borrowing rules - via some functional traits.
    //
    // This needs special treatment - because closures, by definition have
    // deferred execution, and may read or mutate variables from their outer environment at call-time,
    // AND take ownership (or borrow) things from the environment at call-time.
    // AND transfer ownership back to the environment at call-time.
    //
    // Rust provides traits that allow closures to be classified accordingly.
    //
    // - FnOnce
    //      - If a closure transfers ownership of captured items outside of its body - this
    //        is the only one of the traits it implements. I.e. it is ok only to call it once.
    // - FnMut
    //      - Does not transfer ownership out, but may mutate environment. Ok to call N times.
    // - Fn
    //      - Kinda benign - no impacts as above

    #[test]
    fn explain_fn_once() {
        // This is a conventional, named function that implements both: FnOnce, and Fn.
        fn provide_a_default_number() -> i32 {
            42
        }

        // Create an Option, so that we can illustrate FnOnce closures as
        // an argument to the Option's unwrap_or_else() method that will be called to
        // provide a default value if the option is None.

        let an_option = None;

        // The signature for unwrap_or_else, requires an argument that is a function that
        // implements the FnOnce trait.

        // First we cite a named function as that argument.
        #[allow(clippy::unnecessary_literal_unwrap)]
        let foo = an_option.unwrap_or_else(provide_a_default_number);
        assert_eq!(foo, 42);

        // Now we use a closure for the argument.
        // This closure can be read as:
        // - I am a function.
        // - That takes no input arguments || from the environment.
        // - The body of which {} (implicitly) returns a value of type i32.
        #[allow(clippy::unnecessary_literal_unwrap)]
        #[allow(clippy::unnecessary_lazy_evaluations)]
        let foo = an_option.unwrap_or_else(|| 42);
        assert_eq!(foo, 42);
    }

    #[test]
    fn closure_borrowing_and_returning_a_mutable_reference() {
        let mut list = vec![1, 2, 3];
        // Define a closure that WILL mutate the environment's list,
        // if and when it is called.
        //
        // The closure CODE BLOCK now holds a borrow of the mutable list reference.
        // Remember that borrow is a code-scope concept, not a runtime concept.
        //
        // Note the compiler detects that this closure implements the FnMut trait.
        let mut mutate_the_list = || list.push(7);

        // See Note 1 below.

        // Call the closure.
        mutate_the_list();

        // The compiler can see that there are no more calls to the closure below here - so
        // the borrow of the list reference has now been returned, and
        // we are free to mutate it directly
        list.push(8);

        // Note 1: At the position of Note 1 above, there is exactly one
        // mutable reference to list - held by the closure block. And consequently, it would
        // be disallowed to mutate the list directly at that point, or obtain a mutable reference to it.

        // Note also, you can force the closure to take ownership of the list instead of borrowing it,
        // by preceding the closure's || with mov. A useful way to pass ownership into a thread with spawn.
    }
}
