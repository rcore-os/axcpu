#[macro_use]
mod macros;

mod context;
mod trap;

pub mod asm;

#[cfg(feature = "uspace")]
pub mod uspace;

pub use self::context::{TaskContext, TrapFrame};

/// Initializes CPU states on the current CPU.
pub fn cpu_init() {
    #[cfg(feature = "fp_simd")]
    loongArch64::register::euen::set_fpe(true);

    unsafe extern "C" {
        fn exception_entry_base();
    }
    unsafe {
        asm::write_exception_entry_base(exception_entry_base as usize);
    }
}
