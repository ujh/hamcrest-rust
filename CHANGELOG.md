## 0.1.3 [☰](https://github.com/carllerche/hamcrest-rust/compare/0.1.2...0.1.3)

* Comparison matchers `less_than`, `less_than_or_equal_to`, `greater_than`, `greater_than_or_equal_to`. #43
* `in_order` option for `contains`. #44

## 0.1.2 [☰](https://github.com/carllerche/hamcrest-rust/compare/0.1.1...0.1.2)

* Added the `assert_that!` macro. It produces better error messages (with correct file and line
  number).
* Deprecated the `assert_that` function.
* Improvements to `Cargo.toml` (by @killercup)

## 0.1.1 [☰](https://github.com/carllerche/hamcrest-rust/compare/a9f18681c64e3126ef6ccbd68ec2a5b39fe5b58b...0.1.1)

* Licensing change. The crate is now dual licensed under the MIT and Apache 2 licenses.
* Adds the `prelude` submodule to simplify inclusion of all matchers.
* `matches_regex` matcher
