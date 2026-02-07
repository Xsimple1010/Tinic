use crate::video::{RawTextureData, RetroVideoAPi};
use retro_core::av_info::Geometry;
use std::{ptr::null, sync::Arc};
use vulkano::{
    device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags},
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    VulkanLibrary,
};

struct RetroVulkan {
    device: Arc<Device>,
}

impl RetroVideoAPi for RetroVulkan {
    fn request_redraw(&self) {}

    fn draw_new_frame(&self, texture: &RawTextureData, geo: &Geometry) {}

    fn get_proc_address(&self, proc_name: &str) -> *const () {
        null()
    }

    fn set_full_screen(&mut self, mode: winit::window::Fullscreen) {}

    fn context_destroy(&mut self) {}

    fn context_reset(&mut self) {}
}

impl RetroVulkan {
    pub fn new() -> Self {
        let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("failed to create instance");

        let physical_device = instance
            .enumerate_physical_devices()
            .expect("could not enumerate devices")
            .next()
            .expect("no devices available");

        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(_queue_family_index, queue_family_properties)| {
                queue_family_properties
                    .queue_flags
                    .contains(QueueFlags::GRAPHICS)
            })
            .expect("couldn't find a graphical queue family")
            as u32;

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                // here we pass the desired queue family to use by index
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .expect("failed to create device");

        Self { device }
    }
}
