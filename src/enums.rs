#[cfg(test)]
mod tests {

    // This enum defines labels only - with no corresponding values.
    enum LifeForm {
        Mammal,
        Reptile,
    }

    #[test]
    fn can_use_stateless_enum() {
        let _my_pet = LifeForm::Mammal;
        let _ = LifeForm::Reptile;
    }

    // This enum associates a (heterogeneous) typed value to each variant within.
    enum PhysicsMeasurements {
        Weight(i32),
        Velocity(f64),
    }

    #[test]
    fn can_use_statefull_enum() {
        let foo = PhysicsMeasurements::Weight(42);
        let _bar = PhysicsMeasurements::Velocity(3.14);

        match foo {
            PhysicsMeasurements::Weight(v) => {
                assert_eq!(v, 42)
            }
            // This branch uses the todo macro.
            PhysicsMeasurements::Velocity(_v) => todo!(),
        }
        // As an alternative to match, see also "if let".
    }
}
