use std::fmt;

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

pub fn assert_that<T, U: Matcher<T>>(actual: T, matcher: U) {
    match matcher.matches(actual) {
        Ok(_) => return,
        Err(mismatch) => {
            fail!("\nExpected: {}\n    but: {}", matcher, mismatch);
        }
    }
}

pub trait Matcher<T>: fmt::Show {
  fn matches(&self, actual: T) -> MatchResult;
}
