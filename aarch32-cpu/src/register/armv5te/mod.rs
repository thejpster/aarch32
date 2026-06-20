//! Registers available on Armv5TE

// These modules have extra types in we want to export

pub mod access_permissions;
pub mod bufferability;
pub mod cacheability;
pub mod cpsr;
pub mod ctr;
pub mod dfsr;
pub mod ifsr;
pub mod ipac0;
pub mod ipac1;
pub mod ipac2;
pub mod ipac3;
pub mod ipac4;
pub mod ipac5;
pub mod ipac6;
pub mod ipac7;
pub mod midr;
pub mod pac;
pub mod pac0;
pub mod pac1;
pub mod pac2;
pub mod pac3;
pub mod pac4;
pub mod pac5;
pub mod pac6;
pub mod pac7;
pub mod sctlr;

#[doc(inline)]
pub use access_permissions::{AccessPermission, AccessPermissions};
#[doc(inline)]
pub use bufferability::Bufferability;
#[doc(inline)]
pub use cacheability::Cacheability;
#[doc(inline)]
pub use cpsr::Cpsr;
#[doc(inline)]
pub use ctr::Ctr;
#[doc(inline)]
pub use dfsr::Dfsr;
#[doc(inline)]
pub use ifsr::Ifsr;
#[doc(inline)]
pub use ipac0::Ipac0;
#[doc(inline)]
pub use ipac1::Ipac1;
#[doc(inline)]
pub use ipac2::Ipac2;
#[doc(inline)]
pub use ipac3::Ipac3;
#[doc(inline)]
pub use ipac4::Ipac4;
#[doc(inline)]
pub use ipac5::Ipac5;
#[doc(inline)]
pub use ipac6::Ipac6;
#[doc(inline)]
pub use ipac7::Ipac7;
#[doc(inline)]
pub use midr::Midr;
#[doc(inline)]
pub use pac::Pac;
#[doc(inline)]
pub use pac0::Pac0;
#[doc(inline)]
pub use pac1::Pac1;
#[doc(inline)]
pub use pac2::Pac2;
#[doc(inline)]
pub use pac3::Pac3;
#[doc(inline)]
pub use pac4::Pac4;
#[doc(inline)]
pub use pac5::Pac5;
#[doc(inline)]
pub use pac6::Pac6;
#[doc(inline)]
pub use pac7::Pac7;
#[doc(inline)]
pub use sctlr::Sctlr;

/// CP15 Register 7: WaitForInterrupt
pub struct WaitForInterrupt();

impl super::SysReg for WaitForInterrupt {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 0;
    const OP2: u32 = 4;
}

/// CP15 Register 7: InvalidateEntireICache
pub struct InvalidateEntireICache();

impl super::SysReg for InvalidateEntireICache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 0;
}

/// CP15 Register 7: InvalidateICacheLineByVa
pub struct InvalidateICacheLineByVa();

impl super::SysReg for InvalidateICacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 1;
}

/// CP15 Register 7: InvalidateICacheLineBySetIndex
pub struct InvalidateICacheLineBySetIndex();

impl super::SysReg for InvalidateICacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 2;
}

/// CP15 Register 7: FlushPrefetchBuffer
pub struct FlushPrefetchBuffer();

impl super::SysReg for FlushPrefetchBuffer {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 4;
}

/// CP15 Register 7: FlushBranchTargetCache
pub struct FlushBranchTargetCache();

impl super::SysReg for FlushBranchTargetCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 6;
}

/// CP15 Register 7: FlushBranchTargetCacheEntry
pub struct FlushBranchTargetCacheEntry();

impl super::SysReg for FlushBranchTargetCacheEntry {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 5;
    const OP2: u32 = 7;
}

/// CP15 Register 7: InvalidateEntireDCache
pub struct InvalidateEntireDCache();

impl super::SysReg for InvalidateEntireDCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 0;
}

/// CP15 Register 7: InvalidateDCacheLineByVa
pub struct InvalidateDCacheLineByVa();

impl super::SysReg for InvalidateDCacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 1;
}

/// CP15 Register 7: InvalidateDCacheLineBySetIndex
pub struct InvalidateDCacheLineBySetIndex();

impl super::SysReg for InvalidateDCacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 6;
    const OP2: u32 = 2;
}

/// CP15 Register 7: InvalidateEntireUnifiedCache
pub struct InvalidateEntireUnifiedCache();

impl super::SysReg for InvalidateEntireUnifiedCache {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 0;
}

/// CP15 Register 7: InvalidateUnifiedCacheLineByVa
pub struct InvalidateUnifiedCacheLineByVa();

impl super::SysReg for InvalidateUnifiedCacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 1;
}

/// CP15 Register 7: InvalidateUnifiedCacheLineBySetIndex
pub struct InvalidateUnifiedCacheLineBySetIndex();

impl super::SysReg for InvalidateUnifiedCacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 7;
    const OP2: u32 = 2;
}

/*
common::dccsw
common::dccmvac
common::dccmvau
armv4t::dwb

| CRm | opcode2 | Function                            | Data | Location         |
|-----|---------|-------------------------------------|------|------------------|
| c0  | 4       | Wait for interrupt                  | SBZ  | Here             |
| c5  | 0       | Invalidate entire instruction cache | SBZ  | common::iciallu  |
| c5  | 1       | Invalidate instruction cache line   | VA   | ??               |
| c5  | 2       | Invalidate instruction cache line   | S/W  | ??               |
| c5  | 4       | Flush prefetch buffer               | SBZ  | armv4t::fpfb     |

*/

/// CP15 Register 7: CleanUnifiedCacheLineByVa
pub struct CleanUnifiedCacheLineByVa();

impl super::SysReg for CleanUnifiedCacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 1;
}

/// CP15 Register 7: CleanUnifiedCacheLineBySetIndex
pub struct CleanUnifiedCacheLineBySetIndex();

impl super::SysReg for CleanUnifiedCacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 11;
    const OP2: u32 = 2;
}

/// CP15 Register 7: PrefetchICacheLineByVa
pub struct PrefetchICacheLineByVa();

impl super::SysReg for PrefetchICacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 13;
    const OP2: u32 = 1;
}

/// CP15 Register 7: CleanAndInvalidateDCacheLineByVa
pub struct CleanAndInvalidateDCacheLineByVa();

impl super::SysReg for CleanAndInvalidateDCacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 1;
}

/// CP15 Register 7: CleanAndInvalidateDCacheLineBySetIndex
pub struct CleanAndInvalidateDCacheLineBySetIndex();

impl super::SysReg for CleanAndInvalidateDCacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 14;
    const OP2: u32 = 2;
}

/// CP15 Register 7: CleanAndInvalidateUnifiedCacheLineByVa
pub struct CleanAndInvalidateUnifiedCacheLineByVa();

impl super::SysReg for CleanAndInvalidateUnifiedCacheLineByVa {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 1;
}

/// CP15 Register 7: CleanAndInvalidateUnifiedCacheLineBySetIndex
pub struct CleanAndInvalidateUnifiedCacheLineBySetIndex();

impl super::SysReg for CleanAndInvalidateUnifiedCacheLineBySetIndex {
    const CP: u32 = 15;
    const CRN: u32 = 7;
    const OP1: u32 = 0;
    const CRM: u32 = 15;
    const OP2: u32 = 2;
}

// CP15 7 and 9 are for MMU
// CP15 13 is for Fast Context Switch Extension
