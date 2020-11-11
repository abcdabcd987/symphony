use crate::common::{ModelSession, NodeId, QueryId};
use crate::msg_capnp;
use capnp::message::TypedReader;
use capnp::serialize::OwnedSegments;

pub struct Query {
    query_id: QueryId,
    model_session: ModelSession,
    frontend_id: NodeId,
    deadline: chrono::DateTime<chrono::Utc>,
    model_input: Vec<f32>,
    model_output: Vec<f32>,

    source_clock: TypedReader<OwnedSegments, msg_capnp::query_punch_clock::Owned>,
}
