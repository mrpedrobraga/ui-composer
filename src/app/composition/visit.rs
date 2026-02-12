//! # Visitor
//!
//! Experimental mdule for interacting with complex ADTs. This is going to replace much ot the repeated code in the codebase.
//!
//! The main traits of this module are [Visit] and [Drive] (and their counterparts [VisitMut] and [DriveMut]).

/// Trait for anything which is a visitor of structures.
pub trait Visitor {
    fn enter<E>(&self, node: &E)
    where
        E: Drive<Self>,
        Self: Sized,
    {
        node.drive_thru(self);
    }

    fn enter_mut<E>(&self, node: &mut E)
    where
        E: DriveMut<Self>,
        Self: Sized,
    {
        node.drive_thru_mut(self);
    }
}

/// Trait for a visitor which can visit a `T`.
pub trait Visit<T> {
    fn visit(&self, node: &T);
}

/// Trait for a visitor which can visit a `T` and mutate it.
pub trait VisitMut<T> {
    fn visit_mut(&self, node: &mut T);
}

/// Trait for structures that can be driven through and, potentially, visited.
pub trait Drive<V: Visitor> {
    fn drive_thru(&self, visitor: &V);
}

/// Trait for structures that can be driven through and, potentially, visited mutably.
pub trait DriveMut<V: Visitor> {
    fn drive_thru_mut(&mut self, visitor: &V);
}

#[test]
fn test_visit() {
    struct Logger {}

    impl Visitor for Logger {}
    impl Visit<i32> for Logger {
        fn visit(&self, node: &i32) {
            println!("Found a {}", node);
        }
    }

    impl<V, A, B> Drive<V> for (A, B)
    where
        A: Drive<V>,
        B: Drive<V>,
        V: Visitor,
    {
        fn drive_thru(&self, visitor: &V) {
            self.0.drive_thru(visitor);
            self.1.drive_thru(visitor);
        }
    }
    impl<V> Drive<V> for i32
    where
        V: Visitor + Visit<i32>,
    {
        fn drive_thru(&self, visitor: &V) {
            visitor.visit(self);
        }
    }

    let structure = (1, (2, 3));
    let log = Logger {};
    log.enter(&structure);
}

#[test]
fn test_visit_mut() {
    struct Incrementor {}

    impl Visitor for Incrementor {}
    impl VisitMut<i32> for Incrementor {
        fn visit_mut(&self, node: &mut i32) {
            *node += 1;
        }
    }
    impl<V, A, B> DriveMut<V> for (A, B)
    where
        A: DriveMut<V>,
        B: DriveMut<V>,
        V: Visitor,
    {
        fn drive_thru_mut(&mut self, visitor: &V) {
            self.0.drive_thru_mut(visitor);
            self.1.drive_thru_mut(visitor);
        }
    }
    impl<V> DriveMut<V> for i32
    where
        V: Visitor + VisitMut<i32>,
    {
        fn drive_thru_mut(&mut self, visitor: &V) {
            visitor.visit_mut(self);
        }
    }

    let mut structure = (1, (2, 3));
    let inc = Incrementor {};
    inc.enter_mut(&mut structure);
    assert_eq!(structure, (2, (3, 4)));
}
