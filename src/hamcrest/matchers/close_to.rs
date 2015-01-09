use std::fmt::{self, Show, Formatter};
use std::num::{Float, NumCast, cast};
use {success,Matcher,MatchResult};

pub struct CloseTo<T> {
  expected: T,
  epsilon: T
}

impl<T: Show> Show for CloseTo<T> {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      self.expected.fmt(f)
  }
}

impl<T : Float + PartialEq + Show> Matcher<T> for CloseTo<T> {
  fn matches(&self, actual: T) -> MatchResult {
    let d = (self.expected - actual).abs();

    let close = self.expected == actual
        || ((self.expected == Float::zero() || actual == Float::zero() || d < Float::min_pos_value(None)) &&
            d < self.epsilon * Float::min_pos_value(None))
        || d / (self.expected.abs() + actual.abs()) < self.epsilon;

    if close {
      success()
    }
    else {
      Err(format!("was {:?}", actual))
    }
  }
}

pub fn close_to<T: Float + PartialEq + NumCast + Show>(expected: T) -> CloseTo<T> {
    close_to_eps(expected, cast::<f32, T>(0.00001).unwrap())
}

pub fn close_to_eps<T: Float + PartialEq + Show>(expected: T, epsilon: T) -> CloseTo<T> {
  CloseTo { expected: expected, epsilon: epsilon }
}

#[cfg(test)]
mod test {
  use std::num::Float;
  use std::thread::Thread;
  use {assert_that,is,close_to,close_to_eps};

  #[test]
  fn test_equality_of_floats() {
    let inf: f32 = Float::infinity();
    let nan: f32 = Float::nan();

    // Successful match
    assert_that(1.0f32, is(close_to(1.0)));
    assert_that(inf, is(close_to(inf)));
    assert_that(1e-40f32, is(close_to_eps(0.0, 0.01)));

    // Unsuccessful match
    assert!(Thread::scoped(|| {
      assert_that(2.0, is(close_to(1.0f32)));
    }).join().is_err());

    assert!(Thread::scoped(move || {
      assert_that(nan, is(close_to(nan)));
    }).join().is_err());

    assert!(Thread::scoped(|| {
      assert_that(1e-40f32, is(close_to_eps(0.0, 0.000001)));
    }).join().is_err());
  }
}
