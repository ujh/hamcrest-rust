use super::super::core::{Matcher,SelfDescribing,Description};

#[deriving(Clone,Eq)]
pub struct Is<T, M> {
  matcher: M
}

impl<T, M : Matcher<T>> SelfDescribing for Is<T, M> {
  fn describe_to(&self, desc: &mut Description) {
    self.matcher.describe_to(desc);
  }
}

impl<T : Clone, M : Matcher<T>> Matcher<T> for Is<T, M> {
  fn matches(&self, actual: &T) -> bool {
    self.matcher.matches(actual)
  }

  fn describe_mismatch(&self, actual: &T, desc: &mut Description) {
    self.matcher.describe_mismatch(actual, desc);
  }
}

pub fn is<T, M: Matcher<T>>(matcher: M) -> Is<T, M> {
  Is { matcher: matcher.clone() }
}

#[deriving(Clone,Eq)]
pub struct IsNot<T, M> {
  matcher: M
}

impl<T, M : Matcher<T>> SelfDescribing for IsNot<T, M> {
  fn describe_to(&self, desc: &mut Description) {
    self.matcher.describe_to(desc);
  }
}

impl<T : Clone, M : Matcher<T>> Matcher<T> for IsNot<T, M> {
  fn matches(&self, actual: &T) -> bool {
    !self.matcher.matches(actual)
  }

  fn describe_mismatch(&self, actual: &T, desc: &mut Description) {
    self.matcher.describe_mismatch(actual, desc);
  }
}

pub fn is_not<T, M: Matcher<T>>(matcher: M) -> IsNot<T, M> {
  IsNot { matcher: matcher.clone() }
}
