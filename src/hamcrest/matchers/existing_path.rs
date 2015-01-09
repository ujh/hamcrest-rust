use {success,expect,Matcher,MatchResult};
use std::fmt;
use std::io::fs::PathExtensions;

#[derive(Copy)]
pub enum PathType {
  AnyType,
  File,
  Dir
}

#[derive(Copy)]
pub struct ExistingPath {
  path_type: PathType
}

impl ExistingPath {
  fn match_path_type(&self, actual: &Path) -> MatchResult {
    match self.path_type {
      PathType::File => expect(actual.is_file(), format!("`{}` was not a file", actual.display())),
      PathType::Dir => expect(actual.is_dir(), format!("`{}` was not a dir", actual.display())),
      _ => success(),
    }
  }
}

impl fmt::Show for ExistingPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an existing file")
    }
}

impl<'a> Matcher<&'a Path> for ExistingPath {
  fn matches(&self, actual: &Path) -> MatchResult {
    expect(actual.exists(), format!("{} was missing", actual.display()))
      .and(self.match_path_type(actual))
  }
}

pub fn existing_path() -> ExistingPath {
  ExistingPath { path_type: PathType::AnyType }
}

pub fn existing_file() -> ExistingPath {
  ExistingPath { path_type: PathType::File }
}

pub fn existing_dir() -> ExistingPath {
  ExistingPath { path_type: PathType::Dir }
}

#[cfg(test)]
mod test {
  use std::os;
  use {assert_that,is,is_not,existing_file,existing_dir,existing_path};

  #[test]
  fn test_with_existing_file() {
    let path = path(os::getenv("TEST_EXISTS_FILE"), "./README.md");

    assert_that(&path, is(existing_path()));
    assert_that(&path, is(existing_file()));
    assert_that(&path, is_not(existing_dir()));
  }

  #[test]
  fn test_with_existing_dir() {
    let path = path(os::getenv("TEST_EXISTS_DIR"), "./target");

    assert_that(&path, is(existing_path()));
    assert_that(&path, is(existing_dir()));
    assert_that(&path, is_not(existing_file()));
  }

  #[test]
  fn test_with_nonexisting_path() {
    let path = path(os::getenv("TEST_EXISTS_NONE"), "./zomg.txt");

    assert_that(&path, is_not(existing_path()));
    assert_that(&path, is_not(existing_file()));
    assert_that(&path, is_not(existing_dir()));
  }

  fn path(path: Option<String>, default: &str) -> Path {
    Path::new(path.unwrap_or(default.to_string()))
  }
}
