#[macro_use]
mod macros;

mod context;
mod trap;

pub mod asm;

#[cfg(feature = "uspace")]
pub mod uspace;

pub use self::context::{GeneralRegisters, TaskContext, TrapFrame};

/// Initializes CPU states on the current CPU.
///
/// On RISC-V, it sets the trap vector base address.
pub fn cpu_init() {
    unsafe extern "C" {
        fn trap_vector_base();
    }
    unsafe {
        asm::write_trap_vector_base(trap_vector_base as usize);
    }
}
