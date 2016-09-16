// Copyright 2014 Steve Klabnik, Valerii Hiora, Oliver Mader
// Copyright 2015 Carl Lerche, Oliver Mader, Alex Crichton, Graham Dennis,
//                Tamir Duberstein, Robin Gloster
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use num::{Float, Zero};
use std::fmt::{self, Display, Debug, Formatter};
use {success, Matcher, MatchResult};

/// Compares two floating point values for equality.
///
/// The comparison is based on a relative error metric and uses special
/// fallbacks for certain edge cases like very small numbers. The exact
/// algorithm is described [here](http://floating-point-gui.de/errors/comparison/).
pub struct CloseTo<T> {
    expected: T,
    epsilon: T,
}

impl<T: Debug> Display for CloseTo<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl<T: Float + Zero + Debug> Matcher<T> for CloseTo<T> {
    fn matches(&self, actual: T) -> MatchResult {
        let a = self.expected.abs();
        let b = actual.abs();

        let d = (a - b).abs();

        let close =
            // shortcut, handles infinities
            a == b
            // a or b is zero or both are extremely close to it
            // relative error is less meaningful here
            || ((a == Zero::zero() || b == Zero::zero() || d < Float::min_positive_value()) &&
                d < (self.epsilon * Float::min_positive_value()))
            // use relative error
            || d / (a + b).min(Float::max_value()) < self.epsilon;

        if close {
            success()
        } else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn close_to<T>(expected: T, epsilon: T) -> CloseTo<T> {
    CloseTo {
        expected: expected,
        epsilon: epsilon
    }
}

#[cfg(test)]
mod test {
    use std::f64;
    use {assert_that,is,not,close_to};

    #[test]
    fn equality_of_floats() {
        assert_that(1.0f64, is(close_to(1.0, 0.00001)));
        assert_that(1e-40f32, is(close_to(0.0, 0.01)));
        assert_that(1e-40f32, is(not(close_to(0.0, 0.000001))));
        assert_that(2.0, is(not(close_to(1.0f64, 0.00001))));
    }

    #[test]
    fn it_can_handle_infinity() {
        assert_that(f64::INFINITY, is(close_to(f64::INFINITY, 0.00001)));
    }

    #[test]
    fn it_can_handle_nan() {
        assert_that(f64::NAN, is(not(close_to(f64::NAN, 0.00001))));
    }
}
