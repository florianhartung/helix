use std::ops::{Deref, DerefMut};

/// This allows closures to accept some &mut data through an owned handle.
/// It essentially removes the lifetime from the &mut reference temporarily.
pub struct TemporaryOwnedBorrowMut<T> {
    inner: *mut T,
}

static_assertions::assert_not_impl_any!(crate::plugin::temporary_owned_reference::TemporaryOwnedBorrowMut<()>: Send, Sync);

impl<T> Deref for TemporaryOwnedBorrowMut<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // TODO is this safe?
        unsafe {& *self.inner}
    }
}

impl<T> DerefMut for TemporaryOwnedBorrowMut<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.inner }
    }
}

impl<T> TemporaryOwnedBorrowMut<T> {

    #[allow(unused)]
    pub fn provide_with<R>(
        t: &mut T,
        f: impl FnOnce(TemporaryOwnedBorrowMut<T>) -> (TemporaryOwnedBorrowMut<T>, R),
    ) -> R {
        let cx = Self { inner: t as *mut T };

        let (_t, ret) = f(cx);

        ret
    }
}

impl<T: MutTuple> TemporaryOwnedBorrowMut<T> {
    pub fn provide_with_all<R>(
        ts: T,
        f: impl FnOnce(T::EachAsTemporaryBorrowedMut) -> (T::EachAsTemporaryBorrowedMut, R),
    ) -> R {
        let x = ts.wrap_in_temporary_borrowed_mut();

        let (_t, ret) = f(x);

        ret
    }
}

pub trait MutTuple {
    type EachAsTemporaryBorrowedMut;

    fn wrap_in_temporary_borrowed_mut(self) -> Self::EachAsTemporaryBorrowedMut;
}

impl<A> MutTuple for (&mut A,) {
    type EachAsTemporaryBorrowedMut = (TemporaryOwnedBorrowMut<A>,);

    fn wrap_in_temporary_borrowed_mut(self) -> Self::EachAsTemporaryBorrowedMut {
        (TemporaryOwnedBorrowMut { inner: self.0 as _ },)
    }
}

impl<A, B> MutTuple for (&mut A, &mut B) {
    type EachAsTemporaryBorrowedMut = (TemporaryOwnedBorrowMut<A>, TemporaryOwnedBorrowMut<B>);

    fn wrap_in_temporary_borrowed_mut(self) -> Self::EachAsTemporaryBorrowedMut {
        (
            TemporaryOwnedBorrowMut { inner: self.0 as _ },
            TemporaryOwnedBorrowMut { inner: self.1 as _ },
        )
    }
}

impl<A, B, C> MutTuple for (&mut A, &mut B, &mut C) {
    type EachAsTemporaryBorrowedMut = (TemporaryOwnedBorrowMut<A>, TemporaryOwnedBorrowMut<B>, TemporaryOwnedBorrowMut<C>);

    fn wrap_in_temporary_borrowed_mut(self) -> Self::EachAsTemporaryBorrowedMut {
        (
            TemporaryOwnedBorrowMut { inner: self.0 as _ },
            TemporaryOwnedBorrowMut { inner: self.1 as _ },
            TemporaryOwnedBorrowMut { inner: self.2 as _ },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TemporaryOwnedBorrowMut;


    #[test]
    fn single() {
        let mut my_state= 10;

        let ret = TemporaryOwnedBorrowMut::provide_with(&mut my_state,
            |mut my_state_owned| {
                *my_state_owned += 1;

                (my_state_owned, "hello world")
            }
        );

        assert_eq!(ret, "hello world");
        assert_eq!(my_state, 11);
    }

    #[test]
    fn multiple() {
        let mut state_a = 40;
        let mut state_b = 70;

        let ret = TemporaryOwnedBorrowMut::<(&mut i32, &mut i32)>::provide_with_all(
            (&mut state_a, &mut state_b),
            |(mut state_a_owned, mut state_b_owned)| {
                *state_a_owned += 1;
                *state_b_owned += 2;

                ((state_a_owned, state_b_owned), "hello world")
            }
        );

        assert_eq!(ret, "hello world");
        assert_eq!(state_a, 41);
        assert_eq!(state_b, 72);
    }
}
