use crate::{Color, trace_stats::TraceStats};

pub(crate) struct TracePayload {
    pub(crate) color: Color,
    pub(crate) stats: TraceStats,
}
