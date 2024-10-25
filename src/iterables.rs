#[cfg(test)]

mod tests {

    // Iterators are a pattern governed by the Iterator trait.
    // The trait requires you implement at least self.next().
    // They yield an Option<T> to signify completion with None.
    //
    // In general use you don't call next() yourself, but work with higher abstractions.
    // For example the std libraries default implementations of various methods (like sum()).
    // Or a sort function, or the built-in "for x in seq" flow control statement.
    //
    // They are lazy-evaluated.
    //
    // The iterator itself has to be mutable to be able to hold it's
    // current position in the underlying sequence.
    #[test]
    fn iterators_101() {
        let numbers = [1, 2, 3];
        for n in numbers.iter() {
            _ = n;
        }
    }

    #[test]
    fn derived_adapters() {
        let v: Vec<i32> = vec![1, 2, 3];
        // This iterator method creates a new iterator over v, but which transforms each value it
        // yields according to the given closure.
        // Note they are chainable - as illustrated here.
        let _derived_iter = v.iter().map(|x| x + 1);
    }

    #[test]
    fn the_collect_method() {
        // collect() traverses an iterator, harvesting the items into a Collection
        // of an appropriate kind.
        let v: Vec<i32> = vec![1, 2, 3];
        // If we start by creating an iterator it_1
        let it_1 = v.iter();
        // We can copy the entire sequence into a Collection.
        //
        // Seems to be that you need to give it a hint as to what class of Collection,
        // but can omit from that the generic type parameter.
        let collected: Vec<_> = it_1.collect();
        _ = collected;
    }

    #[test]
    fn filtering_iterator_adapter() {
        let v: Vec<i32> = vec![1, 2, 3];
        // Obtain an iterator over the vector.
        // Apply the filter method to the interator - which provides
        //  a (lazy) "filtering iterator" not a sequence.
        // Finally, collect from said iterator.
        let filtered: Vec<_> = v.into_iter().filter(|n| *n > 1).collect();
        assert_eq!(filtered, vec![2, 3]);

        // Note thata into_inter() took ownership of v.
    }
}
