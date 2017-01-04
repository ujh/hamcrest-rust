// Copyright 2017 Matt LaChance
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate hamcrest;

mod compared_to {

    use hamcrest::prelude::*;

    #[test]
    fn ints_less_than() {
        assert_that!(4, is(less_than(5)));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_less_than() {
        assert_that!(4, is(less_than(3)));
    }

    #[test]
    #[should_panic]
    fn less_than_is_not_equal() {
        assert_that!(2, is(less_than(2)));
    }

    #[test]
    fn ints_greater_than() {
        assert_that!(8, is(greater_than(5)));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_greater_than() {
        assert_that!(1, is(greater_than(3)));
    }

    #[test]
    #[should_panic]
    fn greater_than_is_not_equal() {
        assert_that!(2, is(greater_than(2)));
    }

}
