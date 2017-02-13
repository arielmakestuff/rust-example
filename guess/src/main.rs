// -*- coding: utf-8 -*-
// src/tmp.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================
// Externs
extern crate rand;

// Stdlib imports
use std::cmp::Ordering;
use std::io;

// Third-party imports
use rand::Rng;

// Local imports


// ===========================================================================
// Main
// ===========================================================================


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert to u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num)     => num,
            Err(_)      => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
            }
        }
    }
}


// ===========================================================================
//
// ===========================================================================
