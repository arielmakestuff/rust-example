// -*- coding: utf-8 -*-
// src/tmp.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Externs
extern crate unicode_segmentation;

// Stdlib imports
use std::collections::HashSet;
use std::io;
use std::io::Write;

// Third-party imports
use unicode_segmentation::UnicodeSegmentation;

// Local imports


// ===========================================================================
//
// ===========================================================================


fn piglatin(word: &str) -> String {
    let vowels = ["a", "e", "i", "o", "u"].iter().cloned()
        .collect::<HashSet<&str>>();
    let chars = word.graphemes(true).collect::<Vec<&str>>();
    let mut ret: String;

    if vowels.contains(chars[0].to_lowercase().as_str()) {
        ret = String::from(word);
        ret.push_str("-hay");
    } else {
        ret = chars[1..].iter().cloned().collect::<String>().to_lowercase();
        ret = format!("{}-{}ay", &ret, chars[0].to_lowercase());
    }
    ret
}


fn main() {
    println!("Pig-latin translator");
    println!("Press ENTER to exit.");
    println!();

    let exit_chars = ["\n", "\r\n", "\r"].iter().cloned()
        .collect::<HashSet<&str>>();

    // Read input
    loop {
        print!("Phrase: ");
        io::stdout().flush().ok().expect("Could not flush stdout");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        if exit_chars.contains(input.as_str()) {
            break;
        }

        // Trim input
        let input = input.trim();

        let words = input.unicode_words().collect::<Vec<&str>>();
        let mut translation = String::new();
        for &word in words.iter() {
            translation.push_str(&(piglatin(&word) + " "));
        }

        println!("Translation: {}", translation.trim());
        println!();
    }
}


// ===========================================================================
//
// ===========================================================================
