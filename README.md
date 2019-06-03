# `ynab`

A software suite for [You Need A Budget's API](https://api.youneedabudget.com/).

[You Need A Budget](https://youneedabudget.com/) (YNAB) is a service with the
goal of helping users "stop living paycheck to paycheck, get out of debt, and
save more money."

This project is not endorsed or maintained by YNAB.

### License

See [LICENSE](./LICENSE).

### Stability / ready-for-prod-ness

##### Tip: Look at the version of this crate for a rough idea (currently 0.0.1)

***Not ready for production use!***

- There is little-to-no error handling currently (so your application may abort
if something goes wrong)
- There are no automated tests (unit, integration, fuzzing, etc.)

### Prerequisite knowledge

Development of this library is done on the "stable" distribution channel using
Rust `1.35`.

### Building this codebase

In the root of the project (the directory containing `README.md`):

```
$ cargo build
```

### Running the examples

This project has a number of examples meant to illustrate the capabilities of
the library.

Currently they all require you to provide an authentication token and the ID of
the budget to work with via environment variables.

```
$ TOKEN=<your personal access token here> BUDGET_ID=<your budget's ID here> cargo run --features=rfc5545 --example=icalendar_rrules
```

To generate a personal access token go to [your developer settings](https://app.youneedabudget.com/settings/developer) in YNAB.

The budget ID can be found by in the URL while you are viewing your budget as
usual.

For example the URL might be
https://app.youneedabudget.com/889283ca-e89c-4407-a244-826102837984/budget so
the budget ID is `889283ca-e89c-4407-a244-826102837984`.

### Using this project as a dependency

In your project add the following to the `[dependencies]` section of your
manifest (`Cargo.toml`):

```toml
ynab = "0.0.1"
```

After doing so your manifest may look similar to:

```toml
[package]
name = "my-cool-project"
version = "0.1.0"

[dependencies]
ynab = "0.0.1"
```

##### Tip: You can use [`cargo-edit`](https://crates.io/crates/cargo-edit) to easily manage dependencies (for example: `cargo add ynab`)

### Cargo Features

The [cargo features](https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section) this crate provides are as follows:

- `rfc5545`: Adds `RecurFrequency::as_rfc5545_rule` which produces an
`rfc5545::RecurrenceRule` (see [the rfc5545
crate](https://github.com/Phrohdoh/rfc5545-rs) for more information)

### Who uses this crate

- The not-yet-available _WhenCash_ service

##### Open an issue if you'd like to be added to this list!