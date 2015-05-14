pub trait Restricted<T>: AsRef<T> {
    fn is_allowed(&self, content: &T) -> bool;
    fn sanitize(&self, content: &mut T);
    unsafe fn set_unchecked(&mut self, content: T);

    fn set(&mut self, mut new_content: T) {
        while self.is_disallowed(&new_content) {
            self.sanitize(&mut new_content);
        }
        unsafe {
            self.set_unchecked(new_content);
        }
    }

    fn is_disallowed(&self, content: &T) -> bool {
        !self.is_allowed(content)
    }

    /// Check for invalid data, that may have been introduced by unsafe access.
    fn is_valid(&self) -> bool {
        self.is_allowed(self.as_ref())
    }

    /// Check for invalid data, that may have been introduced by unsafe access.
    fn is_invalid(&self) -> bool {
        self.is_disallowed(self.as_ref())
    }

    fn clone_inner(&self) -> T
        where T: Clone {
        self.as_ref().clone()
    }
}
