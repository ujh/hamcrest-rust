// Copyright 2014 Carl Lerche, Alex Crichton, Michael Gehring, Yehuda Katz
// Copyright 2015 Carl Lerche, Alex Crichton, Graham Dennis, Tamir Duberstein,
//                Robin Gloster
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::marker::PhantomData;

use {success, Matcher, MatchResult};

pub struct IsNone<T> {
    marker: PhantomData<T>,
}

impl<T> fmt::Display for IsNone<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "none")
    }
}

impl<T: fmt::Debug> Matcher<Option<T>> for IsNone<T> {
    fn matches(&self, actual: Option<T>) -> MatchResult {
        match actual {
            Some(s) => Err(format!("was Some({:?})", s)),
            None => success(),
        }
    }
}

pub fn none<T>() -> IsNone<T> {
    IsNone { marker: PhantomData }
}

#[cfg(test)]
mod test {
    use {assert_that,is,is_not,none};

    #[test]
    fn none_is_none() {
        assert_that(None, is(none::<i8>()));
    }

    #[test]
    fn some_is_not_none() {
        assert_that(Some(1), is_not(none()));
    }
}
