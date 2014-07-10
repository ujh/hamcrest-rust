use {success,Matcher,MatchResult,SelfDescribing};

pub struct IsNone<T>;

impl<T> SelfDescribing for IsNone<T> {
  fn describe(&self) -> String {
    "none".to_string()
  }
}

impl<T> Matcher<Option<T>> for IsNone<T> {
  fn matches(&self, actual: Option<T>) -> MatchResult {
    if actual.is_none() {
      return success();
    }

    Err(format!("was {:?}", actual))
  }
}

pub fn none<T>() -> Box<IsNone<T>> {
  box IsNone
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
