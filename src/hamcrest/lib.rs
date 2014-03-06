#[crate_id = "hamcrest"];

use std::task;
use std::fmt::Show;

pub struct Description {
  msg: ~str
}

impl Description {
  pub fn new() -> Description {
    Description { msg: ~"" }
  }

  pub fn append_text<'r>(&'r mut self, item: &str) -> &'r mut Description {
    self.msg = self.msg + item;
    self
  }

  pub fn append_description_of<'r>(&'r mut self, item: &SelfDescribing) -> &'r mut Description {
    item.describe_to(self);
    self
  }

  pub fn to_str<'r>(&'r self) -> &'r str {
    self.msg.as_slice()
  }
}

pub trait SelfDescribing {

  fn describe_to(&self, desc: &mut Description) {
    desc.append_text(format!("{:?}", self));
  }

}

pub trait Matcher<T> : SelfDescribing + Clone {
  fn matches(&self, actual: &T) -> bool;

  fn describe_mismatch(&self, _: &T, desc: &mut Description) {
    desc.append_text(format!("was {:?}", self));
  }
}



/*
 *
 * ===== Is =====
 *
 */


#[deriving(Clone)]
struct Is<T, M> {
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

fn is<T, M: Matcher<T>>(matcher: M) -> Is<T, M> {
  Is { matcher: matcher.clone() }
}

/*
 *
 * ===== EqualTo =====
 *
 */

#[deriving(Clone)]
struct EqualTo<T> {
  expected: T
}

impl<T : Show> SelfDescribing for EqualTo<T> {
  fn describe_to(&self, desc: &mut Description) {
    desc.append_text(format!("{}", self.expected));
  }
}

impl<T : Eq + Clone + Show> Matcher<T> for EqualTo<T> {
  fn matches(&self, actual: &T) -> bool {
    self.expected.eq(actual)
  }

  fn describe_mismatch(&self, actual: &T, desc: &mut Description) {
    desc.append_text(format!("was {}", actual));
  }
}

fn equal_to<T : Eq + Clone + Show>(expected: &T) -> EqualTo<T> {
  EqualTo { expected: expected.clone() }
}

/*
 *
 * ===== Assertion =====
 *
 */

fn assert_that<T: Clone, U: Matcher<T> + SelfDescribing>(actual: T, matcher: U) {
  if !matcher.matches(&actual) {
    let mut desc = Description::new();

    desc
      .append_text("\nExpected: ")
      .append_description_of(&matcher as &SelfDescribing)
      .append_text("\n     but: ");

    matcher.describe_mismatch(&actual, &mut desc);

    fail!("{}", desc.to_str());
  }
}

#[test]
fn test_equality_of_ints() {
  // Successful match
  assert_that(1, is(equal_to(&1)));

  // Unsuccessful match
  let res = task::try(proc() {
    assert_that(2, is(equal_to(&1)));
  });

  assert!(res.is_err());
} 
