#[cfg(test)]

mod tests {

    // A smart pointer is a conceptual entity.
    // (Usually) a struct that contains a pointer to data on the heap,
    // alongside various metadata.
    //
    // It also has behaviours defined by various traits.
    //
    // The pointer is allowed to own the pointed-to-data.
    //
    // Smart pointers will implement deref() so they can be used
    // in the same way as references, and drop() to participate in
    // ownership transfer.
    //
    // Both String and Box are smart pointer types.

    use std::rc::Rc;

    #[test]
    fn introducing_box() {
        // Just an illustrative struct.
        struct Thing {
            a: i32,
            b: bool,
        }

        // We want to store a Thing on the heap, so we box it.
        let boxed_thing = Box::new(Thing { a: 42, b: false });

        // We can deconstruct the Thing in the usual way, but first have to
        // dereference the smart pointer to get at the Thing.
        let Thing { a, b } = *boxed_thing;
        assert_eq!(a, 42);
        assert_eq!(b, false);
    }

    #[test]
    fn making_a_self_referential_structure_using_box_and_option() {
        struct Node {
            some_info: i32,
            child: Option<Box<Node>>,
        }

        // Build a root node - as yet with no child.
        let mut root_node = Box::new(Node {
            some_info: 42,
            child: None,
        });

        // Create a node to use as the child.
        let child_node = Box::new(Node {
            some_info: 99,
            child: None,
        });

        // Update the root node to point to the child.
        root_node.child = Some(child_node);

        // Prove that the recursive traversal works.
        assert_eq!(root_node.child.unwrap().some_info, 99);
    }

    #[test]
    fn reference_counting_smart_pointer_rc() {
        // I.e. a std::Rc
        //
        // These are a way of holding references, such as the references in place
        // are counted, and the underlying data is dropped, only when the count falls
        // to zero.
        //
        // NOT THREADSAFE!
        //
        struct Thing {
            x: i32,
        }

        // Instantiate a Thing owned by the reference counter.
        let first_counted_reference = Rc::new(Thing { x: 42 });

        // Establish a second counted reference to the SAME underlying thing.
        let second_counted_reference = first_counted_reference.clone();

        // Either should provide access
        assert_eq!(first_counted_reference.x, 42);
        assert_eq!(second_counted_reference.x, 42);

        // Note that these reference-counter wrappers hold an immutable reference to the underlying
        // data by definition.

        // The dropping semantics are that the underlying data will not be dropped until the
        // reference counter falls to zero. (Like C++ "shared_pointer").

        // WARNING
        // If you set up circular references using Rc<> - you leak memory because the
        // corresponding reference counts cannot fall to zero. (Same as C++ shared_pointer!).
        //
        // Todo - is there a tool that can statically analyse this potential?
    }

    // See also RefCell - an official (but unsafe) way to overcome the readonly semantics.
    // It is still able to enforce borrowing rules - BUT ONLY AT RUNTIME - exposing potential runtime panics.
}
