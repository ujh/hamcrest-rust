#![crate_name = "hamcrest"]
#![crate_type = "lib"]

extern crate collections;
extern crate debug;

pub use core::{assert_that,expect,success,Matcher,MatchResult,SelfDescribing};
pub use matchers::is::{is,is_not};
pub use matchers::is::is_not as not;
pub use matchers::none::{none};
pub use matchers::equal_to::equal_to;
pub use matchers::existing_path::{existing_path,existing_file,existing_dir};
pub use matchers::vecs::{of_len,contains};

pub mod core;
pub mod matchers {
  pub mod equal_to;
  pub mod existing_path;
  pub mod is;
  pub mod none;
  pub mod vecs;
}
