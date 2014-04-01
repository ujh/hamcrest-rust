#![crate_id = "hamcrest"]
#![crate_type = "lib"]

pub use core::{assert_that,expect,success,Matcher,MatchResult,SelfDescribing};
pub use matchers::is::{is,is_not};
pub use matchers::equal_to::equal_to;
pub use matchers::existing_path::{existing_path,existing_file,existing_dir};

pub mod core;
pub mod matchers {
  pub mod equal_to;
  pub mod existing_path;
  pub mod is;
}
