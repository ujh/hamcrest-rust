// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton, Ben Longbons,
//                Michael Gehring, Yehuda Katz
// Copyright 2015 Carl Lerche, Alex Crichton, Graham Dennis, Robin Gloster
// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;
use std::path::{Path, PathBuf};
use std::fs;

use core::*;

#[derive(Clone, Copy)]
pub enum PathType {
    AnyType,
    File,
    Dir,
}

#[derive(Clone, Copy)]
pub struct ExistingPath {
    path_type: PathType,
}

impl ExistingPath {
    fn match_path_type(&self, actual: &Path) -> MatchResult {
        let metadata = fs::metadata(actual);
        match self.path_type {
            PathType::File => {
                expect(metadata.map(|m| m.is_file()).unwrap_or(false),
                       format!("`{}` was not a file", actual.display()))
            }
            PathType::Dir => {
                expect(metadata.map(|m| m.is_dir()).unwrap_or(false),
                       format!("`{}` was not a dir", actual.display()))
            }
            _ => success(),
        }
    }
}

impl fmt::Display for ExistingPath {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an existing file")
    }
}

impl<'a> Matcher<&'a PathBuf> for ExistingPath {
    fn matches(&self, actual: &'a PathBuf) -> MatchResult {
        self.matches(&**actual)
    }
}

impl<'a> Matcher<&'a Path> for ExistingPath {
    fn matches(&self, actual: &Path) -> MatchResult {
        expect(fs::metadata(actual).is_ok(),
               format!("{} was missing", actual.display()))
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
