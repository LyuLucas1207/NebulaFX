

#![allow(dead_code)]

/// Cluster health-related metric descriptors
use crate::{MetricDescriptor, MetricName, new_gauge_md, subsystems};
use std::sync::LazyLock;

pub static HEALTH_DRIVES_OFFLINE_COUNT_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::HealthDrivesOfflineCount,
        "Count of offline drives in the cluster",
        &[],
        subsystems::CLUSTER_HEALTH,
    )
});

pub static HEALTH_DRIVES_ONLINE_COUNT_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::HealthDrivesOnlineCount,
        "Count of online drives in the cluster",
        &[],
        subsystems::CLUSTER_HEALTH,
    )
});

pub static HEALTH_DRIVES_COUNT_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::HealthDrivesCount,
        "Count of all drives in the cluster",
        &[],
        subsystems::CLUSTER_HEALTH,
    )
});
