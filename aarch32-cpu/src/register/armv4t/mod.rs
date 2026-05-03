//! Registers available on Armv4T

pub mod branch_target_cache;
pub mod cpsr;
pub mod dacr;
pub mod dcache;
pub mod drain_write_buffer;
pub mod far;
pub mod fpfb;
pub mod fsr;
pub mod icache;
pub mod idcache;
pub mod midr;
pub mod sctlr;
pub mod tlb;
pub mod ttbr0;

#[doc(inline)]
pub use branch_target_cache::{FlushBranchTargetCache, FlushBranchTargetCacheEntry};
#[doc(inline)]
pub use cpsr::Cpsr;
#[doc(inline)]
pub use dacr::Dacr;
#[doc(inline)]
pub use dcache::{
    CleanDCache, CleanDCacheEntry, CleanFlushDCache, CleanFlushDCacheEntry, FlushDCache,
    FlushDCacheSingleEntry,
};
#[doc(inline)]
pub use drain_write_buffer::DrainWriteBuffer;
#[doc(inline)]
pub use far::Far;
#[doc(inline)]
pub use fpfb::FlushPrefetchBuffer;
#[doc(inline)]
pub use fsr::Fsr;
#[doc(inline)]
pub use icache::{FlushICache, FlushICacheSingleEntry};
#[doc(inline)]
pub use idcache::{
    CleanFlushIDCache, CleanFlushIDCacheEntry, CleanIDCache, CleanIDCacheEntry, FlushIDCache,
    FlushIDCacheSingleEntry,
};
#[doc(inline)]
pub use midr::Midr;
#[doc(inline)]
pub use sctlr::Sctlr;
#[doc(inline)]
pub use tlb::{
    FlushDTLB, FlushDTLBSingleEntry, FlushIDTLB, FlushIDTLBSingleEntry, FlushITLB,
    FlushITLBSingleEntry,
};
#[doc(inline)]
pub use ttbr0::Ttbr0;
