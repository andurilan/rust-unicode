#![cfg_attr(test, deny(missing_docs, warnings))]
#![forbid(unused_variables)]

/// An owned sequence of Unicode scalar values, equivalent to `String`.
pub struct UString(Vec<char>);

/// A Unicode string slice, equivalent to `str`.
pub struct UStr([char]);
