#![feature(custom_attribute, param_attrs)]

use test_fn_arg_attr_codegen::test_attr;

#[test_attr]
pub fn test(#[foo] _bar: ()) {
    // why is `#[foo]` considered to be unused?
    // look at codegen/src/lib.rs for the `#[test_attr]` codegen implementation
    // which actually uses `#[foo]`
}
