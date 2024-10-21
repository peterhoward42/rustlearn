#[cfg(test)]

mod tests {

    #[test]
    fn hashmap_basics() {
        use std::collections::HashMap;

        // implicit typing - viz. string/string
        let mut m: HashMap<String, String> = HashMap::new();

        // Insertion
        m.insert("thiskey".to_string(), "thisvalue".to_string());

        // Contains key query
        // Note the eclectic types that contains can take for the key.
        // It can be anything that implements both Hash and Eq traits in respect to a String.
        assert_eq!(m.contains_key(&"thiskey".to_string()), true);
        assert_eq!(m.contains_key("thiskey"), true);

        // Removal based on key - note that it returns an Option.
        assert_eq!(m.remove("nosuchkey"), None);

        // Get value from key (returns an Option).
        assert_eq!(m.get("thiskey"), Some(&"thisvalue".to_string()));
        assert_eq!(m.get("nosuchkey"), None);

        // Lookup using index notation.
        // Nb. panics if not found
        assert_eq!(m["thiskey"], "thisvalue".to_string());

        // enumerate items
        // Don't have to take reference to m, but doing so borrows.
        for (_k, _v) in &m {
            // consume _k and _v here
        }
    }

    #[test]
    fn construct_from_array() {
        use std::collections::HashMap;
        let _m = HashMap::from([("Mercury", 0.4), ("Venus", 0.7)]);
    }

    #[test]
    fn it_implements_the_entry_api() {
        use std::collections::HashMap;
        let mut m: HashMap<String, String> = HashMap::new();

        // The entry() method populates the map with the provided key/value IFF the map does not
        // contain the key. Then, returns a mutable reference to the in-situ value.
        let _mut_ref_to_the_value = m.entry("age".to_string()).or_insert("42".to_string());
        //
        // See also the variant that provides the default value via a closure.
    }

    #[test]
    fn custom_key_types() {
        // The types you use for map keys must implement certain traits, which you
        // can auto-derive. (todo - always? - seems unlikely)

        #[derive(Hash, Eq, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        use std::collections::HashMap;
        let mut m: HashMap<Point, String> = HashMap::new();
        m.insert(Point { x: 1, y: 2 }, "origin point".to_string());
    }

    #[test]
    fn fetch_all_keys() {
        use std::collections::HashMap;
        let m = HashMap::from([("Mercury", 0.4), ("Venus", 0.7)]);
        for _k in m.keys() {
            // consume _k here
        }

        // See also the values() method.
    }

    // See also the in-common methods as illustrated in the vectors.rs sister module.
}
