// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton, Yehuda Katz,
//                Ben Longbons
// Copyright 2015 Carl Lerche, Alex Crichton, Robin Gloster
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use {success,Matcher,MatchResult};

pub struct EqualTo<T> {
    expected: T
}

impl<T: fmt::Debug> fmt::Display for EqualTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl<T : PartialEq + fmt::Debug> Matcher<T> for EqualTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        if self.expected.eq(&actual) {
            success()
        }
        else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn equal_to<T : PartialEq + fmt::Debug>(expected: T) -> EqualTo<T> {
    EqualTo { expected: expected }
}

#[cfg(test)]
mod test {
    use {assert_that,is,equal_to};

    #[test]
    fn equality_of_ints() {
        assert_that(1, is(equal_to(1)));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_match() {
        assert_that(2, is(equal_to(1)));
    }
}
