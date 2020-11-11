mod profile;

pub mod utils;
pub use profile::*;

pub struct NodeId(u64);
pub struct PlanId(u64);
pub struct QueryId(u64);
pub struct TimestampNs(i64);

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct ModelSession {
    framework: String,
    model: String,
    slo_ms: u32,
}
