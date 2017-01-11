// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest;

mod none {

    use hamcrest::prelude::*;

    #[test]
    fn none_is_none() {
        assert_that!(None, is(none::<i8>()));
    }

    #[test]
    fn some_is_not_none() {
        assert_that!(Some(1), is_not(none()));
    }

}
