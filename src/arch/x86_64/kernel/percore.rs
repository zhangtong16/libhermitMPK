// Copyright (c) 2017 Colin Finck, RWTH Aachen University
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use arch::x86_64::kernel::{BOOT_INFO, BootInfo};
use arch::x86_64::kernel::copy_safe::*;
use arch::x86_64::kernel::processor;
use core::{intrinsics, ptr, mem};
use scheduler::PerCoreScheduler;
use mm;
use x86::bits64::task::TaskStateSegment;
use x86::msr::*;

pub static mut PERCORE: PerCoreVariables = PerCoreVariables::new(0);

pub struct PerCoreVariables {
	/// Sequential ID of this CPU Core.
	core_id: PerCoreVariable<usize>,
	/// Scheduler for this CPU Core.
	scheduler: PerCoreVariable<*mut PerCoreScheduler>,
	/// Task State Segment (TSS) allocated for this CPU Core.
	pub tss: PerCoreVariable<*mut TaskStateSegment>,
}

impl PerCoreVariables {
	pub const fn new(core_id: usize) -> Self {
		Self {
			core_id: PerCoreVariable::new(core_id),
			scheduler: PerCoreVariable::new(ptr::null_mut() as *mut PerCoreScheduler),
			tss: PerCoreVariable::new(ptr::null_mut() as *mut TaskStateSegment),
		}
	}
}

#[repr(C)]
pub struct PerCoreVariable<T> {
	data: T,
}

pub trait PerCoreVariableMethods<T> {
	unsafe fn get(&self) -> T;
	unsafe fn set(&self, value: T);
}

impl<T> PerCoreVariable<T> {
	const fn new(value: T) -> Self {
		Self { data: value }
	}

	#[inline]
	unsafe fn offset(&self) -> usize {
		let base = &PERCORE as *const _ as usize;
		let field = self as *const _ as usize;
		field - base
	}
}

// Treat all per-core variables as 64-bit variables by default. This is true for u64, usize, pointers.
// Implement the PerCoreVariableMethods trait functions using 64-bit memory moves.
// The functions are implemented as default functions, which can be overriden in specialized implementations of the trait.
impl<T> PerCoreVariableMethods<T> for PerCoreVariable<T> {
	#[inline]
	default unsafe fn get(&self) -> T {
		let value: T;
		asm!("movq %gs:($1), $0" : "=r"(value) : "r"(self.offset()) :: "volatile");
		value
	}

	#[inline]
	default unsafe fn set(&self, value: T) {
		asm!("movq $0, %gs:($1)" :: "r"(value), "r"(self.offset()) :: "volatile");
	}
}

// Define and implement a trait to mark all 32-bit variables used inside PerCoreVariables.
pub trait Is32BitVariable {}
impl Is32BitVariable for u32 {}

// For all types implementing the Is32BitVariable trait above, implement the PerCoreVariableMethods
// trait functions using 32-bit memory moves.
impl<T: Is32BitVariable> PerCoreVariableMethods<T> for PerCoreVariable<T> {
	#[inline]
	unsafe fn get(&self) -> T {
		let value: T;
		asm!("movl %gs:($1), $0" : "=r"(value) : "r"(self.offset()) :: "volatile");
		value
	}

	#[inline]
	unsafe fn set(&self, value: T) {
		asm!("movl $0, %gs:($1)" :: "r"(value), "r"(self.offset()) :: "volatile");
	}
}

#[cfg(not(test))]
#[inline]
pub fn core_id() -> usize {
	//if unsafe{UNSAFE_STORAGE} == 0 {
		unsafe { /* FIXME */
			PERCORE.core_id.get()
		}
	//}
/*
	else {
		unsafe {
			//PERCORE.core_id.get()
			//let offset = (*(UNSAFE_STORAGE as *const PerCoreVariables)).core_id as *const _ as usize - UNSAFE_STORAGE;
			let value: usize;
			copy_from_safe(&PERCORE, mem::size_of::<PerCoreVariables>());
			//isolation_start!();
			//asm!("swapgs" :::: "volatile");
			clear_unsafe_storage();
			//isolation_start!();
			value
		}
	}
*/
}

#[cfg(test)]
pub fn core_id() -> usize {
	0
}

#[inline]
pub fn core_scheduler() -> &'static mut PerCoreScheduler {
	unsafe { &mut *PERCORE.scheduler.get() }
}

#[inline]
pub fn set_core_scheduler(scheduler: *mut PerCoreScheduler) {
	unsafe {
		PERCORE.scheduler.set(scheduler);
	}
}

pub fn init() {
	unsafe { /* FIXME */
		let address;
		//if UNSAFE_STORAGE == 0 {
		// Store the address to the PerCoreVariables structure allocated for this core in GS.
		address = intrinsics::volatile_load(&(*BOOT_INFO).current_percore_address);
		//}
		/*
		else {
			error!("else");
			copy_from_safe(BOOT_INFO, mem::size_of::<BootInfo>());
			//isolation_start!();
			address = intrinsics::volatile_load(&(*(UNSAFE_STORAGE as *const BootInfo)).current_percore_address);
			//isolation_start!();
			clear_unsafe_storage();
		}
		*/
		if address == 0 {
			wrmsr(IA32_GS_BASE, &PERCORE as *const _ as u64);
			list_add(&PERCORE as *const PerCoreVariables as usize);
		} else {
			wrmsr(IA32_GS_BASE, address as u64);
		}
	}
}
