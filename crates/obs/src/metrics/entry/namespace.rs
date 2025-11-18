

/// The metric namespace, which represents the top-level grouping of the metric
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MetricNamespace {
    NebulaFX,
}

impl MetricNamespace {
    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NebulaFX => "nebulafx",
        }
    }
}
