//! # Visitor
//!
//! Experimental mdule for interacting with complex ADTs. This is going to replace much ot the repeated code in the codebase.
//!
//! The main traits of this module are [Visit] and [Drive] (and their counterparts [VisitMut] and [DriveMut]).

/// Trait for a visitor which can visit a `T`.
pub trait Apply<T> {
    fn visit(&mut self, node: &T);
}

/// Trait for a visitor which can visit a `T` and mutate it.
pub trait ApplyMut<T> {
    fn visit_mut(&mut self, node: &mut T);
}

/// Trait for structures that can be driven through and, potentially, visited.
pub trait DriveThru<V> {
    fn drive_thru(&self, visitor: &mut V);
}

/// Trait for structures that can be driven through and, potentially, visited mutably.
pub trait DriveThruMut<V> {
    fn drive_thru_mut(&mut self, visitor: &mut V);
}

pub mod implementations {
    use super::{DriveThru, DriveThruMut};

    impl<V> DriveThru<V> for () {
        fn drive_thru(&self, _: &mut V) {
            /* Nothing to visit!  */
        }
    }

    impl<V> DriveThruMut<V> for () {
        fn drive_thru_mut(&mut self, _: &mut V) {
            /* Nothing to visit!  */
        }
    }

    impl<V, A, B> DriveThru<V> for (A, B)
    where
        A: DriveThru<V>,
        B: DriveThru<V>,
    {
        fn drive_thru(&self, visitor: &mut V) {
            self.0.drive_thru(visitor);
            self.1.drive_thru(visitor);
        }
    }

    impl<V, A, B> DriveThruMut<V> for (A, B)
    where
        A: DriveThruMut<V>,
        B: DriveThruMut<V>,
    {
        fn drive_thru_mut(&mut self, visitor: &mut V) {
            self.0.drive_thru_mut(visitor);
            self.1.drive_thru_mut(visitor);
        }
    }

    impl<T, V> DriveThru<V> for Vec<T>
    where
        T: DriveThru<V>,
    {
        fn drive_thru(&self, visitor: &mut V) {
            for item in self {
                item.drive_thru(visitor);
            }
        }
    }

    impl<T, V> DriveThruMut<V> for Vec<T>
    where
        T: DriveThruMut<V>,
    {
        fn drive_thru_mut(&mut self, visitor: &mut V) {
            for item in self {
                item.drive_thru_mut(visitor);
            }
        }
    }

    impl<V, A> DriveThru<V> for Option<A>
    where
        A: DriveThru<V>,
    {
        fn drive_thru(&self, visitor: &mut V) {
            if let Some(inner) = self {
                inner.drive_thru(visitor);
            }
        }
    }

    impl<V, A> DriveThruMut<V> for Option<A>
    where
        A: DriveThruMut<V>,
    {
        fn drive_thru_mut(&mut self, visitor: &mut V) {
            if let Some(inner) = self {
                inner.drive_thru_mut(visitor);
            }
        }
    }
}

#[test]
fn test_visit() {
    struct Logger {
        logs: Vec<String>,
    }

    impl Apply<i32> for Logger {
        fn visit(&mut self, node: &i32) {
            self.logs.push(format!("Found a {}", node));
        }
    }

    #[allow(non_local_definitions)]
    impl<V> DriveThru<V> for i32
    where
        V: Apply<i32>,
    {
        fn drive_thru(&self, visitor: &mut V) {
            visitor.visit(self);
        }
    }

    let structure = (1, (2, 3));
    let mut log = Logger { logs: Vec::new() };
    structure.drive_thru(&mut log);
}

#[test]
fn test_visit_mut() {
    struct Incrementor {}

    impl ApplyMut<i32> for Incrementor {
        fn visit_mut(&mut self, node: &mut i32) {
            *node += 1;
        }
    }

    #[allow(non_local_definitions)]
    impl<V> DriveThruMut<V> for i32
    where
        V: ApplyMut<i32>,
    {
        fn drive_thru_mut(&mut self, visitor: &mut V) {
            visitor.visit_mut(self);
        }
    }

    let mut structure = (1, (2, 3));
    let mut inc = Incrementor {};
    structure.drive_thru_mut(&mut inc);
    assert_eq!(structure, (2, (3, 4)));
}
