// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton
// Copyright 2015 Carl Lerche
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

pub type MatchResult = Result<(), String>;

pub fn success() -> MatchResult {
    Ok(())
}

pub fn expect(predicate: bool, msg: String) -> MatchResult {
    if predicate {
        success()
    }
    else {
        Err(msg)
    }
}

pub fn assert_that<T, U: Matcher<T>>(actual: T, matcher: U) {
    match matcher.matches(actual) {
        Ok(_) => return,
        Err(mismatch) => {
            panic!("\nExpected: {}\n    but: {}", matcher, mismatch);
        }
    }
}

pub trait Matcher<T>: fmt::Display {
    fn matches(&self, actual: T) -> MatchResult;
}
