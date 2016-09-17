// Copyright 2014 Carl Lerche, Yehuda Katz, Steve Klabnik, Alex Crichton,
//                Ben Longbons
// Copyright 2015 Carl Lerche, Graham Dennis, Alex Crichton, Tamir Duberstein,
//                Robin Gloster
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::vec::Vec;
use {success, Matcher, MatchResult};

#[derive(Clone,Copy)]
pub struct OfLen {
    len: usize
}

impl fmt::Display for OfLen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "of len {}", self.len)
    }
}

impl<'a, T> Matcher<&'a Vec<T>> for OfLen {
    fn matches(&self, actual: &Vec<T>) -> MatchResult {
        if self.len == actual.len() {
            success()
        } else {
            Err(format!("was len {}", actual.len()))
        }
    }
}

pub fn of_len(len: usize) -> OfLen {
    OfLen { len: len }
}

#[derive(Clone)]
pub struct Contains<T> {
    items: Vec<T>,
    exactly: bool
}

impl<T> Contains<T> {
    pub fn exactly(mut self) -> Contains<T> {
        self.exactly = true;
        self
    }
}

impl<T: fmt::Debug> fmt::Display for Contains<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.exactly {
            write!(f, "containing exactly {}", Pretty(&self.items))
        } else {
            write!(f, "containing {}", Pretty(&self.items))
        }
    }
}

impl<'a, T: fmt::Debug + PartialEq + Clone> Matcher<&'a Vec<T>> for Contains<T> {
    fn matches(&self, actual: &Vec<T>) -> MatchResult {
        let mut rem = actual.clone();

        for item in self.items.iter() {
            match rem.iter().position(|a| *item == *a) {
                Some(idx) => { rem.remove(idx); },
                None => return Err(format!("was {}", Pretty(&actual)))
            }
        }

        if self.exactly && !rem.is_empty() {
            return Err(format!("also had {}", Pretty(&rem)));
        }

        success()
    }
}

pub fn contains<T>(items: Vec<T>) -> Contains<T> {
    Contains { items: items, exactly: false }
}

struct Pretty<'a, T: 'a>(&'a [T]);

impl<'a, T: fmt::Debug> fmt::Display for Pretty<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "["));
        for (i, t) in self.0.iter().enumerate() {
            if i != 0 { try!(write!(f, ", ")); }
            try!(write!(f, "{:?}", t));
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod test {
    use {
        not,
        assert_that,
        contains,
        of_len,
        is
    };

    #[test]
    fn vec_contains() {
        assert_that(&vec!(1, 2, 3), contains(vec!(1, 2)));
        assert_that(&vec!(1, 2, 3), not(contains(vec!(4))));
    }

    #[test]
    fn vec_contains_exactly() {
        assert_that(&vec!(1, 2, 3), contains(vec!(1, 2, 3)).exactly());
        assert_that(&vec!(1, 2, 3), not(contains(vec!(1, 2)).exactly()));
    }

    #[test]
    fn vec_of_len() {
        assert_that(&vec!(1, 2, 3), of_len(3));
        assert_that(&vec!(1, 2, 3), is(of_len(3)));
    }
}
