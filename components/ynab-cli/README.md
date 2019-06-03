# `ynab-cli`

The opinionated command-line interface to the YNAB set of crates.

---

## Usage

You will need at least 2 values on-hand to use this CLI:

- a [personal access token] (that you never share with anyone!)
- your budget's id (see below for more information)

From the root of this repository run the following:

```shell
$ cargo run -- get-all-scheduled-transactions \
    --budget-id=<your-budget-id> \
    --bearer-token=<your-personal-access-token>
```

If the arguments you have provided are correct you should, assuming no network
issues, see your scheduled transactions pretty-printed to the screen.

### Finding your budget's ID

Your budget ID can be found in the URL while you are viewing your budget via
the YNAB web interface.

For example if the URL were https://app.youneedabudget.com/889283ca-e89c-4407-a244-826102837984/budget
the budget ID would be `889283ca-e89c-4407-a244-826102837984`.


[personal access token]: https://api.youneedabudget.com/#personal-access-tokens