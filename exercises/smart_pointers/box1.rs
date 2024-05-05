// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: the value of the current item and
// the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Nil))
}

pub mod cell {
    // illustrative implementation of std::cell::Cell
    use std::cell::UnsafeCell;

    pub struct CeLL<T> {
        value: UnsafeCell<T>,
    }
    
    impl<T> CeLL<T> {
        pub fn new(val: T) -> Self {
            CeLL {
                value: UnsafeCell::new(val),
            }
        }

        // public interface of get method only provides for working with
        // values implementing the Copy trait
        pub fn get(&self) -> T where T : Copy {
            unsafe {
                *(self.value.get()) /* the outer parentheses is actually redudant due to operator precedence rule */
            }
        }

        // naive implementation of the public interface of set() method proves difficult
        // since the interface would need to take a immutable reference to CeLL to achieve
        // setting the a new value for the its field, which is disallowed under usual circumstances
        // the UnsafeCell is the go-to underlying building block for implementing data structures that
        // provides interior mutability functionality
        // pub fn set(&self, new_val: T) {
        //     self.value = new_val;
        // }

        pub fn set(&self, new_val: T) {
            unsafe {
                *self.value.get() = new_val;
            }
        }
    }
}

pub mod boguscell {
    // illustrative bogus implementation of std::cell::Cell to give an example of data race problem
    use std::cell::UnsafeCell;
    use std::marker::Sync;

    pub struct BogusCell<T> {
        value: UnsafeCell<T>,
    }
    
    // explicitly implement Sync requires unsafe impl, later to exhibit the data race problem when such
    // Cell construct is used in multiple thread
    unsafe impl<T> Sync for BogusCell<T> {}
    
    // the other API implementation follow the regular example otherwise
    impl<T> BogusCell<T> {
        pub fn new(val: T) -> Self {
            BogusCell {
                value: UnsafeCell::new(val),
            }
        }

        pub fn get(&self) -> T where T: Copy{
            unsafe {
                *(self.value.get())
            }
        }

        pub fn set(&self, new_val: T) {
            unsafe {
                *self.value.get() = new_val;
            }
        }
    }
}

pub mod refcell {
    // illustrative implementation of std::cell::RefCell
    
    use std::cell::Cell;
    use std::cell::UnsafeCell;
    use std::ops::{Deref, DerefMut};
    
    // helper enum type (non-public) for indicating the reference borrow status of an RefceLL value
    #[derive(Copy, Clone)]
    enum BorrowStatus {
        NotBorrowed,
        ShareBorrowed(usize), /* modelling the number of immutable references in place of the RefceLL value */
        ExclusiveBorrowed,
    }
    
    pub struct RefceLL<T> {
        value: UnsafeCell<T>,
        borrow_status: Cell<BorrowStatus>,
    }
    
    impl<T> RefceLL<T> {
        pub fn new(val: T) -> Self {
            Self {
                value: UnsafeCell::new(val),
                borrow_status: Cell::new(BorrowStatus::NotBorrowed),
            }
        }
        
        // Respecting the conventional Rust borrow rules at runtime is a guarantee that the RefceLL type upholds, thus
        // the public interface to return an immutable reference is modelled with Option s.t. the outcome of returning
        // a immutable borrow reference is conditional on previously borrowed live references in place in a manner
        // that adheres to the convenrional Rust borrow semantics
        
        // Additionally, the API of the public borrow interface mandates the parameter type to be an immutable reference to
        // the RefceLL value but the implementation would involve changing the internal borrow status of the RefceLL in response
        // to returning the proper outcome of giving an immutable borrow. This gives the case of interior immutability pattern
        // s.t. the borrow_status field is wrapped in Cell to allow for such use case
        
        // pub fn borrow(&self) -> Option<&T> {
        pub fn borrow(&self) -> Option<Ref<T>> {
            match self.borrow_status.get() {
                BorrowStatus::NotBorrowed => {
                    self.borrow_status.set(BorrowStatus::ShareBorrowed(1));
                    Some(Ref(self))
                },
                BorrowStatus::ShareBorrowed(n) => {
                    self.borrow_status.set(BorrowStatus::ShareBorrowed(n+1));                    
                    Some(Ref(self))
                },
                BorrowStatus::ExclusiveBorrowed => None,
            }
        }
        
        // ditto the reasoning of modelling the return type with Option
        
        // Additionally, the borrow_mut method design contract mandates to take a shared reference to the RefceLL value
        // for returning a mutable reference of the inner value gives the ground of the implementation to wrap the inner value
        // with UnsafeCell to work around the blocking restriction that would arise otherwise
        // pub fn borrow_mut(&self) -> Option<&mut T> {
        pub fn borrow_mut(&self) -> Option<RefMut<T>> {
            match self.borrow_status.get() {
                BorrowStatus::NotBorrowed => {
                    self.borrow_status.set(BorrowStatus::ExclusiveBorrowed);
                    Some(RefMut(self))
                },
                BorrowStatus::ShareBorrowed(_) => None,
                BorrowStatus::ExclusiveBorrowed => None,
            }
        }
    }
    
    // the custom type for modelling a immutable reference to a RefceLL value that comes with smart pointer functionality and
    // custom Drop trait logic. Since value of this type acts like an immutable reference to a RefceLL to the public, the most
    // straightforward way is to model the struct with as wrapper with just a field of an immutable reference to the actual RefceLL
    pub struct Ref<'a, T> (&'a RefceLL<T>);
    
    impl<'a, T> Deref for Ref<'a, T> {
        type Target = T;
        
        fn deref(&self) -> &<Self as Deref>::Target {
            unsafe {&*self.0.value.get()}
        }
    }
    
    impl<'a, T> Drop for Ref<'a, T> {
        fn drop(&mut self) {
            match self.0.borrow_status.get() {
                BorrowStatus::ShareBorrowed(1) => {
                    self.0.borrow_status.set(BorrowStatus::NotBorrowed);
                },
                BorrowStatus::ShareBorrowed(n) if n > 1 => {
                    self.0.borrow_status.set(BorrowStatus::ShareBorrowed(n-1));
                },
                _ => unreachable!() /* BorrowStatus::NotBorrowed | BorrowStatus::ExclusiveBorrowed */
                
            }
        }
    }
    
    pub struct RefMut<'a, T> (&'a RefceLL<T>);
    
    impl<'a, T> Deref for RefMut<'a, T> {
        type Target = T;
        
        fn deref(&self) -> &<Self as Deref>::Target {
            unsafe {&*self.0.value.get()}
        }
    }
    
    impl<'a, T> DerefMut for RefMut<'a, T> {
        fn deref_mut(&mut self) -> &mut <Self as Deref>::Target {
            unsafe {&mut *self.0.value.get()}
        }
    }
    
    impl<'a, T> Drop for RefMut<'a, T> {
        fn drop(&mut self) {
            match self.0.borrow_status.get() {
                BorrowStatus::NotBorrowed => unreachable!(),
                BorrowStatus::ShareBorrowed(_) => unreachable!(),
                BorrowStatus::ExclusiveBorrowed => {
                    // todo!()
                    self.0.borrow_status.set(BorrowStatus::NotBorrowed)
                }
            }
        }
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }

    mod cell_test {
        use crate::cell::*;
        #[test]
        fn cell_basic_suite() {

            let new_cell = CeLL::new(1);
            assert_eq!(new_cell.get(), 1);
            new_cell.set(10);
            assert_eq!(new_cell.get(), 10);
        }

    }

    mod bogus_cell_tests{
        use std::thread;
        use std::sync::Arc;
        use super::boguscell::*;
        
        #[test]
        #[should_panic]
        fn data_race_suite() {
            let share_owned_new_cell: Arc<BogusCell<i32>> = Arc::new(BogusCell::new(0));
            let mut handles = vec![];
            {
                let share_cloned_1: Arc<BogusCell<i32>> = Arc::clone(&share_owned_new_cell);
                let thread_handle_1 = thread::spawn(move || -> () {
                    // the repetition needs to be pretty BIG to reliably cause data race :)
                    for _ in 0..1000000 {
                        share_cloned_1.set(share_cloned_1.get()+1);
                    }
                });
                
                let share_cloned_2: Arc<BogusCell<i32>> = Arc::clone(&share_owned_new_cell);
                let thread_handle_2 = thread::spawn(move || -> () {
                    for _ in 0..1000000 {
                        share_cloned_2.set(share_cloned_2.get()+1);
                    }
                });
                
                handles.push(thread_handle_1);
                handles.push(thread_handle_2);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
            
            let result = share_owned_new_cell.get();
            
            // this is bound to fail the expectation that starting the BogusCell wrapped counter at 0 and have each thread
            // increment the inner counter value 1000 times would give the final value 2000. The data race problem arise
            // from the cause that the two threads may read the same value and make increment based on that, resulting in
            // a the later commiting thread to overwrite the result without effectively incrementing by two, hence the final
            // value of the counter falling short of the expected value
            assert_eq!(result, 2000000);
        }
    }

    mod refcell_tests {
        use super::refcell::*;
        
        #[test]
        fn mutiple_share_borrow() {
            let test_refcell = RefceLL::new(1);
    
            let refcell_borrow_1 = test_refcell.borrow();
            let refcell_borrow_2 = test_refcell.borrow();
            assert_eq!(*refcell_borrow_1.unwrap(), 1);
            
            // Can't not give mutable reference while there is some shared reference that is alive in place
            let refcell_mut_borrow: Option<RefMut<'_, i32>> = test_refcell.borrow_mut();
            assert!(matches!(refcell_mut_borrow, None));
            
            assert_eq!(*refcell_borrow_2.unwrap(), 1);        
        }
        
        #[test]
        fn unique_exclusive_borrow() {
            let test_refcell = RefceLL::new(1);
    
            let refcell_borrow_mut_1: Option<RefMut<'_, i32>> = test_refcell.borrow_mut();
            let mut refmut_val: RefMut<'_, i32> = refcell_borrow_mut_1.unwrap();
            *refmut_val = *refmut_val + 1;
            assert_eq!(*refmut_val, 2);
            
            // Can't not give mutable or immutable reference while there is a exclusive reference alive in place
            let refcell_borrow_mut_2 = test_refcell.borrow_mut();
            let refcell_borrow = test_refcell.borrow();        
            assert!(matches!(refcell_borrow_mut_2, None));
            assert!(matches!(refcell_borrow, None));        
        }
        
        #[test]
        fn borrowed_reference_scopes() {
            let test_refcell = RefceLL::new(1);
            // an immutable reference created, used, and go out of scope within the following scope s.t the outcome of borrow
            // attempts later should be unaffected, with regard to which this immutable reference is in effect transparent
            // Such goal cannot be achieved with the returned borrow being modelling as Option wrapped plain immutable/mutable
            // reference type, for the lack of machinary to feedback to the RefceLL value from which the borrow orinated to update
            // its borrow status. Hence, a custom type with smart pointer functionality is implemented to use in place of plain
            // reference types such that the feedback machinary can be achieved by implementing the custom Drop trait logic 
            {
                let refcell_borrow_1 = test_refcell.borrow();
                
                let refcell_borrow_mut_1 = test_refcell.borrow_mut();
                assert!(matches!(refcell_borrow_mut_1, None));           
                
                assert_eq!(*refcell_borrow_1.unwrap(), 1);
            }
            {
                let refcell_borrow_mut_1 = test_refcell.borrow_mut();
                assert!(matches!(refcell_borrow_mut_1, Some(_)));
            }
        }
        
    }
}
