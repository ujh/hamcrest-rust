// Copyright 2017 Povilas Balciunas
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[macro_use]
extern crate hamcrest;

mod boolean {
    mod is_true {
        use hamcrest::prelude::*;

        #[test]
        fn matches_when_true() {
            assert_that!(true, is(true));
        }

        #[test]
        #[should_panic]
        fn fails_when_false() {
            assert_that!(false, is(true));
        }
    }

    mod is_false {
        use hamcrest::prelude::*;

        #[test]
        fn matches_when_false() {
            assert_that!(false, is(false));
        }

        #[test]
        #[should_panic]
        fn fails_when_true() {
            assert_that!(true, is(false));
        }
    }
}
