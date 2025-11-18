

use crate::GlobalError;
use crate::system::attributes::ProcessAttributes;
use crate::system::metrics::Metrics;
use nvml_wrapper::Nvml;
use nvml_wrapper::enums::device::UsedGpuMemory;
use sysinfo::Pid;
use tracing::warn;

/// `GpuCollector` is responsible for collecting GPU memory usage metrics.
pub struct GpuCollector {
    nvml: Nvml,
    pid: Pid,
}

impl GpuCollector {
    pub fn new(pid: Pid) -> Result<Self, GlobalError> {
        let nvml = Nvml::init().map_err(|e| GlobalError::GpuInitError(e.to_string()))?;
        Ok(GpuCollector { nvml, pid })
    }

    pub fn collect(&self, metrics: &Metrics, attributes: &ProcessAttributes) -> Result<(), GlobalError> {
        if let Ok(device) = self.nvml.device_by_index(0) {
            if let Ok(gpu_stats) = device.running_compute_processes() {
                for stat in gpu_stats.iter() {
                    if stat.pid == self.pid.as_u32() {
                        let memory_used = match stat.used_gpu_memory {
                            UsedGpuMemory::Used(bytes) => bytes,
                            UsedGpuMemory::Unavailable => 0,
                        };
                        metrics.gpu_memory_usage.record(memory_used, &attributes.attributes);
                        return Ok(());
                    }
                }
            } else {
                warn!("Could not get GPU stats, recording 0 for GPU memory usage");
            }
        } else {
            return Err(GlobalError::GpuDeviceError("No GPU device found".to_string()));
        }
        metrics.gpu_memory_usage.record(0, &attributes.attributes);
        Ok(())
    }
}
