
pub fn assert_that<T: Clone, U: Matcher<T> + SelfDescribing>(actual: &T, matcher: U) {
  if !matcher.matches(actual) {
    let mut desc = Description::new();

    desc
      .append_text("\nExpected: ")
      .append_description_of(&matcher as &SelfDescribing)
      .append_text("\n     but: ");

    matcher.describe_mismatch(actual, &mut desc);

    fail!("{}", desc.to_str());
  }
}

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
