#![feature(proc_macro_hygiene)]
use tt_identifier::{identify, identify_string};

#[test]
fn it_works() {
    // Test that the identifier can be used in declarations & expressions.
    let identify!(Foo<Bar<(u64, u64)>>) = 2;
    assert_eq!(
        identify!(Foo<Bar<(u64, u64)>>) + identify!(Foo<Bar<(u64, u64)>>),
        4
    );
    // Test that the string equivalent doesn't throw an error.
    let s: &str = identify_string!(Foo<Bar<(u64, u64)>>);
    assert_eq!(s, identify_string!(Foo<Bar<(u64, u64)>>));
}
