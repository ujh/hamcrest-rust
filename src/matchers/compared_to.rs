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
    LessOrEqual,
    LessThan,
    GreaterThan,
}

pub struct ComparedTo<T> {
    operation: CompareOperation,
    right_hand_side: T,
}

impl<T: fmt::Debug> fmt::Display for ComparedTo<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let operation = match self.operation {
            CompareOperation::LessOrEqual => "<=",
            CompareOperation::LessThan => "<",
            CompareOperation::GreaterThan => ">",
        };

        write!(f, "{} {:?}", operation, &self.right_hand_side)
    }
}

impl<T : PartialOrd + fmt::Debug> Matcher<T> for ComparedTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        let it_succeeded = match self.operation {
            CompareOperation::LessOrEqual => actual <= self.right_hand_side,
            CompareOperation::LessThan => actual < self.right_hand_side,
            CompareOperation::GreaterThan => actual > self.right_hand_side,
        };

        if it_succeeded {
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

pub fn less_than_or_equal_to<T : PartialOrd + fmt::Debug>(right_hand_side: T) -> ComparedTo<T> {
    ComparedTo {
        operation: CompareOperation::LessOrEqual,
        right_hand_side: right_hand_side
    }
}

pub fn greater_than<T : PartialOrd + fmt::Debug>(right_hand_side: T) -> ComparedTo<T> {
    ComparedTo {
        operation: CompareOperation::GreaterThan,
        right_hand_side: right_hand_side
    }
}
