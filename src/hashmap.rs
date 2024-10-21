#[cfg(test)]

mod tests {

    #[test]
    fn binary_heap_basics() {
        use std::collections::BinaryHeap;
        let mut h = BinaryHeap::new();
        h.push(42);
        h.push(3);
        h.push(99);

        assert_eq!(h.peek().unwrap(), &99);
        assert_eq!(h.pop().unwrap(), 99);
        assert_eq!(h.peek().unwrap(), &42);
    }
}
