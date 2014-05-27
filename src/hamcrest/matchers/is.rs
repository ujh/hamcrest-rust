use {Matcher,MatchResult,SelfDescribing};

pub struct Is<T, M> {
  matcher: Box<M>
}

impl<T, M : Matcher<T>> SelfDescribing for Is<T, M> {
  fn describe(&self) -> String {
    self.matcher.describe()
  }
}

impl<T, M : Matcher<T>> Matcher<T> for Is<T, M> {
  fn matches(&self, actual: T) -> MatchResult {
    self.matcher.matches(actual)
  }
}

pub fn is<T, M: Matcher<T>>(matcher: Box<M>) -> Box<Is<T, M>> {
  box Is { matcher: matcher }
}

pub struct IsNot<T, M> {
  matcher: Box<M>
}

impl<T, M : Matcher<T>> SelfDescribing for IsNot<T, M> {
  fn describe(&self) -> String {
    format!("not {}", self.matcher.describe())
  }
}

impl<T, M : Matcher<T>> Matcher<T> for IsNot<T, M> {
  fn matches(&self, actual: T) -> MatchResult {
    match self.matcher.matches(actual) {
      Ok(_) => Err("matched".to_owned()),
      Err(_) => Ok(())
    }
  }
}

pub fn is_not<T, M: Matcher<T>>(matcher: Box<M>) -> Box<IsNot<T, M>> {
  box IsNot { matcher: matcher }
}
