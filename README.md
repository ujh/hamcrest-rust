# Hamcrest

A port of [Hamcrest](http://hamcrest.org/) to [Rust](http://rust-lang.org).

## Installing

To use Hamcrest, add this to your `Cargo.toml`:

```
[dependencies.hamcrest]

git = "https://github.com/carllerche/hamcrest-rust.git"
```

And this to your crate root:

```{rust}
#[cfg(test)]
extern crate hamcrest;
```

After a quick `cargo build`, you should be good to go!

## Usage

Hamcrest supports a number of matchers. You'll have to `use`
them just like any other Rust library.

### equal\_to

```{rust}
// Successful match
assert_that(&1, is(equal_to(&1i)));

// Unsuccessful match
let res = task::try(proc() {
    assert_that(&2, is(equal_to(&1i)));
});

assert!(res.is_err());
```

### existing\_{file,path,dir}

```{rust}
assert_that(&path, is(existing_path()));
assert_that(&path, is(existing_file()));
assert_that(&path, is_not(existing_dir()));
```

### none

```{rust}
assert_that(None, is(none::<int>()));
assert_that(Some(1), is_not(none::<int>()));
```

### contains and contains\_exactly

```{rust}
assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2)));
assert_that(&vec!(1i, 2, 3), not(contains(vec!(4i))));

assert_that(&vec!(1i, 2, 3), contains(vec!(1i, 2, 3)).exactly());
assert_that(&vec!(1i, 2, 3), not(contains(vec!(1i, 2)).exactly()));
```
