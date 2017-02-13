// -*- coding: utf-8 -*-
// src/tmp.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Externs
extern crate num;

// Stdlib imports
use std::collections::HashMap;
use std::io;

// Third-party imports
use num::bigint::BigUint;
use num::traits::{Zero, One};
use std::mem::replace;

// Local imports


// ===========================================================================
//
// ===========================================================================


// Naive implementation
fn smallfib(cache: &mut HashMap<usize, usize>, i: usize) -> usize {
    // Compute val
    match cache.get(&i) {
        Some(&val) => val,
        None if i <= 1 => {
            cache.insert(i, i);
            i
        },
        None => {
            let ret = smallfib(cache, i-2) + smallfib(cache, i-1);
            cache.insert(i, ret);
            ret
        },
    }
}


// Calculate large fibonacci numbers.
fn fib(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        // This is a low cost way of swapping f0 with f1 and f1 with f2.
        let f2 = f0 + &f1;
        f0 = replace(&mut f1, f2);
    }
    f0
}


// Calculate large fibonacci numbers.
fn fib2(n: usize) -> BigUint {
    let mut f0: BigUint = Zero::zero();
    let mut f1: BigUint = One::one();
    for _ in 0..n {
        let f2 = f0 + &f1;
        f0 = f1;
        f1 = f2;
    }
    f0
}


fn main() {
    println!("Fibonacci calculator");
    println!("n:");

    // let mut store = HashMap::new();

    loop {
        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        // Exit if "q" received
        if input.trim() == "q" {
            break;
        }

        // Convert input to usize
        let n: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // let ret: usize = fib(&mut store, n);
        // println!("Fib({}) = {}", n, ret);

        let ret: BigUint = fib2(n);
        println!("Fib({}) = {}", n, ret)
    }

}


// ===========================================================================
//
// ===========================================================================
