use std::fmt::{self, Formatter};
use std::f64;
use {success, Matcher, MatchResult};

pub struct CloseTo<T> {
    expected: T,
    epsilon: T,
}

impl<T: fmt::Debug> fmt::Display for CloseTo<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl Matcher<f64> for CloseTo<f64> {
    fn matches(&self, actual: f64) -> MatchResult {
        let d = (self.expected - actual).abs();

        let close = self.expected == actual
            || ((self.expected == 0.0 || actual == 0.0 || d < f64::MIN_POSITIVE) &&
                d < self.epsilon * f64::MIN_POSITIVE)
            || d / (self.expected.abs() + actual.abs()) < self.epsilon;

        if close {
            success()
        }
        else {
            Err(format!("was {:?}", actual))
        }
    }
}

pub fn close_to(expected: f64) -> CloseTo<f64> {
    close_to_eps(expected, 0.00001)
}

pub fn close_to_eps(expected: f64, epsilon: f64) -> CloseTo<f64> {
    CloseTo { expected: expected, epsilon: epsilon }
}

#[cfg(test)]
mod test {
    use std::f64;
    use std::thread;
    use {assert_that,is,close_to,close_to_eps};

    #[test]
    fn test_equality_of_floats() {
        // Successful match
        assert_that(1.0f64, is(close_to(1.0)));
        assert_that(f64::INFINITY, is(close_to(f64::INFINITY)));
        assert_that(1e-40f64, is(close_to_eps(0.0, 0.01)));

        // Unsuccessful match
        assert!(thread::spawn(|| {
            assert_that(2.0, is(close_to(1.0f64)));
        }).join().is_err());

        assert!(thread::spawn(move || {
            assert_that(f64::NAN, is(close_to(f64::NAN)));
        }).join().is_err());

        assert!(thread::spawn(|| {
            assert_that(1e-40f64, is(close_to_eps(0.0, 0.000001)));
        }).join().is_err());
    }
}
