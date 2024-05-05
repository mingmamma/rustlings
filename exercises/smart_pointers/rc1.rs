// rc1.rs
//
// In this exercise, we want to express the concept of multiple owners via the
// Rc<T> type. This is a model of our solar system - there is a Sun type and
// multiple Planets. The Planets take ownership of the sun, indicating that they
// revolve around the sun.
//
// Make this code compile by using the proper Rc primitives to express that the
// sun has multiple owners.
//
// Execute `rustlings hint rc1` or use the `hint` watch subcommand for a hint.

use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::RefMut;

#[derive(Debug)]
struct Sun {}

#[derive(Debug)]
enum Planet {
    Mercury(Rc<Sun>),
    Venus(Rc<Sun>),
    Earth(Rc<Sun>),
    Mars(Rc<Sun>),
    Jupiter(Rc<Sun>),
    Saturn(Rc<Sun>),
    Uranus(Rc<Sun>),
    Neptune(Rc<Sun>),
}

impl Planet {
    fn details(&self) {
        println!("Hi from {:?}!", self)
    }
}

pub mod rc {
    // illustrative minimal viable implementation of std::rc::Rc
    // https://www.youtube.com/watch?v=yOezcP-XaIw&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=14&t=580s 
    use std::ops::Deref;
    
    struct RcInner<T> {
        value: T,
        rc_count: usize
    }

    pub struct RcPtr<T> {
        inner: *mut RcInner<T>,
    }
    
    
    impl<T> RcPtr<T> {
        pub fn new(value: T) -> RcPtr::<T> {
            
            let boxed_inner = Box::new(RcInner {
                value: value,
                rc_count: 1,
            });
            
            RcPtr::<T> {
              inner: Box::into_raw(boxed_inner),
            }
        }
        
        pub fn strong_count(&self) -> usize {
            // todo!()
            let ref_inner = unsafe {& *self.inner};
            
            ref_inner.rc_count
        }
    }
    
    impl<T> Clone for RcPtr<T> {
        fn clone(&self) -> Self {

            let ref_mut_inner_counter: &mut usize = &mut unsafe {&mut *self.inner}.rc_count;
            *ref_mut_inner_counter += 1;
            
            Self {
                inner: self.inner
            }
        }
    }
    
    impl<T> Deref for RcPtr<T> {
        type Target = T;
        
        fn deref(&self) -> &<Self as Deref>::Target {
            let ref_inner = unsafe {& *self.inner};
            
            &ref_inner.value
        }
    }
    
    impl<T> Drop for RcPtr<T> {
        fn drop(&mut self) {
            let ref_mut_inner_counter: &mut usize = &mut unsafe {&mut *self.inner}.rc_count;
            
            if *ref_mut_inner_counter == 1 {
                let _ = unsafe {
                    Box::from_raw(self.inner)
                };
            } else {
                *ref_mut_inner_counter -= 1;
            }
            
        }
    }
}

struct DatasetMan<'a> {
    population_data: Rc<RefCell<HashMap<&'a str, usize>>>
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn solar_system_test() {
        
        let sun = Rc::new(Sun {});
        assert_eq!(Rc::strong_count(&sun), 1);
    
        let mercury = Planet::Mercury(Rc::clone(&sun));
        mercury.details();
    
        let venus = Planet::Venus(Rc::clone(&sun));
        venus.details();
    
        let earth = Planet::Earth(Rc::clone(&sun));
        earth.details();
    
        let mars = Planet::Mars(Rc::clone(&sun));
        mars.details();
    
        let jupiter = Planet::Jupiter(Rc::clone(&sun));
        jupiter.details();
    
        let saturn = Planet::Saturn(Rc::clone(&sun));
        saturn.details();
    
        let uranus = Planet::Uranus(Rc::clone(&sun));
        uranus.details();
        
        let neptune = Planet::Neptune(Rc::clone(&sun));
        neptune.details();
        
        assert_eq!(Rc::strong_count(&sun), 9);
        
        drop(neptune);
        drop(uranus);
        drop(saturn);
        drop(jupiter);
        drop(mars);
        drop(earth);
        drop(venus);
        drop(mercury);
        assert_eq!(Rc::strong_count(&sun), 1);
    }

    #[test]
    fn rc_interior_mutability_test() {
        let shared_data: Rc<RefCell<HashMap<&str, usize>>> = Rc::new(RefCell::new(HashMap::new()));
        let manager_1 = DatasetMan {population_data: Rc::clone(&shared_data)};
        let manager_2 = DatasetMan {population_data: Rc::clone(&shared_data)};

        {
            let mut shared_hashmap: RefMut<'_, _> = shared_data.borrow_mut();
        
            shared_hashmap.insert("kyoto", 1_449_008);
            shared_hashmap.insert("tallinn", 434_562);
        }

        {
            let total_population_from_shared_data = shared_data.borrow().values().sum::<usize>();
            let total_population_from_manager_1: usize = manager_1.population_data.borrow().values().sum();
            let total_population_from_manager_2: usize = manager_2.population_data.borrow().values().sum();
            
            assert_eq!(total_population_from_shared_data, total_population_from_manager_1);
            assert_eq!(total_population_from_manager_1, total_population_from_manager_2);
        }
    }

    mod rc_tests {
        use super::rc::*;
        
        #[test]
        fn rc_tests() {
            let test_rc = RcPtr::new(1);
            
            assert_eq!(*test_rc, 1);
            assert_eq!(test_rc.strong_count(), 1);
            {
                let cloned_rc = RcPtr::clone(&test_rc);
                
                assert_eq!(*cloned_rc, 1);
                assert_eq!(cloned_rc.strong_count(), 2);
                
                assert_eq!(*test_rc, 1);
                assert_eq!(test_rc.strong_count(), 2);
               
            }
            
            assert_eq!(test_rc.strong_count(), 1); 
        }
    }    
}