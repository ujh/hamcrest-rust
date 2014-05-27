use {success,expect,Matcher,MatchResult,SelfDescribing};

pub enum PathType {
  AnyType,
  File,
  Dir
}

pub struct ExistingPath {
  path_type: PathType
}

impl ExistingPath {
  fn match_path_type(&self, actual: &Path) -> MatchResult {
    match self.path_type {
      File => expect(actual.is_file(), format!("`{}` was not a file", actual.display())),
      Dir => expect(actual.is_dir(), format!("`{}` was not a dir", actual.display())),
      _ => success()
    }
  }
}

impl SelfDescribing for ExistingPath {
  fn describe(&self) -> String {
    "an existing file".to_owned()
  }
}

impl<'a> Matcher<&'a Path> for ExistingPath {
  fn matches(&self, actual: &Path) -> MatchResult {
    expect(actual.exists(), format!("{} was missing", actual.display()))
      .and(self.match_path_type(actual))
  }
}

pub fn existing_path() -> Box<ExistingPath> {
  box ExistingPath { path_type: AnyType }
}

pub fn existing_file() -> Box<ExistingPath> {
  box ExistingPath { path_type: File }
}

pub fn existing_dir() -> Box<ExistingPath> {
  box ExistingPath { path_type: Dir }
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

  fn path(path: Option<String>) -> Path {
    Path::new(path.unwrap())
  }
}
