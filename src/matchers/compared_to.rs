// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton, Yehuda Katz,
//                Ben Longbons
// Copyright 2015 Carl Lerche, Alex Crichton, Robin Gloster
// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

use core::*;

enum CompareOperation {
    LessThan,
}

pub struct ComparedTo<T> {
    operation: CompareOperation,
    right_hand_side: T,
}

impl<T: fmt::Debug> fmt::Display for ComparedTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "< {:?}", &self.right_hand_side)
    }
}

impl<T : PartialOrd + fmt::Debug> Matcher<T> for ComparedTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        if actual < self.right_hand_side {
            success()
        }
        else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn less_than<T : PartialOrd + fmt::Debug>(right_hand_side: T) -> ComparedTo<T> {
    ComparedTo {
        operation: CompareOperation::LessThan,
        right_hand_side: right_hand_side
    }
}
