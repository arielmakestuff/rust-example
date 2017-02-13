// -*- coding: utf-8 -*-
// /home/smokybobo/src/me/rust-learning/book2/8.3/intlist/src/util/mod.rs
// Copyright (C) 2017 authors and contributors (see AUTHORS file)
//
// This file is released under the MIT License.

// ===========================================================================
// Imports
// ===========================================================================

// Externs

// Stdlib imports
use std::fmt::{Display, Formatter, Error};

// Third-party imports

// Local imports


// ===========================================================================
//
// ===========================================================================


pub struct VecStr<'a,T: 'a>(pub &'a Vec<T>);


impl<'a,T> Display for VecStr<'a,T>
    where T: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let mut comma_separated = String::new();

        for num in &self.0[0..self.0.len() - 1] {
            comma_separated.push_str(&num.to_string());
            comma_separated.push_str(", ");
        }

        comma_separated.push_str(&self.0[self.0.len() - 1].to_string());
        write!(f, "[{}]", comma_separated)
    }
}


// ===========================================================================
//
// ===========================================================================
