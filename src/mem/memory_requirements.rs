use ash::vk;
use crate::instance::{
	PhysicalDevice,
	physical_device::MemoryType
};

pub struct MemoryRequirements(pub(crate) vk::MemoryRequirements);

impl MemoryRequirements {
	#[inline]
	pub fn size(&self) -> u64 {
		self.0.size
	}

	#[inline]
	pub fn alignment(&self) -> u64 {
		self.0.alignment
	}

	/// Creates new memory requirements aligned to `self.alignment()` but also on `align`.
	/// 
	/// Align must be a power of 2.
	/// 
	/// ## Panics
	/// 
	/// This function panics if the input `align` value is not a power of 2.
	#[inline]
	pub fn align_to(&self, align: u64) -> MemoryRequirements {
		if !align.is_power_of_two() {
			panic!("alignment value must be a power of 2.")
		}
		
		MemoryRequirements(vk::MemoryRequirements {
			size: self.size(),
			alignment: std::cmp::max(self.alignment(), align), // works because alignemnt values are powers of two.
			memory_type_bits: self.memory_type_bits()
		})
	}

	#[inline]
	pub fn memory_type_bits(&self) -> u32 {
		self.0.memory_type_bits
	}

	#[inline]
	pub fn contains_memory_type_index(&self, index: u32) -> bool {
		self.0.memory_type_bits & (1u32 << index) != 0
	}

	#[inline]
	pub fn filter_memory_types<F>(&self, physical_device: PhysicalDevice, f: F) -> MemoryRequirements where F: Fn(MemoryType) -> bool {
		let mut bits = self.memory_type_bits();
		let mut index = 0;
		let mut new_memory_type_bits = 0;

		while bits != 0 {
			if bits & 1 != 0 {
				let memory_type = physical_device.memory_type(index).expect("no such memory type");
				if f(memory_type) {
					new_memory_type_bits |= 1u32 << index;
				}
			}

			index += 1;
			bits >>= 1;
		}

		MemoryRequirements(vk::MemoryRequirements {
			size: self.size(),
			alignment: self.alignment(),
			memory_type_bits: new_memory_type_bits
		})
	}
}