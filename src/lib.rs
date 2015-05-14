//! This crate defines the RestrictedDyn type, which can wrap a variable of arbitrary type and store a validity check and a sanitizer with it. When the data is modified and falls outside the allowed ranges as given by the check, it will be brought back in by the sanitizer. The sanitizer will be applied repeatedly if it does not mutate the value sufficiently on the first time.
//!
//! The idea is to be able to forego manual checks when mutating a value and still be able to trust that it falls within the boundaries set at the beginning. Should these boundaries change at some point, the functions can be redefined.
//!
//! Ergonomics are subpar and it doesn't play very well with collections as data, if the entries are to be restricted. There are either a lot duplicated functions or very inefficient writing.
//!
//! ```
//! extern crate restricted;
//! use restricted::RestrictedDyn;
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
//! }
//! ```
mod restricted_trait;
mod restricted_dynamic_type;

pub use restricted_trait::Restricted;
pub use restricted_dynamic_type::RestrictedDyn;
