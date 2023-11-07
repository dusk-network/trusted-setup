
# Contribute to the ceremony

## Prerequisites

You will need Rust to compile the code that generates your contribution. You can follow the recommended [guide](https://www.rust-lang.org/tools/install).

## Compute your contribution

First, clone the following code:

```
git clone https://github.com/dusk-network/powersoftau.git
```

Once you get contacted by the coordinator, you will be provided with a challenge file `challenge`. Place the challenge in the root of the cloned repository, enter the repository, and execute:

```
cargo run --release --bin compute
```

When executing the previous command, you will be asked for entropy. It is important that you introduce a completely random input. 

It will take some time to compute the response. Once finished, provide the generated `response` file to the coordinator, along with the hash that you will see in the screen.