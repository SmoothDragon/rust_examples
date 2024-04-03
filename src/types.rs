
pub fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[cfg(test)]
pub mod test {
    use super::type_of;

    #[test]
    fn test_type_of() {
        assert_eq!("&str", type_of(&"abracadabra"));
        assert_eq!("i32", type_of(&1));
        assert_eq!("u64", type_of(&1u64));
        assert_eq!("char", type_of(&'A'));
    }

}


