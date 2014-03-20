use core::{Matcher,SelfDescribing,Description};

#[deriving(Clone,Eq)]
pub enum PathType {
  AnyType,
  File,
  Dir
}

#[deriving(Clone,Eq)]
pub struct ExistingPath {
  path_type: PathType
}

impl SelfDescribing for ExistingPath {
  fn describe_to(&self, desc: &mut Description) {
    desc.append_text("an existing file");
  }
}

impl Matcher<Path> for ExistingPath {
  fn matches(&self, actual: &Path) -> bool {
    if !actual.exists() {
      return false;
    }

    match self.path_type {
      File => actual.is_file(),
      Dir => actual.is_dir(),
      AnyType => true
    }
  }

  fn describe_mismatch(&self, actual: &Path, desc: &mut Description) {
    desc.append_text(format!("`{}` was missing", actual.display()));
  }
}

pub fn existing_path() -> ExistingPath {
  ExistingPath { path_type: AnyType }
}

pub fn existing_file() -> ExistingPath {
  ExistingPath { path_type: File }
}

pub fn existing_dir() -> ExistingPath {
  ExistingPath { path_type: Dir }
}

#[cfg(test)]
mod test {
  use std::os;
  use {assert_that,is,is_not,existing_file,existing_dir,existing_path};

  #[test]
  fn test_with_existing_file() {
    let path = path(os::getenv("TEST_EXISTS_FILE"));

    assert_that(&path, is(existing_path()));
    assert_that(&path, is(existing_file()));
    assert_that(&path, is_not(existing_dir()));
  }

  #[test]
  fn test_with_existing_dir() {
    let path = path(os::getenv("TEST_EXISTS_DIR"));

    assert_that(&path, is(existing_path()));
    assert_that(&path, is(existing_dir()));
    assert_that(&path, is_not(existing_file()));
  }

  #[test]
  fn test_with_nonexisting_path() {
    let path = path(os::getenv("TEST_EXISTS_NONE"));

    assert_that(&path, is_not(existing_path()));
    assert_that(&path, is_not(existing_file()));
    assert_that(&path, is_not(existing_dir()));
  }

  fn path(path: Option<~str>) -> Path {
    Path::new(path.unwrap())
  }
}
