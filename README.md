> **Fork Notice:** This is a fork of [`libp2p-bitswap`](https://github.com/ipfs-rust/libp2p-bitswap)
> published as [`co-libp2p-bitswap`](https://crates.io/crates/co-libp2p-bitswap) on crates.io for use in [COKIT](https://www.cokit.org).
> See [Divergence](#divergence) below for details on changes from upstream.

# co-libp2p-bitswap

Implementation of the bitswap protocol.

## Efficiently syncing dags of blocks

Bitswap is a very simple protocol. It was adapted and simplified for ipfs-embed. The message
format can be represented by the following enums.

```rust
pub enum BitswapRequest {
    Have(Cid),
    Block(Cid),
}

pub enum BitswapResponse {
    Have(bool),
    Block(Vec<u8>),
}
```

The mechanism for locating providers can be abstracted. A dht can be plugged in or a centralized
db query. The bitswap api looks as follows:

```rust
#[derive(Debug)]
pub enum BitswapEvent {
    /// Received a block from a peer. Includes the number of known missing blocks for a
    /// sync query. When a block is received and missing blocks is not empty the counter
    /// is increased. If missing blocks is empty the counter is decremented.
    Progress(QueryId, usize),
    /// A get or sync query completed.
    Complete(QueryId, Result<()>),
}

pub trait BitswapStore: Send + Sync + 'static {
    /// A have query needs to know if the block store contains the block.
    async fn contains(&mut self, cid: &Cid, remote_peer: &PeerId, tokens: &[Token]) -> Result<bool>;
    /// A block query needs to retrieve the block from the store.
    async fn get(&mut self, cid: &Cid, remote_peer: &PeerId, tokens: &[Token]) -> Result<Option<Vec<u8>>>;
    /// A block response needs to insert the block into the store.
    async fn insert(&mut self, block: &Block, remote_peer: &PeerId, tokens: &[Token]) -> Result<()>;
    /// A sync query needs a list of missing blocks to make progress.
    async fn missing_blocks(&mut self, cid: &Cid, tokens: &[Token]) -> Result<Vec<Cid>>;
}

pub struct BitswapConfig {
    /// Timeout of a request.
    pub request_timeout: Duration,
    /// The upper bound for the number of concurrent inbound + outbound streams.
    pub max_concurrent_streams: usize,
    /// Maximum accepted block size in bytes.
    pub max_block_size: usize,
}

impl Bitswap {
    /// Creates a new `Bitswap` behaviour.
    pub fn new<S: BitswapStore>(config: BitswapConfig, store: S, executor: Box<dyn FnOnce(BoxFuture<'static, ()>)>) -> Self;

    /// Adds an address for a peer.
    pub fn add_address(&mut self, peer_id: &PeerId, addr: Multiaddr);

    /// Removes an address for a peer.
    pub fn remove_address(&mut self, peer_id: &PeerId, addr: &Multiaddr);

    /// Starts a get query with an initial guess of providers.
    pub fn get(&mut self, cid: Cid, peers: impl IntoIterator<Item = PeerId>, tokens: impl IntoIterator<Item = Token>) -> QueryId;

    /// Starts a sync query with an the initial set of missing blocks.
    pub fn sync(&mut self, cid: Cid, peers: Vec<PeerId>, missing: impl IntoIterator<Item = Cid>, tokens: impl IntoIterator<Item = Token>) -> QueryId;

    /// Cancels an in progress query. Returns true if a query was cancelled.
    pub fn cancel(&mut self, id: QueryId) -> bool;

    /// Register bitswap stats in a prometheus registry.
    #[cfg(feature = "metrics")]
    pub fn register_metrics(&self, registry: &Registry) -> Result<()>;
}
```

So what happens when you create a get request? First all the providers in the initial set
are queried with the have request. As an optimization, in every batch of queries a block
request is sent instead. If the get query finds a block it returns a query complete. If the
block wasn't found in the initial set, a `Providers` event is emitted. This is where
the bitswap consumer tries to locate providers by for example performing a dht lookup. After
the locating of providers completes, it is signaled by calling `inject_providers`. The query
manager then performs bitswap requests using the new provider set which results in the block
being found or a `BlockNotFound` error.

Often we want to sync an entire dag of blocks. We can efficiently sync dags of blocks by adding
a sync query that runs get queries in parallel for all the references of a block. The set of
providers that had a block is used as the initial set in a reference query.

## Divergence

This fork diverges from upstream [`libp2p-bitswap`](https://github.com/ipfs-rust/libp2p-bitswap) v0.25.0.
See the [CHANGELOG](./CHANGELOG.md) for the full per-release delta, including breaking changes and migration notes.

## License

MIT OR Apache-2.0
