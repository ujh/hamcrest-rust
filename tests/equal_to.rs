// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use] extern crate hamcrest;

mod equal_to {

    use hamcrest::prelude::*;

    #[test]
    fn equality_of_ints() {
        assert_that!(1, is(equal_to(1)));
    }

    #[test]
    #[should_panic]
    fn unsuccessful_match() {
        assert_that!(2, is(equal_to(1)));
    }

}
