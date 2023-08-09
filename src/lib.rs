use std::{
    rc::{Weak, Rc},
    cell::{RefCell, BorrowMutError, BorrowError, Ref, RefMut},
};

#[derive(Debug)]
pub enum TreeBoxError {
    BorrowMutError(BorrowMutError),
    BorrowError(BorrowError),
}

impl From<BorrowError> for TreeBoxError {
    fn from(err: BorrowError) -> Self {
        TreeBoxError::BorrowError(err)
    }
}

impl From<BorrowMutError> for TreeBoxError {
    fn from(err: BorrowMutError) -> Self {
        TreeBoxError::BorrowMutError(err)
    }
}

type TreeBoxResult<T> = Result<T, TreeBoxError>;

#[cfg(test)]
pub mod test;

pub struct TreeBox<T> {
    inner: Rc<RefCell<TreeBoxImpl<T>>>,
}

struct TreeBoxImpl<T> {
    value: T,
    parent: Option<Weak<RefCell<TreeBoxImpl<T>>>>,
    children: Vec<Weak<RefCell<TreeBoxImpl<T>>>>,
}

impl<T> TreeBox<T> {
    /// Creates a new tree box as the children of self.
    /// This will fail if the inner data of self is already borrowed mutably.
    pub fn create_child(&mut self, value: T) -> TreeBoxResult<TreeBox<T>> {
        let self_inner = Rc::<RefCell<TreeBoxImpl<T>>>::downgrade(&self.inner);
        let child_impl = TreeBoxImpl {
            value,
            parent: Some(self_inner),
            children: Vec::new(),
        };
        let child_inner = Rc::new(RefCell::new(child_impl));
        RefCell::try_borrow_mut(&self.inner)?.children.push(Rc::<RefCell<TreeBoxImpl<T>>>::downgrade(&child_inner));
        Ok(TreeBox { inner: child_inner })
    }

    /// Sets the parent of self.
    /// If self already had a parent, we will remove ourself from its children.
    pub fn set_parent(&mut self, parent: Option<&TreeBox<T>>) -> TreeBoxResult<()> {
        if let Some(prev_parent) = RefCell::try_borrow_mut(&self.inner)?.parent.take() {
            match prev_parent.upgrade() {
                Some(prev_parent) => RefCell::try_borrow_mut(&prev_parent)?.children
                    .retain(|v| v.as_ptr() != Rc::<RefCell<TreeBoxImpl<T>>>::downgrade(&self.inner).as_ptr()),
                None => {/* parent have been dropped, so don't need to remove ourself from it */}
            }
        }
        RefCell::try_borrow_mut(&self.inner)?.parent = parent.map(|v| Rc::<RefCell<TreeBoxImpl<T>>>::downgrade(&v.inner));
        Ok(())
    }

    /// Access a reference to the inner value of this box.
    pub fn value(&self) -> TreeBoxResult<Ref<'_, T>> {
        Ok(Ref::map(self.inner.try_borrow()?, |v| &v.value))
    }

    /// Access a mutable reference to the inner value of this box.
    pub fn value_mut(&mut self) -> TreeBoxResult<RefMut<T>> {
        Ok(RefMut::map(self.inner.try_borrow_mut()?, |v| &mut v.value))
    }

    /// Iterates over the children of self.
    /// The result types are tree box, but they are really a shared reference to the inner value,
    /// because the tree box is an `Rc` at its core.
    pub fn children(&self) -> TreeBoxResult<impl Iterator<Item = TreeBox<T>> + '_> {
        let child_ref = &RefCell::try_borrow(&self.inner)?.children;
        let result = child_ref.iter().map(|v| v.upgrade())
            .filter(|v| v.is_some())
            .map(|v| TreeBox { inner: v.unwrap() })
            .collect::<Vec<_>>();
        Ok(result.into_iter())
    }
}

impl<T> From<T> for TreeBox<T> {
    /// Create a new tree box with the given value.
    fn from(value: T) -> Self {
        TreeBox {
            inner: Rc::<RefCell<_>>::new(RefCell::new(TreeBoxImpl {
                value,
                parent: None,
                children: Vec::new(),
            })),
        }
    }
}
