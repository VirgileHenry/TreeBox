use std::{
    rc::{Weak, Rc},
    cell::RefCell,
};

#[cfg(test)]
pub mod test;

/// A tree box.
/// A structure holding a value, with non-owning references to parents and children.
pub struct TreeBox<T> {
    /// We assume that no ref to this can't ever exist.
    /// This way, we can safely borrow the data.
    inner: Rc<RefCell<TreeBoxData<T>>>,
}


struct TreeBoxData<T> {
    value: T,
    parent: Option<Weak<RefCell<TreeBoxData<T>>>>,
    children: Vec<Weak<RefCell<TreeBoxData<T>>>>,
}

impl<T> TreeBoxData<T> {

    fn get_parent_rec<U, F: Fn(&T) -> Option<U>, G: Fn(&T, Option<U>) -> U>(&self, try_get: &F, parent_fallback: &G) -> U {
        match try_get(&self.value) {
            Some(v) => v,
            None => match self.parent {
                Some(ref parent) => match parent.upgrade() {
                    Some(parent) => {
                        let parent = RefCell::borrow(&parent);
                        parent_fallback(&self.value, Some(parent.get_parent_rec(try_get, parent_fallback)))
                    },
                    None => parent_fallback(&self.value, None),
                },
                None => parent_fallback(&self.value, None),
            }
        }
    }
    
    fn mutate_parent_rec<F: Fn(&mut T)>(&mut self, f: F) {
        match self.parent {
            Some(ref parent) => match parent.upgrade() {
                Some(parent) => {
                    let mut parent = RefCell::borrow_mut(&parent);
                    f(&mut parent.value);
                    parent.mutate_parent_rec(f);
                },
                None => {/* parent got dropped, don't execute */},
            },
            None => {/* no parent, don't execute */},
        }
    }

    fn mutate_children_rec<F: Fn(&mut T)>(&mut self, f: &F) {
        for child in &self.children {
            match child.upgrade() {
                Some(child) => {
                    let mut child_tb = RefCell::borrow_mut(&child);
                    f(&mut child_tb.value);
                    child_tb.mutate_children_rec(f);
                    
                },
                None => {/* child got dropped, don't execute */}
            }
        }
    }
}

impl<T> TreeBox<T> {
    /// Creates a new tree box as the children of self.
    /// This will fail if the inner data of self is already borrowed mutably.
    pub fn create_child(&mut self, value: T) -> TreeBox<T> {
        let self_inner = Rc::<RefCell<TreeBoxData<T>>>::downgrade(&self.inner);
        let child_impl = TreeBoxData {
            value,
            parent: Some(self_inner),
            children: Vec::new(),
        };
        let child_inner = Rc::new(RefCell::new(child_impl));
        // the fact that no ref exists to our rc is our invariant, so we can safely borrow mutably
        RefCell::borrow_mut(&self.inner).children.push(Rc::<RefCell<TreeBoxData<T>>>::downgrade(&child_inner));
        TreeBox { inner: child_inner }
    }

    /// Sets the parent of self.
    /// If self already had a parent, we will remove ourself from its children.
    pub fn set_parent(&mut self, parent: Option<&TreeBox<T>>) {
        // ok bc invariant
        if let Some(prev_parent) = RefCell::borrow_mut(&self.inner).parent.take() {
            match prev_parent.upgrade() {
                Some(prev_parent) => RefCell::borrow_mut(&prev_parent).children
                    .retain(|v| v.as_ptr() != Rc::<RefCell<TreeBoxData<T>>>::downgrade(&self.inner).as_ptr()),
                None => {/* parent have been dropped, so don't need to remove ourself from it */}
            }
        }
        RefCell::borrow_mut(&self.inner).parent = parent.map(|v| Rc::<RefCell<TreeBoxData<T>>>::downgrade(&v.inner));
        if let Some(parent) = parent {
            RefCell::borrow_mut(&parent.inner).children.push(Rc::<RefCell<TreeBoxData<T>>>::downgrade(&self.inner));
        }
    }

    /// Gets a value out of the tree box by calling the given function with a reference to the inner value.
    /// We can't return a reference to the inner value, because our invariant prevents it.
    pub fn get<U, F: FnOnce(&T) -> U>(&self, f: F) -> U {
        f(&self.inner.borrow().value)
    }

    /// Gets a value from the tree box, with options to look up recursively to parents if necessary.
    /// This is super specific to my use case, but this is why I built this ds.
    /// the try get will be called on the value, and if it fails, it will call the parent fallback.
    /// If we have no parent, the param of the parent fallback is none, otherwise it is the result of a recursive call,
    /// So it's like we called this func on the parent, and used the result as the param for the parent fallback.
    pub fn get_parent_rec<U, F: Fn(&T) -> Option<U>, G: Fn(&T, Option<U>) -> U>(&self, try_get: F, parent_fallback: G) -> U {
        self.inner.borrow().get_parent_rec(&try_get, &parent_fallback)
    }

    /// Call a mutable operation on the inner value.
    pub fn mutate<F: Fn(&mut T)>(&mut self, f: F) {
        f(&mut self.inner.borrow_mut().value);
    }

    /// Call a mutable operation on the parent.
    /// If the parent does not exist, this will do nothing.
    pub fn mutate_parent<F: Fn(&mut T)>(&mut self, f: F) {
        match self.inner.borrow().parent {
            Some(ref parent) => match parent.upgrade() {
                Some(parent) => f(&mut RefCell::borrow_mut(&parent).value),
                None => {/* parent got dropped, don't execute */}
            },
            None => {/* no parent, don't execute */}
        }
    }

    /// Call a mutable operation on the parent, and recursively on all parents.
    /// If the parent does not exist, this will do nothing.
    pub fn mutate_parent_rec<F: Fn(&mut T)>(&mut self, f: F) {
        self.inner.borrow_mut().mutate_parent_rec(f);
    }

    /// Calls the mutating function on all children.
    /// If we encounter a reference to a child that have been dropped, we will not execute the function.
    pub fn mutate_children<F: Fn(&mut T)>(&mut self, f: F) {
        for child in &RefCell::borrow(&self.inner).children {
            match child.upgrade() {
                Some(child) => f(&mut RefCell::borrow_mut(&child).value),
                None => {/* child got dropped, don't execute */}
            }
        }
    }

    /// Calls the mutating function on all children, and on all children recursively.
    /// This allow recursive calls if needed.
    pub fn mutate_children_rec<F: Fn(&mut T)>(&mut self, f: F) {
        self.inner.borrow_mut().mutate_children_rec(&f);
    }
}

impl<T> From<T> for TreeBox<T> {
    /// Create a new tree box with the given value.
    fn from(value: T) -> Self {
        TreeBox {
            inner: Rc::<RefCell<_>>::new(RefCell::new(TreeBoxData {
                value,
                parent: None,
                children: Vec::new(),
            })),
        }
    }
}
