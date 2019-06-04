# `ynab`

A software suite for [You Need A Budget]'s [API].

[You Need A Budget] (YNAB) is a service with the goal of helping people "stop
living paycheck to paycheck, get out of debt, and save more money."

This project is not funded, endorsed, or maintained by You Need A Budget LLC.

## Install

This project is composed of multiple sub-projects, one of which is an
executable [command-line interface] that you will, one day, be able to install.

Currently a pre-built [cli] executable is not hosted anywhere that you can
install from.

If you would like to use the [cli] you must build it from source.

## Usage

```
$ ynab help

ynab-cli 0.0.1
The opinionated command-line interface to the YNAB set of crates

USAGE:
    ynab <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    get-all-scheduled-transactions    Get all scheduled transactions for a budget
    get-category-by-id                Get a category in a budget by its ID
    help                              Prints this message or the help of the given subcommand(s)
```

You can, for example, get all of your scheduled transactions by running the
following command, substituting the `budget-id` and `bearer-token` values.

```
$ ynab get-all-scheduled-transactions \
    --bearer-token=your-bearer-token \
    --budget-id=your-budget-id
```

## Build from source

Building this software from source can be done by executing the following
command from the root of this repository assuming you have installed the [Rust
tooling] (which, typically, only developers do).

```shell
$ cargo build
```

## Contribute

For the time being please see the tickets labeled [good first issue].

## Support the project financially

Please consider financially supporting the project if it has made your life
easier, saved you time, etc., or as a way to say "thanks!" by [becoming a Patron]
of the maintainer.

[You Need A Budget]: https://youneedabudget.com
[API]: https://api.youneedabudget.com
[command-line interface]: https://en.wikipedia.org/wiki/Command-line_interface
[cli]: https://en.wikipedia.org/wiki/Command-line_interface
[Rust tooling]: https://rustup.rs/
[becoming a Patron]: https://www.patreon.com/Phrohdoh
[good first issue]: https://github.com/Phrohdoh/ynab-rs/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22