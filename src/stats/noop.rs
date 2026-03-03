#[derive(Debug)]
pub struct NoopTimer;

pub struct NoopCounterVec;
impl NoopCounterVec {
    pub fn with_label_values(&self, _: &[&str]) -> NoopCounter {
        NoopCounter
    }
}

pub struct NoopCounter;
impl NoopCounter {
    pub fn inc(&self) {}
    pub fn inc_by(&self, _: u64) {}
}

pub struct NoopHistogramVec;
impl NoopHistogramVec {
    pub fn with_label_values(&self, _: &[&str]) -> NoopHistogram {
        NoopHistogram
    }
}

pub struct NoopHistogram;
impl NoopHistogram {
    pub fn start_timer(&self) -> NoopTimer {
        NoopTimer
    }
}

pub static REQUESTS_TOTAL: NoopCounterVec = NoopCounterVec;
pub static REQUEST_DURATION_SECONDS: NoopHistogramVec = NoopHistogramVec;
pub static REQUESTS_CANCELED: NoopCounter = NoopCounter;
pub static BLOCK_NOT_FOUND: NoopCounter = NoopCounter;
pub static PROVIDERS_TOTAL: NoopCounter = NoopCounter;
pub static MISSING_BLOCKS_TOTAL: NoopCounter = NoopCounter;
pub static RECEIVED_BLOCK_BYTES: NoopCounter = NoopCounter;
pub static RECEIVED_INVALID_BLOCK_BYTES: NoopCounter = NoopCounter;
pub static SENT_BLOCK_BYTES: NoopCounter = NoopCounter;
pub static RESPONSES_TOTAL: NoopCounterVec = NoopCounterVec;
pub static THROTTLED_INBOUND: NoopCounter = NoopCounter;
pub static THROTTLED_OUTBOUND: NoopCounter = NoopCounter;
pub static OUTBOUND_FAILURE: NoopCounterVec = NoopCounterVec;
pub static INBOUND_FAILURE: NoopCounterVec = NoopCounterVec;
