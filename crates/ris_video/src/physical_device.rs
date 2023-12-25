use std::sync::Arc;

use vulkano::device::physical::PhysicalDevice;
use vulkano::device::physical::PhysicalDeviceType;
use vulkano::device::DeviceExtensions;
use vulkano::device::QueueFlags;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use ris_util::error::RisError;

pub fn select_physical_device(
    instance: &Arc<Instance>,
    surface: &Arc<Surface>,
    device_extensions: &DeviceExtensions,
) -> Result<(Arc<PhysicalDevice>, u32), RisError> {
    let available_devices = 
        ris_util::unroll!(
            instance.enumerate_physical_devices(),
            "failed to enumerate_physical_devices"
        )?
        .filter(|p| p.supported_extensions().contains(device_extensions))
        .filter_map(|p| {
            p.queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, q)| {
                    q.queue_flags.contains(QueueFlags::GRAPHICS)
                        && p.surface_support(i as u32, surface).unwrap_or(false)
                })
                .map(|q| (p, q as u32))
        })
        .collect::<Vec<_>>();

    let mut log_string = format!("{} available video devices:", available_devices.len());
    for (device, i) in available_devices.iter() {
        log_string.push_str(&format!("\n    [{}] => {}", i, device.properties().device_name));
    }

    ris_log::info!("{}", log_string);

    let device = ris_util::unroll_option!(
        available_devices.into_iter()
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        }),
        "no devices available"
    )?;

    ris_log::info!("selected physical video device: {}", device.0.properties().device_name);

    Ok(device)
}
