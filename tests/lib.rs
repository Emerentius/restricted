mod tests {
    extern crate restricted_types;
    use self::restricted_types::RestrictedDyn;
    use self::restricted_types::Restricted;

    #[test]
    fn make_valid() {
        let mut num = RestrictedDyn::new(
            2u32,
            |n: &u32| *n >= 20 && *n <= 40,
            |n: &mut u32| *n = *n % 20 + 20
        );
        unsafe {
            num.set_unchecked(62);
        }
        num.make_valid();
        assert_eq!(22, *num);
        num = num.add(37);
        assert_eq!(39, *num);

        // also with deref coercion
        let r: &u32 = &num;
        assert_eq!(&39, r);
    }
}
