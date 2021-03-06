use ash::vk;
use crate::pipeline;
use super::{
	task,
	fence
};

/// GPU future.
pub unsafe trait Future {
	/// Semaphore signaled when the future finishes.
	fn signal_semaphore(&self) -> Option<&vk::Semaphore> {
		None
	}

	/// Fence signaled when the future finishes.
	fn signal_fence(&self) -> Option<&vk::Fence> {
		None
	}
}

/// Group of GPU futures.
pub unsafe trait Futures {
	/// Semaphores signaled by the futures.
	/// 
	/// If not `None`, then each underlying future signals at least one of the returned semaphores.
	/// Otherwise, a fence is signaled when all the futures are done.
	fn signal_semaphores(&self) -> Option<&[vk::Semaphore]>;

	/// Fence signaled when *all* the futures are done.
	/// 
	/// If `None`, then each underlying future signals a semaphore.
	fn signal_fence(&self) -> Option<&vk::Fence>;
}

unsafe impl<F: Future> Futures for F {
	fn signal_semaphores(&self) -> Option<&[vk::Semaphore]> {
		self.signal_semaphore().map(std::slice::from_ref)
	}

	fn signal_fence(&self) -> Option<&vk::Fence> {
		Future::signal_fence(self)
	}
}

pub trait SignalSemaphore: Future {
	#[inline]
	fn semaphore(&self) -> &vk::Semaphore {
		Future::signal_semaphore(self).unwrap()
	}

	#[inline]
	fn and_then<T: task::Wait>(self, task: T) -> task::Delayed<Self, T> where Self: Sized {
		task::Delayed::new(self, task)
	}
}

pub trait SignalFence: Futures {
	#[inline]
	fn fence(&self) -> &vk::Fence {
		Futures::signal_fence(self).unwrap()
	}

	fn wait(self, timeout: Option<u64>) -> Result<(), fence::WaitError>;

	fn is_signaled(&self) -> Result<bool, fence::DeviceLost>;
}

pub trait SignalSemaphores {
	fn semaphores(&self) -> &[vk::Semaphore];

	#[inline]
	fn and_then_pipeline_stages_of<T: task::WaitPipelineStages>(self, task: T, wait_pipeline_stage_mask: pipeline::stage::Flags) -> task::DelayedPipelineStages<Self, T> where Self: Sized {
		task::DelayedPipelineStages::new(self, task, wait_pipeline_stage_mask)
	}
}

impl<F: SignalSemaphore> SignalSemaphores for F {
	fn semaphores(&self) -> &[vk::Semaphore] {
		std::slice::from_ref(self.semaphore())
	}
}