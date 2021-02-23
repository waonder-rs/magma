use std::sync::Arc;
use ash::vk;
use crate::{
	device,
	Device,
	DeviceOwned
};

pub struct SharingQueues {
	device: Arc<Device>,
	queues: Vec<u32>
}

impl SharingQueues {
	pub(crate) fn as_vulkan(&self) -> (vk::SharingMode, u32, *const u32) {
		if self.queues.len() > 1 {
			(vk::SharingMode::EXCLUSIVE, 0, std::ptr::null())
		} else {
			(vk::SharingMode::CONCURRENT, self.queues.len() as u32, self.queues.as_ptr())
		}
	}

	pub fn contains(&self, queue: &device::Queue) -> bool {
		&self.device == queue.device() && self.queues.contains(&queue.index())
	}

	pub fn insert(&mut self, queue: &device::Queue) -> bool {
		if !self.contains(queue) {
			assert_eq!(self.device, *queue.device());
			self.queues.push(queue.index());
			true
		} else {
			false
		}
	}
}

impl<'a, I: IntoIterator<Item=&'a device::Queue>> From<I> for SharingQueues {
	fn from(it: I) -> Self {
		let mut device = None;

		let mut ids: Vec<_> = it.into_iter().map(|q| {
			match &device {
				Some(dev) => assert_eq!(dev, q.device()),
				None => device = Some(q.device().clone())
			}

			q.family_index()
		}).collect();
		ids.sort();
		ids.dedup();

		SharingQueues {
			device: device.unwrap(),
			queues: ids
		}
	}
}