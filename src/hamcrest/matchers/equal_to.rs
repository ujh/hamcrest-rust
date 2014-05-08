use {success,Matcher,MatchResult,SelfDescribing};

pub struct EqualTo<T> {
  expected: T
}

impl<T> SelfDescribing for EqualTo<T> {
  fn describe(&self) -> ~str {
    format!("{:?}", self.expected)
  }
}

impl<T : Eq> Matcher<T> for EqualTo<T> {
  fn matches(&self, actual: T) -> MatchResult {
    if self.expected.eq(&actual) {
      success()
    }
    else {
      Err(format!("was {:?}", actual))
    }
  }
}

pub fn equal_to<T : Eq>(expected: T) -> Box<EqualTo<T>> {
  box EqualTo { expected: expected }
}

#[cfg(test)]
mod test {
  use std::task;
  use {assert_that,is,equal_to};

  #[test]
  fn test_equality_of_ints() {
    // Successful match
    assert_that(&1, is(equal_to(&1)));

    // Unsuccessful match
    let res = task::try(proc() {
      assert_that(&2, is(equal_to(&1)));
    });

    assert!(res.is_err());
  }
}
