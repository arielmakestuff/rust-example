// -*- coding: utf-8 -*-
// /home/smokybobo/src/me/rust-learning/book2/8.3/intlist/src/tmp.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================


// Modules
pub mod util;

// Externs
extern crate rand;

// Stdlib imports
use std::collections::HashMap;

// Third-party imports
use rand::distributions::{IndependentSample, Range};

// Local imports
use util::VecStr;

// ===========================================================================
//
// ===========================================================================


fn intlist() -> Vec<usize> {
    let mut ilist: Vec<usize> = Vec::new();
    let mut rgen = rand::thread_rng();
    let numrange = Range::new(0, 10);
    for _ in 0..10 {
        let newnum = numrange.ind_sample(&mut rgen);
        ilist.push(newnum);
    }
    ilist
}


fn main() {
    let mut random_numbers = intlist();
    println!("Random numbers: {}", VecStr(&random_numbers));

    random_numbers.sort();
    let mut sum = 0;
    let mut counts: HashMap<usize, usize> = HashMap::new();
    for &num in random_numbers.iter() {
        let key = counts.entry(num).or_insert(0);
        *key += 1;
        sum = sum + num;
    }

    // Average
    let len = random_numbers.len();
    let mean = sum as f64 / len as f64;
    println!("Average: {}", mean);

    // Median
    let med: f64 = match len % 2 {
        0 => {
            let right = len / 2;
            let left = right - 1;
            (random_numbers[left] + random_numbers[right]) as f64 / 2.0
        },

        // For odd numbers, this will result in a .5 float eg 7 / 2 == 3.5
        // However, due to 0-indexing, rounding down will give the correct
        // index number
        _ => random_numbers[len / 2] as f64
    };
    println!("Median: {}", med);

    // Mode
    let mut sorted_numbers: Vec<(&usize, &usize)> = counts.iter().collect();
    sorted_numbers.sort_by(|el1, el2| el2.1.cmp(el1.1));  // Sort in reverse order
    let (val, count) = sorted_numbers[0];
    println!("Most frequent value: {} appeared {} times", val, count);
}
