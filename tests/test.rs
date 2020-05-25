#![feature(proc_macro_hygiene)]
use tt_identifier::{identify, identify_string};

#[test]
fn it_works() {
    let identify!(Foo<Bar<(u64, u64)>>) = 2;
    assert_eq!(
        identify!(Foo<Bar<(u64, u64)>>) + identify!(Foo<Bar<(u64, u64)>>),
        4
    );
}
