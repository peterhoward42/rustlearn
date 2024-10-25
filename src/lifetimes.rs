// Lifetimes are often / usually implicit.
// But sometimes you must resolve ambiguity as show here.

#[cfg(test)]

mod tests {

    #[test]
    fn compiler_blocks_dangling_references() {
        let _r; // This defines the variable name, but not its value or type.

        {
            let x = 5;
            _r = &x; // Not a problem in of itself.
        }

        // This would not compile, because while r itself is still in scope - the thing (x)
        // it references got dropped when the inner block scope ended.

        // _ = _r;
    }

    #[test]
    fn use_lifetime_label_to_sync_lifetimes() {
        // Note this function returns one or the other of the
        // references passed in as arguments.
        //
        // It follows that both the function arguments must therefore live
        // for at least as long as the reference that is returned.
        //
        // The compiler cannot deduce which of the two function parameters
        // the returned value is going to become dependent on. So
        // we use lifetime specifiers to sync the necessary lifetimes, and enable
        // it to compile.
        //
        // It does NOT CHANGE the lifetimes of anything! It is giving the
        // compiler the info it needs to POLICE the lifetimes you have given
        // these objects.
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        assert_eq!(longest("foo", "barry"), "barry");
    }

    #[test]
    fn sync_reference_fields_in_struct_to_the_struct_instance() {
        // When a struct has a field that is a referennce, we have
        // to tell the compiler that no struct instance can outlive the reference it holds.

        // Here we are using generics to parameterise the Person struct with a lifetime
        // label. So you can't now refer to the type as just a plain Person - you must use the
        // fully qualified form with the label.
        struct Person<'a> {
            name: &'a str, // Tell the compiler that no instance of Person can outlive the name field ref.
            age: i32,
        }

        let brother = Person {
            name: "fred",
            age: 42,
        };

        let _ = (brother.name, brother.age);
    }

    #[test]
    // One might expect the reference to &self in the is_water() method to require
    // a lifetime annotion - but the rules of "Elision" cover this as a time saving
    // exception.
    fn show_how_lifetime_elision_can_cope_with_self_references_in_methods() {
        struct Bucket<'a> {
            filled_with: &'a str, // A reference field.
        }
        impl Bucket<'_> {
            fn is_water(&self) -> bool {
                self.filled_with == "water"
            }
        }
        let my_bucket = Bucket {
            filled_with: "water",
        };
        assert!(my_bucket.is_water());
    }

    #[test]
    fn you_can_set_a_static_lifetime() {
        // Using the static lifetime means the reference will be
        // kept alive for the duration of the process.
        let s: &'static str = "I have a static lifetime.";
        _ = s;
    }
}
