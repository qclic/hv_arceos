//! Platform-specific operations.

cfg_if::cfg_if! {
    if #[cfg(target_arch = "aarch64")]{
        mod aarch64_common;
    }
}

mod aarch64_qemu_virt;
pub use self::aarch64_qemu_virt::*;
