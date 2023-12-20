
# Contribute to the ceremony

## Prerequisites

You will need Rust to compile the code that generates your contribution. You can follow the recommended [guide](https://www.rust-lang.org/tools/install).

## Compute your contribution

First, clone the following code:

```
git clone https://github.com/dusk-network/powersoftau.git
```

Once you get contacted by the coordinator, you will be provided with a challenge file named `challenge`. Place the file in the root of the cloned repository, navigate to the repository, and execute:

```
cargo run --release --bin compute
```

When executing the previous command, you will be asked for entropy. It is important that you introduce a large and completely random input for optimal security. Some examples of entropy sources are:

- Randomly typing on your keyboard with the eyes closed.
- Take random words from random pages in random books or magazines you have at home.
- Generate various values using `/dev/urandom` in different devices.

Such input **must not** be copied or stored anywhere.

It will take some time to compute a `response` file containing your contribution. Once finished, you will need to submit your contribution.

## Submit your contribution

To submit your contribution, provide the following to the coordinator:

- The generated `response` file.
- A `report` file containing the following information:
  - Brief description of the device used.
  - The time taken to compute the contribution.
  - The hash of the challenge you used.
  - The hash of the response file.
  - Brief description of the entropy source you used and any precautions you took to ensure that it's not possible to recover the value you introduced.
- One of the following ways to verify your identity:
  - A signature of the `report` file, and the link to a public profile containing the public key (e.g. the GPG key in your GitHub account).
  - A link to a post on a social network where you publicly published your report.
