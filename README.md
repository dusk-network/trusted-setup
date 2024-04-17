# Trusted setup for BLS12-381

This repository contains a multi-party computation (MPC) ceremony to compute the powers of tau needed by different proof systems, using the BLS12-381 elliptic curve construction. In particular, we compute up to 2**21 powers of tau, taking as a starting point the ceremony performed by Zcash, that can be found [here](https://github.com/ZcashFoundation/powersoftau-attestations/tree/master).

## The ceremony

First, an open call for participation will be made through Dusk socials. For transparency purposes, some information will be required (real name, link to social network profile, etc.). Then, a Dusk team member will be chosen as a coordinator for the ceremony. The coordinator will verify the contributions of Zcash up to number 87, and will create a new challenge following these [instructions](instructions/VERIFY.md).

The list of participants will be sorted, and the coordinator will contact the first participant providing the previously computed challenge. The first participant will compute their contribution using these [instructions](instructions/CONTRIBUTE.md), and will send the response back to the coordinator. The coordinator will follow these [instructions](instructions/COORDINATOR.md) to verify the contribution and its integrity. In this step, the coordinator computes a new challenge as well, and the same process is repeated with the next participant.

All contributions will be available in the folder `contributions`.


## Contributions

The starting challenge is computed from the verification of the Zcash response number 87, and can be directly downloaded from [here](https://drive.google.com/file/d/1-rrLqAjshpEJaGybl_traBnpAQazWjTs/view?usp=sharing).

| #    | Contributor    | Contribution                                | Public Profile                                           |
| ---- | ----           | ----                                        | ----                                                     |
| 0001 | MCFValidator   | [contributions/0001/](contributions/0001/)  | [Twitter](https://twitter.com/MCFvalidator)              |
| 0002 | m_arcel        | [contributions/0002/](contributions/0002/)  | [Twitter](https://twitter.com/duskdart)                  | 
| 0003 | Cyborgjox      | [contributions/0003/](contributions/0003/)  | [Twitter](https://twitter.com/cyborgjox)                 | 
| 0004 | Milkington     | [contributions/0004/](contributions/0004/)  | [Twitter](https://twitter.com/Paphahghkhan)              | 
| 0005 | Duskanor       | [contributions/0005/](contributions/0005/)  | [Twitter](https://twitter.com/Guv_Duskanor)              | 
| 0006 | Xavier G.      | [contributions/0006/](contributions/0006/)  | [Twitter](https://twitter.com/UserNotAvailableRightNow)  | 
| 0007 | sshmaxime      | [contributions/0007/](contributions/0007/)  | [GitHub](https://github.com/sshmaxime)                   | 
| 0008 | Elviro Junior  | [contributions/0008/](contributions/0008/)  | [Twitter](https://twitter.com/e_viruz)                    |
| 0009 | TMiNus         | [contributions/0009/](contributions/0009/)  | [GitHub](https://github.com/tminus)                      | 
| 0010 | MoCello         | [contributions/0010/](contributions/0010/)  | [GitHub](https://github.com/moCello)                      | 

## License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT).
