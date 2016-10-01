// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use Matcher;
use MatchResult;
use regex::Regex;
use std::fmt;
use success;

pub struct MatchesRegex {
    regex: Regex
}

impl fmt::Display for MatchesRegex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.regex.fmt(f)
    }
}

impl<'a> Matcher<&'a str> for MatchesRegex {
    fn matches(&self, actual: &'a str) -> MatchResult {
        if self.regex.is_match(actual) {
            success()
        }
        else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn matches_regex(regex: &str) -> MatchesRegex {
    MatchesRegex { regex: Regex::new(regex).unwrap() }
}

#[cfg(test)]
mod test {
    use assert_that;
    use matches_regex;
    use does_not;

    #[test]
    fn succesful_match() {
        assert_that("123", matches_regex(r"^\d+$"));
    }

    #[test]
    fn successful_negative_match() {
        assert_that("abc", does_not(matches_regex(r"\d")));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_match() {
        assert_that("abc", matches_regex(r"\d"));
    }

}
