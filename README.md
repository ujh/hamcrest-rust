[![Build Status](https://travis-ci.org/ujh/hamcrest-rust.svg?branch=master)](https://travis-ci.org/ujh/hamcrest-rust)

# Hamcrest

A port of [Hamcrest](http://hamcrest.org/) to [Rust](http://rust-lang.org).

## Installing

To use Hamcrest, add this to your `Cargo.toml`:

```
[dev-dependencies]
hamcrest = "*"
```

And this to your crate root:

``` rust
#[cfg(test)] #[macro_use] extern crate hamcrest;
```

After a quick `cargo build`, you should be good to go!

## Usage

Hamcrest supports a number of matchers. The easiest way is to just `use` them all like this:

``` rust
use hamcrest::prelude::*;
```

If you want to be more selective make sure that you also import the `HamcrestMatcher` trait.

### equal\_to

``` rust
assert_that!(1, is(equal_to(1)));
```

### close\_to

``` rust
assert_that!(1e-40f32, is(close_to(0.0, 0.01)));
assert_that!(1e-40f32, is_not(close_to(0.0, 0.000001)));
```

### compared\_to

``` rust
assert_that!(1, is(less_than(2)));
assert_that!(1, is(less_than_or_equal_to(1)));
assert_that!(2, is(greater_than(1)));
assert_that!(2, is(greater_than_or_equal_to(2)));
```

### existing\_{file,path,dir}

``` rust
assert_that!(&path, is(existing_path()));
assert_that!(&path, is(existing_file()));
assert_that!(&path, is_not(existing_dir()));
```

### none

``` rust
assert_that!(None, is(none::<int>()));
assert_that!(Some(1), is_not(none::<int>()));
```

### anything

``` rust
assert_that!(42, is(anything()));
assert_that!("test", is(anything()));
```

### contains, contains\_exactly, contains\_in order

``` rust
assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(4i))));

assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));

assert_that!(&vec!(1i, 2, 3), contains(vec!(1i, 2)).in_order());
assert_that!(&vec!(1i, 2, 3), not(contains(vec!(1i, 3)).in_order()));
```

### matches_regex

``` rust
assert_that!("1234", matches_regex(r"\d"));
assert_that!("abc", does_not(match_regex(r"\d")));
```

### type_of

``` rust
assert_that!(123usize, is(type_of::<usize>()));
assert_that!("test", is(type_of::<&str>()));
```

### all_of

``` rust
assert_that!(4, all_of!(less_than(5), greater_than(3)));
assert_that!(
    &vec![1, 2, 3],
    all_of!(contains(vec![1, 2]), not(contains(vec![4])))
);
```

### any_of

``` rust
assert_that!(4, any_of!(less_than(2), greater_than(3)));
assert_that!(
    &vec![1, 2, 3],
    any_of!(contains(vec![1, 2, 5]), not(contains(vec![4])))
);
```

### is(bool)

``` rust
assert_that!(true, is(true));
assert_that!(false, is(false));
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
