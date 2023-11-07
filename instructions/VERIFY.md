
# Verify the ceremony

## Prerequisites

You will need Rust to compile the code that verifies the contributions. You can follow the recommended [guide](https://www.rust-lang.org/tools/install).

## Verify the contributions

First, clone the following code:

```
git clone https://github.com/dusk-network/powersoftau.git
```

Download the first contribution `response`, place it into the root of the cloned repository, and execute:

```
cargo run --release --bin new # this generates the first challenge
cargo run --release --bin verify_transform # this verifies that the response has been computed using the given challenge
```
Once verified, a new file `new_challenge` will be generated. Now you can download the next response file, and keep verifying all the contributions as follows:

```
mv new_challenge challenge
cargo run --release --bin verify_transform
```