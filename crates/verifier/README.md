# SP1 Verifier

This crate contains primitives for verifying SP1 proofs generated using the [SP1 SDK](../sdk).

It is split into the following modules:
- `compressed`: Verifiers for SP1 "compressed" proofs.
- `groth16`: Verifiers for Groth16 proofs.
- `plonk`: Verifiers for Plonk proofs.


## no_std fork: on-chain verification

Upstream `sp1-verifier` claims `no_std`, but its compressed/recursion path drags in std-only crates
(`zkhash`→`bls12_381`→`getrandom`, `ahash`→`once_cell`) unconditionally — so it won't compile into a
runtime, even for Groth16 alone.

This fork gates that path (`compressed`, `proof`, `recursion_vks`) behind an off-by-default
`recursion` feature, leaving a lean no_std Groth16/Plonk verifier. The one recursion-derived value
Groth16 needs — the 32-byte verifying-key root — is an embedded constant, re-derived and asserted by a
`--features recursion` test. Cryptography unchanged: same v6.1.0 release, keys, trusted setup and
checks; proofs verify identically upstream.

## Features

Default build: Groth16 + Plonk in `no_std`. Compressed (recursion) verification is behind the
`recursion` feature and needs `std`. zkVM-context verification is patched to use the
[bn254 precompiles](https://blog.succinct.xyz/succinctshipsprecompiles/).

### Pre-generated verification keys

Verification keys for Groth16 and Plonk are stored in the [`vk-artifacts`](./vk-artifacts/) directory. These
vkeys are used to verify all SP1 proofs.

These vkeys are the same as those found locally in
`~/.sp1/circuits/<circuit_name>/<version>/<circuit_name>_vk.bin`, and should be automatically
updated after every release.

## Tests

Run tests with the following command:

```sh
cargo test --package sp1-verifier
```

These tests generate a groth16/plonk proof and verify it.

## Acknowledgements

Adapted from [@Bisht13's](https://github.com/Bisht13/gnark-bn254-verifier) `gnark-bn254-verifier` crate.
