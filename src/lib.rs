// Copyright 2014 Carl Lerche, Oliver Mader, Alex Crichton, Thiago Pontes,
//                Yehuda Katz
// Copyright 2015 Carl Lerche, Oliver Mader
// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_name = "hamcrest"]
#![crate_type = "lib"]

extern crate num;
extern crate regex;

pub use prelude::*;

#[macro_export]
macro_rules! assert_that {
    ($actual:expr, $matcher:expr) => ({
        // The separate statement is necessary to keep the compiler happy.
        let m = $matcher;
        match m.matches($actual) {
            Ok(_) => {},
            Err(mismatch) => {
                // The panic macro produces the correct file and line number
                // when used in a macro like this, i.e. it's the line where
                // the macro was originally written.
                panic!("\nExpected: {}\n    but: {}", m, mismatch);
            }
        }
    }
    );
}

pub mod core;
pub mod matchers;
pub mod prelude {
    #[allow(deprecated)] pub use core::assert_that;
    pub use core::Matcher as HamcrestMatcher;
    pub use matchers::close_to::close_to;
    pub use matchers::equal_to::equal_to;
    pub use matchers::existing_path::existing_dir;
    pub use matchers::existing_path::existing_file;
    pub use matchers::existing_path::existing_path;
    pub use matchers::is::is_not as does_not;
    pub use matchers::is::is_not as not;
    pub use matchers::is::is_not;
    pub use matchers::is::is;
    pub use matchers::none::none;
    pub use matchers::regex::matches_regex as match_regex;
    pub use matchers::regex::matches_regex;
    pub use matchers::vecs::contains;
    pub use matchers::vecs::of_len;
}
