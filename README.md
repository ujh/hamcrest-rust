[![Build Status](https://travis-ci.org/carllerche/hamcrest-rust.svg?branch=master)](https://travis-ci.org/carllerche/hamcrest-rust)

# Hamcrest

A port of [Hamcrest](http://hamcrest.org/) to [Rust](http://rust-lang.org).

## Installing

To use Hamcrest, add this to your `Cargo.toml`:

```
[dev-dependencies.hamcrest]

git = "https://github.com/carllerche/hamcrest-rust.git"
```

And this to your crate root:

``` rust
#[cfg(test)]
extern crate hamcrest;
```

After a quick `cargo build`, you should be good to go!

## Usage

Hamcrest supports a number of matchers. You'll have to `use`
them just like any other Rust library. Alternatively you can also `use` all of them at once like this:

``` rust
use hamcrest::prelude::*;
```

### equal\_to

``` rust
assert_that(1, is(equal_to(1)));
```

### close\_to

``` rust
assert_that(1e-40f32, is(close_to(0.0, 0.01)));
assert_that(1e-40f32, is_not(close_to(0.0, 0.000001)));
```

### existing\_{file,path,dir}

``` rust
assert_that(&path, is(existing_path()));
assert_that(&path, is(existing_file()));
assert_that(&path, is_not(existing_dir()));
```

### none

``` rust
assert_that(None, is(none::<int>()));
assert_that(Some(1), is_not(none::<int>()));
```

### contains and contains\_exactly

``` rust
assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
assert_that(&vec!(1i, 2, 3), not(contains(vec!(4i))));

assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
assert_that(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));
```

### matches_regex

``` rust
assert_that("1234", matches_regex(r"\d"));
assert_that("abc", does_not(match_regex(r"\d")));
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
