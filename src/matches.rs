#[cfg(test)]

mod tests {

    #[test]
    fn match_value() {
        let foo = 42;

        match foo {
            42 => {} // a statement
            43 => {} // a statement
            _ => {}  // a statement
        }
    }

    #[test]
    fn match_enum_label() {
        enum Pet {
            Dog,
            Rabbit,
        }

        let my_pet: Pet = Pet::Rabbit;
        let _: Pet = Pet::Dog;

        match my_pet {
            Pet::Dog => {}
            Pet::Rabbit => {}
        }
    }

    #[test]
    fn match_with_or_branch() {
        let foo = 42;

        match foo {
            42 | 43 => {} // a statement
            _ => {}       // a statement
        }
    }
    #[test]
    fn match_with_range() {
        let foo = 42;

        match foo {
            42..=99 => {} // a statement
            _ => {}       // a statement
        }
    }

    #[test]
    fn matches_to_destructure_a_struct() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };

        // This tells the compiler to destructure the point p, selecting the fields
        // x and y from it to initialise the NEW variables a and b.
        //
        // Perhaps we could read it as "let new variables "a" and "b" be created - taken from the values
        // in the struct p"
        let Point { x: a, y: b } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

        // Nb, if you want to name the new variables the same as the field names, you
        // can shortcut thus:
        let Point { x, y } = p;
        _ = x;
        _ = y;
    }

    #[test]
    fn matches_struct_for_particular_field_values() {
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => _ = x, // Matches to points with field y == 0
            _ => {}
        }
    }

    #[test]
    fn matches_to_destructure_a_heterogeneous_enum() {
        struct Point {
            x: i32,
            y: i32,
        }
        struct Line {
            start: Point,
            end: Point,
        }
        // The fields of this enum carry data of disparate types.
        enum Prim {
            Pt(Point),
            Ln(Line),
        }

        // Build a Prim::Ln
        let prim = Prim::Ln(Line {
            start: Point { x: 1, y: 2 },
            end: Point { x: 3, y: 4 },
        });
        // Build a Prim::Pt
        let _prim2 = Prim::Pt(Point { x: 5, y: 6 });

        // Match against a Prim - with a branch for each type, that destructures,
        // said types.
        match prim {
            Prim::Ln(line) => {
                _ = line.start;
                _ = line.end;
            }
            Prim::Pt(pt) => {
                _ = pt.x;
                _ = pt.y;
            }
        }
    }

    // See also matching nested enums and structs.
    // See also matching mixed structs and tuples.
}
