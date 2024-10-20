#[cfg(test)]
mod tests {

    #[test]
    fn types_that_implement_copy_do_not_get_moved() {
        fn add_one(n: i32) -> i32 {
            return n + 1;
        }

        let foo = 42;

        // Pass foo into a function.
        assert_eq!(add_one(foo), 43);

        // foo is still available in this scope after calling
        // add_one() because the argument type i32 implements the Copy trait,
        // which means the calling scope can retain ownership. The called
        // function receives a copy of the argument.
        _ = foo;
    }

    #[test]
    fn types_that_do_not_implement_copy_do_get_moved() {
        fn measure(s: String) -> usize {
            return s.len();
        }
        let s = String::from("Hello, world!");
        // This function call, moves ownership of s into the measure function.
        let _ = measure(s);
        // now "s" is no longer defined
    }

    #[test]
    fn assignment_can_drop_ancestor() {
        let s1 = String::from("hello");
        let s2 = s1; // This moves s1 (ownership passes to s2)

        // This would not compile because it accesses s1
        //
        // assert_eq!(s1, String::from("hello"));

        _ = s2;
    }

    #[test]
    fn avoid_the_move_by_cloning() {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        assert_eq!(s1, String::from("hello"));
        _ = s2;
    }

    #[test]
    fn use_return_to_transfer_ownership_back() {
        fn append_to_string(mut s: String) -> String {
            s.push('a');
            return s;
        }
        let s1 = String::from("hello");
        let s1 = append_to_string(s1);

        _ = s1;
    }

    #[test]
    fn avoid_the_move_by_borrowing_ie_taking_a_referennce() {
        // measure() is defined this time to take a reference to the string.
        // i.e. it borrows it, which is to take temporary ownership.
        // By definition when the reference goes out of scope, the ownership
        // reverts to wherever it was borrowed from.
        fn measure(s: &String) -> usize {
            return s.len();
        }
        let s = String::from("hello");
        let _ = measure(&s); // Taking a reference to s to pass in.
        assert_eq!(s, String::from("hello")); // ownership has been restored to s in this scope.

        // See also mutable references.
        // These allow the function to mutate the borrowed item.
        // There's only ever one underlying string data on the heap, so
        // when ownership is returned to the call-site, it is the mutated string.
        //
        // Note also that the compiler demands that only ONE mutable reference exists
        // at a time. To avoid competing mutators - even across threads I believe.
    }
}
