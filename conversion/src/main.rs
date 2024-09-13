// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright (c) DUSK NETWORK. All rights reserved.

//! This code serves the purpose of converting to a dusk_plonk suitable format
//! the Trusted Setup response file computed using this code:
//!
//! https://github.com/dusk-network/powersoftau.git

use dusk_bls12_381::{G1Affine, G2Affine};
use std::{fmt::Error, fs, fs::File, io::Write};

// The maximum number of tau powers computed in the Dusk Trusted Setup
const FILE_MAX_TAU_POWERS: usize = 1 << 21;

// The maximum number of tau powers needed for Dusk Blockchain circuits
const DUSK_MAX_TAU_POWERS: usize = 1 << 17;

// The size of G1 affine points in compressed form
const G1_AFFINE_COMPRESSED_SIZE: usize = 48;

// The size of G2 affine points in compressed form
const G2_AFFINE_COMPRESSED_SIZE: usize = 96;

fn main() -> Result<(), Error> {
    // We read the "response" file from disk
    println!("Reading response file...");
    let response_bytes = fs::read("response").expect("Response file not found.");

    // We read the powers of tau in G1
    println!("Reading powers of tau...");
    let mut powers_of_tau_g1: Vec<u8> = vec![];
    let mut it_size = 64; // We start at 64 since the response included a hash at the beginning

    for _ in 0..DUSK_MAX_TAU_POWERS {
        powers_of_tau_g1
            .extend_from_slice(&response_bytes[it_size..it_size + G1_AFFINE_COMPRESSED_SIZE]);
        it_size += G1_AFFINE_COMPRESSED_SIZE;
    }

    // We read the generator of G2
    println!("Reading G2 generator...");
    it_size = ((FILE_MAX_TAU_POWERS << 1) - 1) * G1_AFFINE_COMPRESSED_SIZE + 64;
    let g2 = &response_bytes[it_size..it_size + G2_AFFINE_COMPRESSED_SIZE];

    // We read the generator of G2 * tau
    println!("Reading tau * G2 generator...");
    it_size += G2_AFFINE_COMPRESSED_SIZE;
    let tau_g2 = &response_bytes[it_size..it_size + G2_AFFINE_COMPRESSED_SIZE];

    // We perform some basic consistency checks
    let g1 = &powers_of_tau_g1[..G1_AFFINE_COMPRESSED_SIZE];
    if G1Affine::from_compressed(g1.try_into().expect("Failed to convert to slice."))
        .expect("Failed to decompress G1 point.")
        != G1Affine::generator()
    {
        panic!("No consistency.");
    };

    if G2Affine::from_compressed(g2.try_into().expect("Failed to convert to slice."))
        .expect("Failed to decompress G2 point.")
        != G2Affine::generator()
    {
        panic!("No consistency.");
    };

    // Write OpeningKey bytes to a new vector
    println!("Writing Trusted Setup to disk (./dusk-trusted-setup)...");
    let mut pp_bytes = G1Affine::generator().to_compressed().to_vec();
    pp_bytes.extend_from_slice(g2);
    pp_bytes.extend_from_slice(tau_g2);

    // Write CommitKey bytes to the previous vector
    pp_bytes.extend_from_slice(&powers_of_tau_g1);

    // Write serialization to disk in dusk_plonk format
    let mut file = File::create("dusk-trusted-setup").unwrap();
    file.write_all(&pp_bytes)
        .expect("Trusted setup failed to be written to disk.");

    println!("All good.");
    Ok(())
}
