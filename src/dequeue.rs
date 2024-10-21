#[cfg(test)]

mod tests {

    #[test]
    fn vector_dequeue() {
        use std::collections::VecDeque;

        let mut buf = VecDeque::new();

        buf.push_back(1);
        buf.push_back(2);
        buf.push_front(3);
        buf.push_front(4);

        _ = buf.pop_front();
        _ = buf.pop_back();

        // Underlying storage is a (growable) ring buffer.
        // So contents will often wrap (index-wise), which is awkward for sorting.
        // You can call its make_contiguous() method to solve this.
    }
}
