// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.

use std::sync::mpsc::SyncSender;
use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new_ten_ele_even_queue() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc: Arc<Queue> = Arc::new(q);
    let qc1: Arc<Queue> = Arc::clone(&qc);
    let qc2: Arc<Queue> = Arc::clone(&qc);


    let tx1: mpsc::Sender<u32> = tx.clone();
    
    // the following line of the for loop in original and adapted implementations both
    // takes into considertation to NOT take ownership of the inner value wrapped by
    // the smart pointer types like Rc, Arc, Ref (the return of RefCell.borrow()). It 
    // would be a pitfall to do that since the intended use of such smart pointer types
    // is to work with a reference of the inner value provided via the machinery of Deref
    // trait implemented by these smart pointer type. Thus, attempting to take ownership of 
    // the inner value when what's available is a reference to it is wrong, and would
    // result in E507 error: https://doc.rust-lang.org/error_codes/E0507.html

    // thread::spawn(move || {
    //     for val in &qc1.first_half {
    //         tx1.send(*val).unwrap();
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // });

    thread::spawn(move || {
        for val in qc1.first_half.iter() {
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });
}

pub mod mpscd {
    // illustrative implementation of std::sync::mpsc

    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::sync::Arc;
    use std::sync::Condvar;
    use std::mem;

    struct Inner<T> {
        queue: Mutex<VecDeque<T>>, /* backward compatibility*/
        inner_core: Mutex<InnerCore<T>>,
        msg_available: Condvar,
    }
    
    struct InnerCore<T> {
        queue: VecDeque<T>,
        sender_count: usize,
    }
    
    pub struct Sender<T> {
        inner: Arc<Inner<T>>
    }
    
    impl<T> Sender<T> {
        pub fn send(&self, val_to_send: T) 
        where T: Copy /* backward compatibility*/
        {
            {
                let mut inner_queue_guard = self.inner.queue.lock().unwrap();
                inner_queue_guard.push_back(val_to_send);
                drop(inner_queue_guard);
                self.inner.msg_available.notify_one();                
            } /* backward compatibility*/
            
            let mut inner_core_guard = self.inner.inner_core.lock().unwrap();
            inner_core_guard.queue.push_back(val_to_send);
            drop(inner_core_guard);
            self.inner.msg_available.notify_one();
        }
    }
    
    impl<T> Clone for Sender<T> {
        fn clone(&self) -> Sender<T> {
            let mut inner_core_guard = self.inner.inner_core.lock().unwrap();
            inner_core_guard.sender_count += 1;
            drop(inner_core_guard);
        
            Sender {
                inner: Arc::clone(&self.inner),
            }
        }
    }
    
    impl<T> Drop for Sender<T> {
        fn drop(&mut self) {
            let mut inner_core_guard = self.inner.inner_core.lock().unwrap();
            let dropped_last: bool = inner_core_guard.sender_count == 1;
            inner_core_guard.sender_count -= 1;
            drop(inner_core_guard);
            
            if dropped_last {
                self.inner.msg_available.notify_one();
            }
        }
    }
    
    // an optimisation on Receiver implementation, made available by the fact that
    // the mpsc model only have one receiver, concerns with reducing the contention
    // on the lock of Mutex to the msg queue by minimise the Receiver's demand of
    // that lock to perform receive action. Specifically, instead of naively acquire
    // the lock to perform each receive action, the Receiver would take a whole copy
    // of msg queue to store it locally s.t. more msgs can be pulled from the local
    // copy queue without asking for the lock, relying on the given uniqueness of the 
    // Receiver s.t. it receive action would take place exclusive by itself, hence
    // reducing the frequency by it to require the lock to the shared queue
    pub struct Receiver<T> {
        inner: Arc<Inner<T>>,
        local_copy_queue: VecDeque<T>,
    }
    
    impl<T> Receiver<T> {
        
        // non-thread-blocking receive
        pub fn try_recv(&self) -> Option<T> {
            let mut inner_queue_guard = self.inner.queue.lock().unwrap();
            inner_queue_guard.pop_front()
        }
        
        // naive thread-blocking receive, stuck in loop when no
        // sender is left and no more msg to be received in future
        pub fn naive_recv(&self) -> T {
            let mut inner_queue_guard = self.inner.queue.lock().unwrap();
            loop {
                match inner_queue_guard.pop_front() {
                    None => {
                        inner_queue_guard = self.inner.msg_available.wait(inner_queue_guard).unwrap();
                    },
                    Some(val_recvd) => {
                        return val_recvd;
                    }
                }
            }
        }
        
        pub fn recv(&mut self) -> Option<T> {
            if let Some(val_recvd) = self.local_copy_queue.pop_front() {
                return Some(val_recvd);
            }
            
            // acquire lock to shared inner core for its queue only if local queue is empty
            let mut inner_core_guard = self.inner.inner_core.lock().unwrap();
            loop {
                match inner_core_guard.queue.pop_front() {
                    Some(val_recvd) => {
                        // swap the whole shared queue with the local queue, which
                        // is certain to be an empty one at this point by assumption,
                        // to obtain all msgs accumulated in the shared queue since
                        // the shared queue was fetched last time
                        if !inner_core_guard.queue.is_empty() {
                            // functionally equivalent but ineffient in terms of memory use
                            // self.local_copy_queue = mem::take(&mut inner_core_guard.queue);
                            
                            mem::swap(&mut inner_core_guard.queue, &mut self.local_copy_queue);
                        }
                        return Some(val_recvd);
                    },
                    // when no new msg is received, there are two cases: on one hand if there are
                    // senders present, the thread-blocking action to wait for future msg to receive
                    // needs to take place, and on the other hand, immediately return if there
                    // is no sender present and therefore certain that no more msg to receive
                    None if inner_core_guard.sender_count == 0 => {
                        return None;            
                    },
                    None => {
                        inner_core_guard = self.inner.msg_available.wait(inner_core_guard).unwrap();
                    }
                }            
            }
                
        }
    }
    
    // cheap derivation of Iterator functionality given the exisint Receiver setup
    // to enhance the ergonomics the API to receive multiple messages
    impl<T> Iterator for Receiver<T> {
        type Item = T;
        
        fn next(&mut self) -> Option<<Self as Iterator>::Item> {
          self.recv()
        }
    }

    pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    
        let new_inner_core = Mutex::new(InnerCore {
            queue: VecDeque::<T>::new(),
            sender_count: 1,
        });

        let new_inner = Arc::new(Inner {
            queue: Mutex::new(VecDeque::<T>::new()),
            inner_core: new_inner_core,
            msg_available: Condvar::new(),
        });
        
        let sender = Sender {inner: Arc::clone(&new_inner)};
        let receiver = Receiver {
            inner: Arc::clone(&new_inner),
            local_copy_queue: VecDeque::new(),
        };
        
        (sender, receiver)
    }
}

// #[test]
// fn main() {

#[cfg(test)]
mod tests {
    use super::*;

    // mpsc example: default asynchronous channel, and its sender, receiver using regular send(), receive() API
    #[test]
    fn send_queue_elements_mpsc_async_channel() {
        let (tx, rx) = mpsc::channel();
        let test_queue = Queue::new_ten_ele_even_queue();
        let queue_length = test_queue.length;
    
        send_tx(test_queue, tx);
    
        let mut total_received: u32 = 0;
        for received in rx {
            println!("Got: {}", received);
            total_received += 1;
        }
    
        println!("total numbers received: {}", total_received);
        assert_eq!(total_received, queue_length)        
    }

    // mpsc example: synchronous channel s.t. the main diff with async channel is that
    #[test]
    fn mpsc_sync_channel_blocking_send() {
        use std::sync::mpsc::sync_channel;
        // use std::thread;
        use std::time::Duration;
        
        let (original_sender, receiver) = sync_channel(2);
        let sender_2: SyncSender<i32> = original_sender.clone();
        
        thread::spawn(move|| {
            original_sender.send(42).unwrap();
            original_sender.send(66).unwrap();
        });
        
        thread::spawn(move || {
           sender_2.send(69).unwrap();
           
           println!("Returning to this point takes a while which shows that the sender's
           send above is delayed since receiving from the channel is delayed in 
           the main thread causing the buffer to be full and hence blocking the send");       
        });
            
        thread::sleep(Duration::from_secs(1));
        
        let mut recv_val;
        recv_val = receiver.recv().unwrap();
        assert_eq!(recv_val, 42);
        recv_val = receiver.recv().unwrap();
        assert_eq!(recv_val, 66);
        recv_val = receiver.recv().unwrap();
        assert_eq!(recv_val, 69);
    }

    mod mpscd_tests {
        use std::collections::HashSet;
        use super::*;
        
        #[test]
        fn basic_send_and_non_blocking_receive() {
            let (tx, rx) = mpscd::channel();
            
            let handle = thread::spawn(move || {
                tx.send(42);    
            });
            
            handle.join().unwrap();
            
            let recv_val = rx.try_recv();
            assert_eq!(recv_val, Some(42));
            
            let recv_val_2 = rx.try_recv();
            assert!(recv_val_2.is_none());
        }
        
        #[test]
        fn send_and_naive_blocking_receive() {
            let (tx, rx) = mpscd::channel();
            
            thread::spawn(move || {
                tx.send(42);    
            });
            
            let recv_val = rx.naive_recv();
            assert_eq!(recv_val, 42);
            
            // the following would make the execution hang
            // let _ = rx.naive_recv();
        }
        
        #[test]
        fn send_and_blocking_receive() {
            let (tx, mut rx) = mpscd::channel();
            
            thread::spawn(move || {
                let tx_cloned = tx.clone();
                
                tx.send(42);
                tx_cloned.send(66);
            });
            
            let recv_val = rx.recv();
            assert_eq!(recv_val, Some(42));
            
            let recv_val_2 = rx.recv();
            assert_eq!(recv_val_2, Some(66));
            
            assert!(rx.recv().is_none());
        }
        
        #[test]
        fn iterator_supported_receiver() {
            let (tx, rx) = mpscd::channel();
            
            thread::spawn(move || {
                tx.send(42);
                tx.send(66);
                tx.send(69);            
            });
            
            let sent_val_set = HashSet::from([42, 66, 69]);
            
            for ref recv_val in rx {
                assert!(sent_val_set.contains(recv_val));
            }
        }
    }    
}