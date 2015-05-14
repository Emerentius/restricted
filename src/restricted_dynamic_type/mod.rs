use restricted_trait::Restricted;
use std::cmp::*;
use std::ops::*;
use std::convert::{AsRef, Into};

pub struct RestrictedDyn<T> {
    data: T,
    pub check: Box<Fn(&T) -> bool>,
    pub sanitizer: Box<Fn(&mut T)>,
}

/* T is not AsRef<T>, therefore not possible
impl<T,R> AsRef<R> for RestrictedDyn<T>
    where T: AsRef<R> {
    fn as_ref(&self) -> &R {
        self.data.as_ref()
    }
}
*/

impl<T> AsRef<T> for RestrictedDyn<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T> Restricted<T> for RestrictedDyn<T> {
    fn is_allowed(&self, data: &T) -> bool {
        (self.check)(data)
    }

    fn sanitize(&self, data: &mut T) {
        while self.is_disallowed(data) {
            (self.sanitizer)(data)
        }
    }

    unsafe fn set_unchecked(&mut self, data: T) {
        self.data = data;
    }
}

impl<T> RestrictedDyn<T> {
    pub fn new<C,S>(data: T, check: C, sanitizer: S) -> Self
        where C: 'static + Fn(&T) -> bool,
              S: 'static + Fn(&mut T) {
        RestrictedDyn {
            data: data,
            check: Box::new(check),
            sanitizer: Box::new(sanitizer),
        }.make_valid()
    }

    /// Creates a new instance of RestrictedDyn, that panics on invalid values.
    #[allow(unused_variables)]
    pub fn new_strict<C>(data: T, check: C) -> Self
        where C: 'static + Fn(&T) -> bool, {
        RestrictedDyn {
            data: data,
            check: Box::new(check),
            sanitizer: Box::new(|inv| panic!("Invalid assignment to variable of type Restricted")),
        }.make_valid() // <= executes panic, if data is invalid
    }

    // todo: get rid of this
    pub fn into_inner(self) -> T {
        self.data
    }

    /// Changes the validity check. If the current data is invalid under the
    /// under the new definition, the sanitizer will be invoked on it.
    ///
    /// __Caution: If there is a mismatch between sanitizer and__
    /// __validity check, an infinite loop may occur at this point or later.__
    pub fn set_check<C>(&mut self, check: C)
        where C: 'static + Fn(&T) -> bool{
        while !check(&self.data) {
            (self.sanitizer)(&mut self.data)
        }
        self.check = Box::new(check);
    }

    /// Changes the sanitizing function.
    ///
    /// __Caution: If there is a mismatch between sanitizer and__
    /// __validity check, an infinite loop may occur later.__
    pub fn set_sanitizer<S>(&mut self, sanitizer: S)
        where S: 'static + Fn(&mut T) {
        self.sanitizer = Box::new(sanitizer);
    }

    /// Changes both validity check and sanitizer. If the current data is
    /// invalid under the new definition, it will be sanitized.
    pub fn set_bounds<C,S>(&mut self, check: C, sanitizer: S)
        where C: 'static + Fn(&T) -> bool,
              S: 'static + Fn(&mut T) {
        while !check(&self.data) {
            sanitizer(&mut self.data)
        }
        self.check = Box::new(check);
        self.sanitizer = Box::new(sanitizer);
    }

    fn make_valid(mut self) -> Self {
        while !self.is_valid() {
            (self.sanitizer)(&mut self.data);
        }
        self
    }

    pub fn add<U>(mut self, rhs: U) -> Self
        where T: Add<U, Output=T> {
        self.data = self.data + rhs;
        self.make_valid()
    }

    pub fn bitand<U>(mut self, rhs: U) -> Self
        where T: BitAnd<U, Output=T> {
        self.data = self.data & rhs;
        self.make_valid()
    }

    pub fn bitor<U>(mut self, rhs: U) -> Self
        where T: BitOr<U, Output=T> {
        self.data = self.data | rhs;
        self.make_valid()
    }

    pub fn bitxor<U>(mut self, rhs: U) -> Self
        where T: BitXor<U, Output=T> {
        self.data = self.data ^ rhs;
        self.make_valid()
    }

    // Deref => actual trait

    /// Returns a mutable reference to the inner value.
    /// Writes may leave the value in an invalid state as no checks can be done
    pub unsafe fn deref_mut<O>(&mut self) -> &O
        where T: DerefMut<Target=O> {
        self.data.deref_mut()
    }

    pub fn div<U>(mut self, rhs: U) -> Self
        where T: Div<U, Output=T> {
        self.data = self.data / rhs;
        self.make_valid()
    }

    // Drop
    // Fn
    // FnMut
    // FnOnce
    // Index => actual trait

    // no checks done on write
    pub unsafe fn index_mut<Idx, O>(&mut self, index: Idx) -> &mut O
        where T: IndexMut<Idx, Output=O> {
        self.data.index_mut(index)
    }

    pub fn mul<U>(mut self, rhs: U) -> Self
        where T: Mul<U, Output=T> {
        self.data = self.data * rhs;
        self.make_valid()
    }

    // Neg => actual trait
    // Not => actual trait
    // Rem => actual trait
    // Shl => actual trait
    // Shr => actual trait

    pub fn sub<U>(mut self, rhs: U) -> Self
        where T: Sub<U, Output=T> {
        self.data = self.data - rhs;
        self.make_valid()
    }
}


/*
 *  Implementing std::ops Traits
 */

/* currently (permanently?) not possible to implement unsafely

unsafe impl<T> DerefMut for RestrictedDyn<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.data
    }
}
*/

impl<T> Deref for RestrictedDyn<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.data
    }
}


impl<T, Idx, O> Index<Idx> for RestrictedDyn<T>
    where T: Index<Idx,Output=O>, {
    type Output = O;
    fn index(&self, index: Idx) -> &O {
        self.data.index(index)
    }
}

impl<T> Neg for RestrictedDyn<T>
    where T: Neg<Output=T> {
    type Output = RestrictedDyn<T>;
    fn neg(mut self) -> Self {
        self.data = -self.data;
        self.make_valid()
    }
}

impl<T> Not for RestrictedDyn<T>
    where T: Not<Output=T> {
    type Output = RestrictedDyn<T>;
    fn not(mut self) -> Self {
        self.data = !self.data;
        self.make_valid()
    }
}

impl<T,U> Rem<U> for RestrictedDyn<T>
    where T: Rem<U, Output=T> {
    type Output = RestrictedDyn<T>;
    fn rem(mut self, rhs: U) -> Self {
        self.data = self.data % rhs;
        self.make_valid()
    }
}

impl<T,U> Shl<U> for RestrictedDyn<T>
    where T: Shl<U, Output=T> {
    type Output = RestrictedDyn<T>;
    fn shl(mut self, rhs: U) -> Self {
        self.data = self.data << rhs;
        self.make_valid()
    }
}

impl<T,U> Shr<U> for RestrictedDyn<T>
    where T: Shr<U, Output=T> {
    type Output = RestrictedDyn<T>;
    fn shr(mut self, rhs: U) -> Self {
        self.data = self.data >> rhs;
        self.make_valid()
    }
}


/*
 *  Implementing std::cmp Traits
 */

/*
impl<T> PartialEq<T> for RestrictedDyn<T>
    where T: PartialEq<T> {
        fn eq(&self, other: &T) -> bool {
            self.data.eq(other)
        }

        fn ne(&self, other: &T) -> bool {
            self.data.ne(other)
        }
}
*/

// cannot be implemented
/*
impl<T> Eq for RestrictedDyn<T>
    where T: Eq<T> {}
*/

/*
impl<T> PartialOrd<T> for RestrictedDyn<T>
    where T: PartialOrd<T> {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.data.partial_cmp(other)
    }
}
*/

/* cannot be implemented

impl<T> Ord<T> for RestrictedDyn<T>
    where T: Ord<T> {
    fn cmp(&self, other: &T) -> Ordering {
        self.data.cmp(other)
    }
}
*/


/*
 *  Implement std::convert Traits
 *  Into
 */
/*
impl<T,U> Into<U> for RestrictedDyn<T>
    where T: Into<U> {
    fn into(self) -> U {
        self.data.into()
    }
}
*/
/*
impl Into<u32> for RestrictedDyn<u32>
    //where T: Into<U> {
{
    fn into(self) -> u32 {
        self.data.into()
    }
}
*/
