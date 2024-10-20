// I am comprised of the submodule "abc", but I am NOT making it public to my parent module.
mod abc;

#[cfg(test)]

mod tests {

    #[test]
    fn show_access_to_nested_module_item_using_full_path() {
        assert_eq!(crate::nestedmodules::abc::def::FIBBLE, 76);
    }

    #[test]
    fn show_access_to_nested_module_item_using_super_relative_path() {
        assert_eq!(super::abc::def::FIBBLE, 76);
    }

    #[test]
    fn show_access_to_nested_module_item_using_shortened_path() {
        use crate::nestedmodules::abc::def::FIBBLE;
        assert_eq!(FIBBLE, 76);
    }

    #[test]
    fn show_access_to_nested_module_item_using_aliased_path() {
        use crate::nestedmodules::abc::def::FIBBLE as FIB;
        assert_eq!(FIB, 76);
    }
}
