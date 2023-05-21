#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused)]


use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::{Builder, JoinHandle};

struct Primes {
    limit: usize,
}

impl Primes {
    fn new(limit: usize) -> Self {
        Primes { limit }
    }

    fn iter(&self) -> PrimesIter {
        PrimesIter {
            index: 2,
            computed: compute_primes(self.limit),
        }
    }
}

fn compute_primes(limit: usize) -> Vec<bool> {
    let mut sieve = vec![true; limit];
    let mut m = 2;

    while m * m < limit {
        if sieve[m] {
            for i in (m * 2..limit).step_by(m) {
                sieve[i] = false
            }
        }
        m += 1;
    }

    sieve
}

struct PrimesIter {
    index: usize,
    computed: Vec<bool>,
}

impl Iterator for PrimesIter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.index += 1;
            if self.index > self.computed.len() - 1 {
                return None;
            } else if self.computed[self.index] {
                return Some(self.index);
            } else {
                continue;
            }
        }
    }
}

pub fn get_prime_iterator(){
    let primes =Primes::new(100);
    for i in primes.iter(){
        println!("{}",i)
    }
}


pub fn thread_builder(){
    let mythred = Builder::new().name("worker thread".to_string()).stack_size(1024 * 4);

    let handle =mythred.spawn(||{
       println!("thread is running")
    });

    let status =handle.unwrap().join();

    println!("{:?}",status)

}

pub fn thread_data_access(){
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    let nums = Arc::new(Mutex::new(nums));

    let mut join_handles = Vec::new();
    for n in 0..7 {
        let nums = Arc::clone(&nums);
        join_handles.push(thread::spawn(move || {
            {
                let mut nums = nums.lock().unwrap();
                nums.push(n)

            }
            println!("{:?}", nums.lock().unwrap()[n]);
            nums

        }));
    }

    for handle in join_handles {
        handle.join().unwrap();
    }
    println!("{:?}", nums.lock().unwrap());

}