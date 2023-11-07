
# Coordinate the ceremony

## Prerequisites

You will need Rust to compile the code that verifies the contributions. You can follow the recommended [guide](https://www.rust-lang.org/tools/install).

## Coordinate the participants

First, verify the [contributions](https://github.com/dusk-network/powersoftau-attestations) of Zcash up to number 87 using this [guide](VERIFY.md). This will output a challenge file you need to send to the first participant on the list.

Once you get the response from the participant, you will need to do the following:

- Check the integrity of the file using the provided hash.
- Verify that the contribution has been performed using the correct challenge. To do so, place the response from the participant into the root directory of the the code, where you must have the challenge you sent them, and verify as follows:

```
cargo run --release --bin verify_transform 
```
Once verified, a new file `new_challenge` will be generated. Rename it to `challenge` and send it to the next participant on the list.