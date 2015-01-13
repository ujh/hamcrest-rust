use std::fmt;
use {Matcher,MatchResult};

pub struct Is<T, M> {
  matcher: M
}

impl<T, M: Matcher<T>> fmt::String for Is<T, M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      self.matcher.fmt(f)
  }
}

impl<T, M : Matcher<T>> Matcher<T> for Is<T, M> {
  fn matches(&self, actual: T) -> MatchResult {
    self.matcher.matches(actual)
  }
}

pub fn is<T, M: Matcher<T>>(matcher: M) -> Is<T, M> {
  Is { matcher: matcher }
}

pub struct IsNot<T, M> {
  matcher: M
}

impl<T, M : Matcher<T>> fmt::String for IsNot<T, M> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "not {}", self.matcher)
  }
}

impl<T, M : Matcher<T>> Matcher<T> for IsNot<T, M> {
  fn matches(&self, actual: T) -> MatchResult {
    match self.matcher.matches(actual) {
      Ok(_) => Err("matched".to_string()),
      Err(_) => Ok(())
    }
  }
}

pub fn is_not<T, M: Matcher<T>>(matcher: M) -> IsNot<T, M> {
  IsNot { matcher: matcher }
}
