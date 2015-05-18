//! This crate defines the RestrictedDyn type, which can wrap a variable of arbitrary type and store a validity check and a sanitizer with it. The idea is to be able to express restrictions that don't fit in the type system via dynamically dispatched checks that travel with the data, allowing one to forego manual checks along the way. Should these restrictions change at some point, the functions can be redefined.
//!
//! When the data is modified and falls outside the allowed ranges as given by the check, it will be brought back in by the sanitizer. It will be applied repeatedly if it does not mutate the value sufficiently on the first run. This can result in an infinite loop, if the sanitizer is not carefully created.
//!
//! Most operators are overloaded, so long as the operators work only on one variable or are distinctly asymmetric (like %, <<) with the restricted variable being on the left side. Typically symmetric operations are defined as methods.
//! Ergonomics are subpar and it doesn't play very well with big data structures, if the entries are to be restricted in a similar fashion. There are either a lot duplicated functions or very inefficient checks and sanitizing.
//!
//! ```
//! extern crate restricted_types;
//! use restricted_types::RestrictedDyn;
//!
//! fn main() {
//!     let mut num = RestrictedDyn::new(
//!         2u32,
//!         |n: &u32| *n >= 20 && *n <= 40,
//!         |n: &mut u32| *n = *n % 20 + 20
//!     );
//!     assert_eq!(22, *num);
//!     num = num.add(37);
//!     assert_eq!(39, *num);
//!
//!     // also with deref coercion
//!     let r: &u32 = &num;
//!     assert_eq!(&39, r);
//! }
//! ```
mod restricted_trait;
mod restricted_dynamic_type;

pub use restricted_trait::Restricted;
pub use restricted_dynamic_type::RestrictedDyn;
