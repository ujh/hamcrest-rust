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

pub use core::{expect, success, Matcher, MatchResult};
pub use prelude::*;

pub mod core;
pub mod matchers;
pub mod prelude {
    pub use core::assert_that;
    pub use matchers::is::{is, is_not};
    pub use matchers::is::is_not as not;
    pub use matchers::is::is_not as does_not;
    pub use matchers::none::{none};
    pub use matchers::equal_to::equal_to;
    pub use matchers::close_to::close_to;
    pub use matchers::existing_path::{existing_path, existing_file, existing_dir};
    pub use matchers::regex::matches_regex;
    pub use matchers::regex::matches_regex as match_regex;
    pub use matchers::vecs::{of_len, contains};
}
