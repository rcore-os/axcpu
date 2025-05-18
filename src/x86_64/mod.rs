mod context;
mod gdt;
mod idt;

pub mod asm;

#[cfg(target_os = "none")]
mod trap;

#[cfg(feature = "uspace")]
mod syscall;

#[cfg(feature = "uspace")]
pub mod uspace;

pub use self::context::{ExtendedState, FxsaveArea, TaskContext, TrapFrame};
pub use self::gdt::GdtStruct;
pub use self::idt::IdtStruct;
pub use x86_64::structures::tss::TaskStateSegment;

/// Initializes CPU states on the current CPU.
///
/// In detail, it initializes the GDT, IDT on x86_64 platforms. If the `uspace`
/// feature is enabled, it also initializes relevant model-specific registers
/// to enable the `syscall` instruction.
pub fn cpu_init() {
    self::gdt::init_gdt();
    self::idt::init_idt();
    #[cfg(feature = "uspace")]
    self::syscall::init_syscall();
}
