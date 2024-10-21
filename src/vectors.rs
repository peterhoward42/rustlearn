#[cfg(test)]

mod tests {

    // Vectors use continguous, growable storage, and are optimised for addition and removal at their end,
    // i.e. stack-like behaviour.

    // Note also that String is a wrapper over a Vec<u8>

    #[test]
    fn vector_construction() {
        // Inferred type for members.
        let mut v1 = Vec::new();
        v1.push(42);

        // Constructor macro shorthand.
        let mut _v2 = vec![1, 2, 3];

        // Construction and batch-wise member init.
        let _v3 = vec![0; 5];
    }

    // Not going to bother with the usual suspects like the len() method, or
    // indexing - these follow the usual patterns.

    #[test]
    fn vector_stack_like_behaviour() {
        let mut _v1 = vec![0; 5];
        // Note that pop() returns an Option.
        assert_eq!(_v1.pop().unwrap(), 0);

        _v1.push(99);
    }

    #[test]
    fn give_vect_initial_capacity_to_mitigate_relocation() {
        // This guarantees not to cost a wholesale reallocation until the number
        // of members exceeds 10.
        let mut _v = Vec::with_capacity(10);
        _v.push(3.14);
    }

    #[test]
    fn obtaining_a_read_only_slice_from_a_vector() {
        let v = vec![0; 5];
        // You can get a slice like this from a Vec,
        // BUT ONLY if you declare the TYPE of the LHS explicitly like I've done here.
        // IF YOU RELY ON implicit typing, you get a reference to a Vec.
        let _x: &[usize] = &v;

        // Or use the as-slice() method.
        assert_eq!(v.as_slice(), &[0, 0, 0, 0, 0]);
    }

    #[test]
    fn vectors_can_shrink_to_fit() {
        // Give it capacity of 5 elements.
        let mut _v = vec![0; 5];
        // Push only one element into it.
        _v.push(42);
        // Give it the opportunity to shrink.
        // But the contract is it may retain some free capacity.
        _v.shrink_to_fit();
    }

    #[test]
    fn harvest_a_region_of_a_vect_with_drain() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        {
            // drain() takes a mutable reference to v, so
            // this code is in its own block so that the variable "drained",
            // gets dropped before the assertion on the remainder that comes
            // afterwards.
            let drained = v.drain(2..5);
            assert_eq!(drained.as_slice(), &[3, 4, 5]);
        }
        assert_eq!(v, vec![1, 2, 6]);
    }

    #[test]
    fn you_can_truncate_a_vector() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        {
            v.truncate(3);
        }
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn insert_in_middle() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        {
            v.insert(3, 99);
        }
        assert_eq!(v, vec![1, 2, 3, 99, 4, 5, 6]);
    }

    #[test]
    fn remove_in_middle() {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        {
            v.remove(3);
        }
        assert_eq!(v, vec![1, 2, 3, 5, 6]);
    }

    #[test]
    fn retain_filter() {
        let mut v = vec![1, 2, 3, 4];
        v.retain(|&x| x % 2 == 0);
        assert_eq!(v, [2, 4]);
    }

    #[test]
    fn dedupe_with_comparison_operator() {
        // This DOES NOT dedupe ALL caught items, vector-wise,
        // it only dedupes CONSECUTIVE sequences!!!
        let mut vec = vec!["foo", "bar", "Bar", "baz", "bar"];
        vec.dedup_by(|a, b| a.eq_ignore_ascii_case(b));
        assert_eq!(vec, ["foo", "bar", "baz", "bar"]);
    }
    // See also the simpler variant that inspects only one item at a time, rather than
    // a pair. I.e. dedup_by_key().

    #[test]
    fn append_one_vec_to_another() {
        let mut vec = vec![1, 2, 3];
        let mut vec2 = vec![4, 5, 6];
        vec.append(&mut vec2);
        assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
        // Observe it empties the donor vector.
        assert_eq!(vec2, []);
    }

    #[test]
    fn split_a_vector_into_two_segments_at_index() {
        let mut vec = vec![1, 2, 3];
        let vec2 = vec.split_off(1);
        assert_eq!(vec, [1]);
        assert_eq!(vec2, [2, 3]);
    }

    // I think many of these provide common trait implementations.
    //
    // See also
    // extend_*()
    // into_flattene()

    // See also helpers for ascii byte vectors - such as various trim() functions,
    // and "contains", "startswith" etc.

    // See also methods associated with "first" and "last".
    // See also methods associated with "iter"".
    // See also methods associated with "reverse".
    // See also methods associated with "windows".
    // See also methods associated with "search".
    // See also methods associated with "sort".
}
