# random_number_generator

```bash
dfx start --clean --background

# deploying canister
dfx deploy generator

# calling function `generate_random_number`
dfx canister call generator generate_random_number

# calling function `generate_random_number_between_ranges` with args: to generate a number between a range of -10 to 10
dfx canister call generator generate_random_number_between_ranges '(-10, 10)'
```