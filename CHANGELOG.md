# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **BREAKING:** replaced the [deprecated](https://github.com/ipld/libipld/pull/198) `libipld = "0.16"` dependency with the modern
  successor crates `cid 0.11`, `ipld-core 0.4`, `ipld-dagpb 0.2`, `multihash 0.19`, and
  `multihash-codetable 0.2`.
- **BREAKING:** introduced thin in-crate replacements for the libipld-provided types in a
  new `src/block.rs` module:
  - `Block` — holds a `Cid` + `Vec<u8>`; `new` verifies the hash, `new_unchecked` skips
    verification.
  - `BlockNotFound(pub Cid)` — replaces `libipld::error::BlockNotFound`.
  - `InvalidMultihash` — returned by `Block::new` when the data does not hash to the cid
    or the multihash code is not in the shipped codetable.
- **BREAKING:** dropped the `StoreParams` trait and `DefaultParams` struct. `Block`,
  `Bitswap`, `BitswapCodec`, `BitswapStore`, and the internal `DbRequest` are no longer
  generic over a `StoreParams` parameter.
- **BREAKING:** moved the wire-level block-size limit from the compile-time
  `StoreParams::MAX_BLOCK_SIZE` associated constant to a runtime
  `BitswapConfig::max_block_size: usize` field (default `1_048_576` bytes, matches the
  previous `DefaultParams` value). `BitswapCodec` carries the limit as a field populated
  via `BitswapCodec::new(max_block_size)`, and `Bitswap::new` now uses
  `RequestResponse::with_codec` to install the configured codec.
- internal: switched `#[cfg(test)]` tests that used `libipld::cbor::DagCborCodec` and
  `libipld::ipld!` to `serde_ipld_dagcbor::codec::DagCborCodec` and `ipld_core::ipld!`.

### Migration

Downstream code that implements `BitswapStore`:

```rust
// before
use libipld::{Block, Cid, Result, store::{DefaultParams, StoreParams}};

#[async_trait]
impl BitswapStore for MyStore {
    type Params = DefaultParams;
    async fn insert(&mut self, block: &Block<Self::Params>, …) -> Result<()> { … }
    …
}

// after
use anyhow::Result;
use cid::Cid;
use co_libp2p_bitswap::Block;

#[async_trait]
impl BitswapStore for MyStore {
    async fn insert(&mut self, block: &Block, …) -> Result<()> { … }
    …
}
```

Downstream code that constructs `Bitswap`:

```rust
// before
Bitswap::<DefaultParams>::new(BitswapConfig::new(), store, executor);

// after (1 MiB default, matches previous behavior)
Bitswap::new(BitswapConfig::new(), store, executor);

// after (custom limit)
let config = BitswapConfig { max_block_size: 2 * 1024 * 1024, ..BitswapConfig::new() };
Bitswap::new(config, store, executor);
```

## [0.26.1] - 2025-04-09

### Fixed

- `docs.rs` build.

## [0.26.0] - 2025-04-09

Initial release of the `co-libp2p-bitswap` fork. Cumulative delta against upstream
[`libp2p-bitswap`](https://github.com/ipfs-rust/libp2p-bitswap) v0.25.0:

### Added

- `wasm-bindgen` feature for browser WASM support.
- `metrics` feature with prometheus instrumentation.
- Authentication support: pass remote `PeerId` to `get`/`contains` calls.

### Changed

- Updated libp2p through versions 0.51 to 0.56.
- Migrated protobuf codegen to `quick-protobuf`.

### Fixed

- Limit token sending to 1 MB cumulative size.
