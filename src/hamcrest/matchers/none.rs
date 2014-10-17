use std::fmt;

use {success,Matcher,MatchResult};

pub struct IsNone<T>;

impl<T> fmt::Show for IsNone<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "none")
  }
}

impl<T: fmt::Show> Matcher<Option<T>> for IsNone<T> {
  fn matches(&self, actual: Option<T>) -> MatchResult {
    if actual.is_none() {
      return success();
    }

    Err(format!("was {}", actual))
  }
}

pub fn none<T>() -> IsNone<T> {
  IsNone
}

#[cfg(test)]
mod test {
  use {assert_that,is,is_not,none};

  #[test]
  fn test_none_is_none() {
    assert_that(None, is(none::<int>()));
    assert_that(Some(1), is_not(none::<int>()));
  }
}
