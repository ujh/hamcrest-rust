// Copyright 2014 Carl Lerche, Steve Klabnik, Alex Crichton
// Copyright 2015 Carl Lerche
// Copyright 2016 Urban Hafner
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// Principal trait to implement to extend Hamcrest.
pub trait Matcher<T>: fmt::Display {
    fn matches(&self, actual: T) -> MatchResult;
}

/// Complete set of information that `Matcher`s must provide.
pub struct MatchResult {
    /// Matchers must provide a formatted version of any object they can attempt
    /// to match. This is stored as `formatted_actual`.
    formatted_actual: String,
    /// Appearance of the `Matcher`, typically provided with `fmt::Display`.
    formatted_matcher: String,
    /// Matchers must also state whether the match was successful or not, and
    /// optionally give an explanation for failures.
    status: MatchStatus,
}

impl MatchResult {
    /// Make a `MatchResult` indicating a successful match.
    pub fn success(
        formatted_matcher: String,
        formatted_actual: String,
    ) -> Self {
        Self {
            formatted_actual: formatted_actual,
            formatted_matcher: formatted_matcher,
            status: Ok(()),
        }
    }
    /// Make a `MatchResult` indicating a failed match.
    pub fn failure(
        formatted_matcher: String,
        formatted_actual: String,
    ) -> Self {
        Self {
            formatted_actual: formatted_actual,
            formatted_matcher: formatted_matcher,
            status: Err(None),
        }
    }

    /// Attach an explanation for why the match was unsuccessful.
    pub fn with_explanation<TS: ToString>(self, explanation: TS) -> Self {
        if self.status == Ok(()) {
            panic!("Applied explanation to successful status")
        }
        Self {
            status: Err(Some(explanation.to_string())),
            ..self
        }
    }

    pub fn formatted_assertion_failure(&self) -> String {
        format!(
            "\n  Expected: {}\n       Got: {}{}\n",
            self.formatted_matcher(),
            self.formatted_actual(),
            match self.status {
                Ok(()) => {
                    panic!("Status was Ok");
                }
                Err(ref explanation_option) => match explanation_option {
                    &Some(ref explanation) => format!(" ({})", explanation),
                    &None => "".to_owned(),
                },
            }
        )
    }

    pub fn formatted_actual(&self) -> &String {
        return &self.formatted_actual;
    }
    pub fn formatted_matcher(&self) -> &String {
        return &self.formatted_matcher;
    }
    pub fn status(&self) -> &MatchStatus {
        return &self.status;
    }
}

/// Preferred way of constructing a `MatchResult` indicating success.
pub fn success<M: fmt::Display, TS: ToString>(
    matcher: &M,
    formatted_actual: TS,
) -> MatchResult {
    MatchResult::success(format!("{}", matcher), formatted_actual.to_string())
}
/// Preferred way of constructing a `MatchResult` indicating failure.
pub fn failure<M: fmt::Display, TS: ToString>(
    matcher: &M,
    formatted_actual: TS,
) -> MatchResult {
    MatchResult::failure(format!("{}", matcher), formatted_actual.to_string())
}

pub type MatchStatus = Result<(), Option<String>>;
