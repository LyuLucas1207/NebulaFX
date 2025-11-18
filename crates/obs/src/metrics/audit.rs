

#![allow(dead_code)]

/// audit related metric descriptors
///
/// This module contains the metric descriptors for the audit subsystem.
use crate::{MetricDescriptor, MetricName, new_counter_md, new_gauge_md, subsystems};
use std::sync::LazyLock;

const TARGET_ID: &str = "target_id";
pub const RESULT: &str = "result"; // success / failure
pub const STATUS: &str = "status"; // success / failure

pub const SUCCESS: &str = "success";
pub const FAILURE: &str = "failure";

pub static AUDIT_FAILED_MESSAGES_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::AuditFailedMessages,
        "Total number of messages that failed to send since start",
        &[TARGET_ID],
        subsystems::AUDIT,
    )
});

pub static AUDIT_TARGET_QUEUE_LENGTH_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::AuditTargetQueueLength,
        "Number of unsent messages in queue for target",
        &[TARGET_ID],
        subsystems::AUDIT,
    )
});

pub static AUDIT_TOTAL_MESSAGES_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_counter_md(
        MetricName::AuditTotalMessages,
        "Total number of messages sent since start",
        &[TARGET_ID],
        subsystems::AUDIT,
    )
});
