use std::result::Result;

pub type MatchResult = Result<(), String>;

pub fn success() -> MatchResult {
  Ok(())
}

pub fn expect(predicate: bool, msg: String) -> MatchResult {
  if predicate {
    success()
  }
  else {
    Err(msg)
  }
}

pub fn assert_that<T, U: Matcher<T> + SelfDescribing>(actual: T,
                                                      matcher: U) {
  match matcher.matches(actual) {
    Ok(_) => return,
    Err(mismatch) => {
      let expected = matcher.describe();
      fail!("\nExpected: {}\n    but: {}", expected, mismatch);
    }
  }
}

pub trait SelfDescribing {

  fn describe(&self) -> String {
    format!("{:?}", self)
  }

}

pub trait Matcher<T> : SelfDescribing {
  fn matches(&self, actual: T) -> MatchResult;
}
