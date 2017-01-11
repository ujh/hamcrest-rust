// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest;

mod regex {

    use hamcrest::prelude::*;
    #[test]
    fn successful_match() {
        assert_that!("123", matches_regex(r"^\d+$"));
    }

    #[test]
    fn successful_negative_match() {
        assert_that!("abc", does_not(matches_regex(r"\d")));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_match() {
        assert_that!("abc", matches_regex(r"\d"));
    }

}
