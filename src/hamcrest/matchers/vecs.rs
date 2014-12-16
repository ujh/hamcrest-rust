use std::fmt::{mod, Show};
use std::vec::Vec;
use {success, Matcher, MatchResult};

#[deriving(Clone, Copy)]
pub struct OfLen {
  len: uint
}

impl Show for OfLen {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "of len {}", self.len)
  }
}

impl<'a, T> Matcher<&'a Vec<T>> for OfLen {
  fn matches(&self, actual: &Vec<T>) -> MatchResult {
    if self.len == actual.len() {
      success()
    }
    else {
      Err(format!("was len {}", actual.len()))
    }
  }
}

pub fn of_len(len: uint) -> Box<OfLen> {
  box OfLen { len: len }
}

#[deriving(Clone)]
pub struct Contains<T> {
    items: Vec<T>,
    exactly: bool
}

impl<T> Contains<T> {
    pub fn exactly(mut self) -> Contains<T> {
        self.exactly = true;
        self
    }
}

impl<T: Show> Show for Contains<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.exactly {
            write!(f, "containing exactly {}", self.items)
        } else {
            write!(f, "containing {}", self.items)
        }
    }
}

impl<'a, T: Show + PartialEq + Clone> Matcher<&'a Vec<T>> for Contains<T> {
  fn matches(&self, actual: &Vec<T>) -> MatchResult {
    let mut rem = actual.clone();

    for item in self.items.iter() {
        match rem.iter().position(|a| *item == *a) {
            Some(idx) => { rem.remove(idx); },
            None => return Err(format!("was {}", actual))
        }
    }

    if self.exactly && !rem.is_empty() {
        return Err(format!("also had {}", rem));
    }

    success()
  }
}

pub fn contains<T>(items: Vec<T>) -> Contains<T> {
  Contains { items: items, exactly: false }
}

#[cfg(test)]
mod test {
    use {
        not,
        assert_that,
        contains
    };

    #[test]
    fn test_vec_contains() {
        assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
        assert_that(&vec!(1i, 2, 3), not(contains(vec!(4i))));
    }

    #[test]
    fn test_vec_contains_exactly() {
        assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
        assert_that(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));
    }
}
