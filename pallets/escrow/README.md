# Assesment

This is a short assesment of substrate and pallet development knowledge. The following pallet does not compile. In this asseesment we'd like for you to fix the code in `lib.rs` and in `test.rs` to successfully compile and complete the prompts.


## How to test

You can run tests on this pallet via the following command.

```bash
SKIP_WASM_BUILD= cargo test -p pallet-escrow
```

## Files to update

- [lib.rs](src/lib.rs)
- [test.rs](src/test.rs)

## Prompts

#### Prompt 1
Please fix the pallet to make this test pass.

#### Prompt 2
Please fix the test named `timelocked_escrow_example` to only lets escrow recipients to withdraw if enough time (blocks) have past.

#### Extra prompts (optional)

A) so far all of the amounts are not linked to any asset how might we use assets instead of arbitraty values

possible solutions
- maybe we can complete the mint and balance_of functions in the pallet and add create our own token. Then we can use this in our escrow logic above
- maybe we can use the `pallet_assets` or `pallet_balances` to add native token functionality into our pallet for our escrow logic

B) what other things can we do with an escrow? we use time and recipient above what are some other ways we can leverage an escrow. What might a use case look like?

C) please point out, fix and optimize this pallet code. What improvements can be made?
