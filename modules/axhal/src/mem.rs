//! Physical memory management.

use core::fmt;
use somehal::mem::region::{AccessFlags, MemRegionKind};

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
    memory_addr::PhysAddr::from_usize(paddr.raw())
}

/// Converts a physical address to a virtual address.
#[inline]
pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    let vaddr = somehal::mem::phys_to_virt(somehal::mem::PhysAddr::new(paddr.as_usize()));
    memory_addr::VirtAddr::from_usize(vaddr.raw())
}

/// Returns an iterator over all physical memory regions.
pub fn memory_regions() -> impl Iterator<Item = MemRegion> {
    somehal::mem::memory_regions().map(|reg| {
        MemRegion {
            flags: map_flags(reg.clone()),
            paddr: memory_addr::PhysAddr::from(reg.phys_start.raw()),
            size: reg.size,
            name: reg.name,
        }
    })
}

pub fn map_flags(kind: somehal::mem::MemRegion) -> MemRegionFlags {
    let mut flags = MemRegionFlags::empty();
    if kind.config.access.contains(AccessFlags::Read) {
        flags |= MemRegionFlags::READ;
    }
    if kind.config.access.contains(AccessFlags::Write) {
        flags |= MemRegionFlags::WRITE;
    }
    if kind.config.access.contains(AccessFlags::Execute) {
        flags |= MemRegionFlags::EXECUTE;
    }
    match kind.kind {
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