# Trusted setup for BLS12-381

This repository contains a multi-party computation (MPC) ceremony to compute the powers of tau needed by different proof systems, using the BLS12-381 elliptic curve construction. In particular, we compute up to 2**21 powers of tau, taking as a starting point the ceremony performed by Zcash, that can be found [here](https://github.com/ZcashFoundation/powersoftau-attestations/tree/master).

## The ceremony

First, an open call for participation will be made through Dusk socials. For transparency purposes, some information will be required (real name, link to social network profile, etc.). Then, a Dusk team member will be chosen as a coordinator for the ceremony. The coordinator will verify the contributions of Zcash up to number 87, and will create a new challenge following these [instructions](instructions/VERIFY.md).

The list of participants will be sorted, and the coordinator will contact the first participant providing the previously computed challenge. The first participant will compute their contribution using these [instructions](instructions/CONTRIBUTE.md), and will send the response back to the coordinator. The coordinator will follow these [instructions](instructions/COORDINATOR.md) to verify the contribution and its integrity. In this step, the coordinator computes a new challenge as well, and the same process is repeated with the next participant.

All contributions will be available in the folder `contributions`.

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT).
