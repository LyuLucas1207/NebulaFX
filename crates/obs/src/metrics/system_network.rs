

#![allow(dead_code)]

/// Network-related metric descriptors
///
/// These metrics capture internode network communication statistics including:
/// - Error counts for connection and general internode calls
/// - Network dial performance metrics
/// - Data transfer volume in both directions
use crate::{MetricDescriptor, MetricName, new_counter_md, new_gauge_md, subsystems};
use std::sync::LazyLock;

/// Total number of failed internode calls counter
pub static INTERNODE_ERRORS_TOTAL_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::InternodeErrorsTotal,
        "Total number of failed internode calls",
        &[],
        subsystems::SYSTEM_NETWORK_INTERNODE,
    )
});

/// TCP dial timeouts and errors counter
pub static INTERNODE_DIAL_ERRORS_TOTAL_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::InternodeDialErrorsTotal,
        "Total number of internode TCP dial timeouts and errors",
        &[],
        subsystems::SYSTEM_NETWORK_INTERNODE,
    )
});

/// Average dial time gauge in nanoseconds
pub static INTERNODE_DIAL_AVG_TIME_NANOS_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::InternodeDialAvgTimeNanos,
        "Average dial time of internode TCP calls in nanoseconds",
        &[],
        subsystems::SYSTEM_NETWORK_INTERNODE,
    )
});

/// Outbound network traffic counter in bytes
pub static INTERNODE_SENT_BYTES_TOTAL_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::InternodeSentBytesTotal,
        "Total number of bytes sent to other peer nodes",
        &[],
        subsystems::SYSTEM_NETWORK_INTERNODE,
    )
});

/// Inbound network traffic counter in bytes
pub static INTERNODE_RECV_BYTES_TOTAL_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::InternodeRecvBytesTotal,
        "Total number of bytes received from other peer nodes",
        &[],
        subsystems::SYSTEM_NETWORK_INTERNODE,
    )
});
