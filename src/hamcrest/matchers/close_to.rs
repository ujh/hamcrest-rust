use std::fmt::{self, Formatter};
use {success, Matcher, MatchResult};

pub struct CloseTo<T> {
    expected: T,
    delta: T,
}

impl<T: fmt::Debug> fmt::Display for CloseTo<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.expected.fmt(f)
    }
}

impl Matcher<f64> for CloseTo<f64> {
    fn matches(&self, actual: f64) -> MatchResult {
        // Handle cases like infinity / nan
        if self.expected == actual {
            return success();
        }

        let delta = (self.expected - actual).abs() - self.delta;

        if delta <= 0.0 {
            return success();
        }

        Err(format!("was {:?}", actual))

        //let delta = (self.expected - actual).abs();

        //if self.expected == actual {
            //return success();
        //}

        //if self.expected == 0.0 || actual == 0.0 || delta < f64::MIN_POSITIVE {
            //println!("~~~~~~~ ONE {}, {}, {}, {}, {}, {}", self.expected == 0.0, actual == 0.0, delta < f64::MIN_POSITIVE, delta, self.epsilon * f64::MIN_POSITIVE, self.epsilon);
            //if delta < self.epsilon * f64::MIN_POSITIVE {
                //return success();
            //}
        //}

        //if delta / (self.expected.abs() + actual.abs()) < self.epsilon {
            //println!("~~~~~~~ TWO");
            //return success();
        //}

        //Err(format!("was {:?}", actual))
    }
}

pub fn close_to<T>(expected: T, delta: T) -> CloseTo<T> {
    CloseTo {
        expected: expected,
        delta: delta
    }
}

#[cfg(test)]
mod test {
    use std::f64;
    use std::thread;
    use {assert_that,is,close_to};

    #[test]
    fn test_equality_of_floats() {
        // Successful match
        assert_that(1.0f64, is(close_to(1.0, 0.00001)));
        assert_that(f64::INFINITY, is(close_to(f64::INFINITY, 0.00001)));
        assert_that(1e-40f64, is(close_to(0.0, 0.01)));
        assert_that(1e-40f64, is(close_to(0.0, 0.000001)));

        // Unsuccessful match
        assert!(thread::spawn(|| {
            assert_that(2.0, is(close_to(1.0f64, 0.0001)));
        }).join().is_err());

        assert!(thread::spawn(move || {
            assert_that(f64::NAN, is(close_to(f64::NAN, 0.0001)));
        }).join().is_err());
    }
}
