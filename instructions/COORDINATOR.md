
# Coordinate the ceremony

## Prerequisites

You will need Rust to compile the code that verifies the contributions. You can follow the recommended [guide](https://www.rust-lang.org/tools/install).

## Coordinate the participants

First, verify the [contributions](https://github.com/dusk-network/powersoftau-attestations) of Zcash up to number 87 using this [guide](VERIFY.md). The last step of this verification will output a challenge file you need to send to the first participant on the list.

Once you get the submission from the participant, you will need to do the following:

- Check the identity of the participant by verifying the provided signature or the provided post on a social network.
- Check the integrity of the `response` file using the provided hash.
- Verify that the contribution has been performed using the correct challenge. To do so, first place the response from the participant into the root directory of the the code, where you must also have the challenge you sent them. Then, verify as follows:

```
cargo run --release --bin verify_transform 
```
Once the contribution has been verified, the same instruction will generate a new file named `new_challenge`. Rename it to `challenge` and send it to the next participant on the list.
