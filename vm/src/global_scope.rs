//! Scopes for module-local global variables.
use std::ops::Deref;
use std::cell::UnsafeCell;

use object_pointer::ObjectPointer;

/// A GlobalScope contains all the global variables defined in a module.
///
/// Access to variables is _not_ synchronized to reduce overhead. As such one
/// must take care not to modify the list of variables in a concurrent manner.
///
/// Since modules are only executed once this should typically not be a problem.
///
/// Furthermore, a global scope may only contain permanent pointers. This is
/// necessary as otherwise a scope may outlive the variables stored in in.
pub struct GlobalScope {
    variables: UnsafeCell<Vec<ObjectPointer>>,
}

/// A pointer to a global scope.
///
/// Because a GlobalScope sticks around forever (as modules are not removed
/// until the VM terminates) there is no need for reference counting (e.g. using
/// Arc). A GlobalScopeReference allows one to store many references to a global
/// scope without having to add lifetimes all over the place.
#[derive(Clone, Copy)]
pub struct GlobalScopeReference {
    pointer: *const GlobalScope,
}

impl GlobalScope {
    pub fn new() -> GlobalScope {
        GlobalScope { variables: UnsafeCell::new(Vec::new()) }
    }

    /// Returns a global variable.
    ///
    /// This method will panic when attempting to retrieve a non-existing global
    /// variable.
    pub fn get(&self, index: usize) -> ObjectPointer {
        self.locals()[index]
    }

    /// Sets a global variable.
    pub fn set(&self, index: usize, value: ObjectPointer) {
        if !value.is_permanent() {
            panic!("Only permanent objects can be stored in a global scope");
        }

        let mut locals = self.locals_mut();

        if index >= locals.len() {
            locals.resize(index + 1, ObjectPointer::null());
        }

        locals[index] = value;
    }

    fn locals(&self) -> &Vec<ObjectPointer> {
        unsafe { &*self.variables.get() }
    }

    fn locals_mut(&self) -> &mut Vec<ObjectPointer> {
        unsafe { &mut *self.variables.get() }
    }
}

impl GlobalScopeReference {
    pub fn new(scope: &GlobalScope) -> Self {
        GlobalScopeReference { pointer: scope as *const GlobalScope }
    }
}

impl Deref for GlobalScopeReference {
    type Target = GlobalScope;

    fn deref(&self) -> &GlobalScope {
        unsafe { &*self.pointer }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use object_pointer::ObjectPointer;
    use immix::local_allocator::LocalAllocator;
    use immix::global_allocator::GlobalAllocator;

    mod global_scope {
        use super::*;

        #[test]
        #[should_panic]
        fn test_get_invalid() {
            GlobalScope::new().get(0);
        }

        #[test]
        #[should_panic]
        fn test_set_not_permanent() {
            let scope = GlobalScope::new();
            let mut alloc = LocalAllocator::new(GlobalAllocator::new());
            let pointer = alloc.allocate_empty();

            scope.set(0, pointer);
        }

        #[test]
        fn test_get_set() {
            let scope = GlobalScope::new();

            scope.set(0, ObjectPointer::integer(5));

            assert!(scope.get(0) == ObjectPointer::integer(5));
        }
    }

    mod global_scope_reference {
        use super::*;

        #[test]
        fn test_deref() {
            let scope = GlobalScope::new();

            scope.set(0, ObjectPointer::integer(5));

            let scope_ref = GlobalScopeReference::new(&scope);

            assert!(scope_ref.get(0) == ObjectPointer::integer(5));
        }
    }
}
