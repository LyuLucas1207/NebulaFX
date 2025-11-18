

#![allow(dead_code)]

/// Metric descriptors related to cluster configuration
use crate::{MetricDescriptor, MetricName, new_gauge_md, subsystems};

use std::sync::LazyLock;

pub static CONFIG_RRS_PARITY_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::ConfigRRSParity,
        "Reduced redundancy storage class parity",
        &[],
        subsystems::CLUSTER_CONFIG,
    )
});

pub static CONFIG_STANDARD_PARITY_MD: LazyLock<MetricDescriptor> = LazyLock::new(|| {
    new_gauge_md(
        MetricName::ConfigStandardParity,
        "Standard storage class parity",
        &[],
        subsystems::CLUSTER_CONFIG,
    )
});
