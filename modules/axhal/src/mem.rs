//! Physical memory management.

use core::fmt;
use somehal::mem::region::MemRegionKind;

#[doc(no_inline)]
pub use memory_addr::{MemoryAddr, PAGE_SIZE_4K, PhysAddr, VirtAddr};

bitflags::bitflags! {
    /// The flags of a physical memory region.
    pub struct MemRegionFlags: usize {
        /// Readable.
        const READ          = 1 << 0;
        /// Writable.
        const WRITE         = 1 << 1;
        /// Executable.
        const EXECUTE       = 1 << 2;
        /// Device memory. (e.g., MMIO regions)
        const DEVICE        = 1 << 4;
        /// Uncachable memory. (e.g., framebuffer)
        const UNCACHED      = 1 << 5;
        /// Reserved memory, do not use for allocation.
        const RESERVED      = 1 << 6;
        /// Free memory for allocation.
        const FREE          = 1 << 7;
    }
}

impl fmt::Debug for MemRegionFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

/// A physical memory region.
#[derive(Debug)]
pub struct MemRegion {
    /// The start physical address of the region.
    pub paddr: PhysAddr,
    /// The size in bytes of the region.
    pub size: usize,
    /// The region flags, see [`MemRegionFlags`].
    pub flags: MemRegionFlags,
    /// The region name, used for identification.
    pub name: &'static str,
}

/// Converts a virtual address to a physical address.
#[inline]
pub fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    let paddr = somehal::mem::virt_to_phys(somehal::mem::VirtAddr::new(vaddr.as_usize()));
    memory_addr::PhysAddr::new(paddr.as_usize())
}

/// Converts a physical address to a virtual address.
#[inline]
pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    let vaddr = somehal::mem::phys_to_virt(somehal::mem::PhysAddr::new(paddr.as_usize()));
    memory_addr::VirtAddr::new(vaddr.as_usize())
}

/// Returns an iterator over all physical memory regions.
pub fn memory_regions() -> impl Iterator<Item = MemRegion> {
    [].into_iter()
    // somehal::mem::memory_regions().map(|reg| {
    //     MemRegion {
    //         paddr: memory_addr::PhysAddr::from(reg.phys_start.as_usize()),
    //         size: reg.size,
    //         flags: map_flags(reg.kind),
    //         name: reg.name,
    //     }
    // })
}

pub fn map_flags(kind: MemRegionKind) -> MemRegionFlags {
    let mut flags = MemRegionFlags::empty();

    match kind {
        MemRegionKind::Code => {
            flags |= MemRegionFlags::RESERVED;
        }
        MemRegionKind::Stack => {
            flags |= MemRegionFlags::RESERVED;
        }
        MemRegionKind::PerCpu => {
            flags |= MemRegionFlags::RESERVED;
        }
        MemRegionKind::Device => {
            flags |= MemRegionFlags::DEVICE;
        }
        MemRegionKind::Memory => {
            flags |= MemRegionFlags::FREE;
        }
    }
    flags
}

pub fn default_free_regions() -> impl Iterator<Item = MemRegion> {
    [].into_iter()
}

pub fn default_mmio_regions() -> impl Iterator<Item = MemRegion> {
    [].into_iter()
}

// /// Returns the memory regions of the kernel image (code and data sections).
// fn kernel_image_regions() -> impl Iterator<Item = MemRegion> {
//     [
//         MemRegion {
//             paddr: virt_to_phys((_stext as usize).into()),
//             size: _etext as usize - _stext as usize,
//             flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::EXECUTE,
//             name: ".text",
//         },
//         MemRegion {
//             paddr: virt_to_phys((_srodata as usize).into()),
//             size: _erodata as usize - _srodata as usize,
//             flags: MemRegionFlags::RESERVED | MemRegionFlags::READ,
//             name: ".rodata",
//         },
//         MemRegion {
//             paddr: virt_to_phys((_sdata as usize).into()),
//             size: _edata as usize - _sdata as usize,
//             flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
//             name: ".data .tdata .tbss .percpu",
//         },
//         MemRegion {
//             paddr: virt_to_phys((boot_stack as usize).into()),
//             size: boot_stack_top as usize - boot_stack as usize,
//             flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
//             name: "boot stack",
//         },
//         MemRegion {
//             paddr: virt_to_phys((_sbss as usize).into()),
//             size: _ebss as usize - _sbss as usize,
//             flags: MemRegionFlags::RESERVED | MemRegionFlags::READ | MemRegionFlags::WRITE,
//             name: ".bss",
//         },
//     ]
//     .into_iter()
// }

// /// Returns the default MMIO memory regions (from [`axconfig::MMIO_REGIONS`]).
// #[allow(dead_code)]
// pub(crate) fn default_mmio_regions() -> impl Iterator<Item = MemRegion> {
//     axconfig::devices::MMIO_REGIONS.iter().map(|reg| MemRegion {
//         paddr: reg.0.into(),
//         size: reg.1,
//         flags: MemRegionFlags::RESERVED
//             | MemRegionFlags::DEVICE
//             | MemRegionFlags::READ
//             | MemRegionFlags::WRITE,
//         name: "mmio",
//     })
// }

// /// Returns the default free memory regions (kernel image end to physical memory end).
// #[allow(dead_code)]
// pub(crate) fn default_free_regions() -> impl Iterator<Item = MemRegion> {
//     let start = virt_to_phys((_ekernel as usize).into()).align_up_4k();
//     let end = pa!(PHYS_MEMORY_BASE + PHYS_MEMORY_SIZE).align_down_4k();
//     core::iter::once(MemRegion {
//         paddr: start,
//         size: end.as_usize() - start.as_usize(),
//         flags: MemRegionFlags::FREE | MemRegionFlags::READ | MemRegionFlags::WRITE,
//         name: "free memory",
//     })
// }

// /// Fills the `.bss` section with zeros.
// #[allow(dead_code)]
// pub(crate) fn clear_bss() {
//     unsafe {
//         core::slice::from_raw_parts_mut(_sbss as usize as *mut u8, _ebss as usize - _sbss as usize)
//             .fill(0);
//     }
// }

// unsafe extern "C" {
//     fn _stext();
//     fn _etext();
//     fn _srodata();
//     fn _erodata();
//     fn _sdata();
//     fn _edata();
//     fn _sbss();
//     fn _ebss();
//     fn _ekernel();
//     fn boot_stack();
//     fn boot_stack_top();
// }
