mod context;

pub mod asm;

#[cfg(target_os = "none")]
mod trap;

#[cfg(feature = "uspace")]
pub mod uspace;

pub use self::context::{FpState, TaskContext, TrapFrame};

/// Initializes CPU states on the current CPU.
///
/// On AArch64, it sets the exception vector base address (`VBAR_EL1`).
pub fn cpu_init() {
    unsafe extern "C" {
        fn exception_vector_base();
    }
    unsafe {
        asm::write_exception_vector_base(exception_vector_base as usize);
    }
}
