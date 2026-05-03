//! Registers example

#![no_std]
#![no_main]

use aarch32_rt::entry;
use semihosting::println;
use versatileab as _;

/// The entry-point to the Rust application.
///
/// It is called by the start-up.
#[entry]
fn main() -> ! {
    versatileab::init();
    chip_info();
    test_changing_sctlr();
    #[cfg(arm_architecture = "v7-r")]
    mpu_pmsa_v7();
    versatileab::exit(0);
}

fn chip_info() {
    println!("{:x?}", aarch32_cpu::register::Midr::read());
    println!("{:x?}", aarch32_cpu::register::Cpsr::read());

    #[cfg(any(arm_architecture = "v5te", arm_architecture = "v6"))]
    {
        let ctr = aarch32_cpu::register::Ctr::read();
        println!("{:x?}", ctr);
        println!("Cache type: {:?}", ctr.ctype());
        if ctr.s() {
            println!("Cache kind: split");
            println!(
                "ICache    : size={} bytes, assoc={:?}, line_length={} bytes",
                ctr.isize().size_bytes(),
                ctr.isize().associativity(),
                ctr.isize().cache_line_length_bytes()
            );
            println!(
                "DCache    : size={} bytes, assoc={:?}, line_length={} bytes",
                ctr.dsize().size_bytes(),
                ctr.dsize().associativity(),
                ctr.dsize().cache_line_length_bytes()
            );
        } else {
            println!("Cache kind: unified");
            println!(
                "Cache    : size={} bytes, assoc={:?}, line_length={} bytes",
                ctr.isize().size_bytes(),
                ctr.isize().associativity(),
                ctr.isize().cache_line_length_bytes()
            );
        }
    }
    #[cfg(any(arm_architecture = "v7-r", arm_architecture = "v7-a"))]
    {
        let ctr = aarch32_cpu::register::Ctr::read();
        println!("{:08x?}", ctr);
    }

    #[cfg(not(any(arm_architecture = "v4t", arm_architecture = "v5te")))]
    println!("{:x?}", aarch32_cpu::register::Mpidr::read());
}

#[cfg(arm_architecture = "v7-r")]
fn mpu_pmsa_v7() {
    use aarch32_cpu::{
        pmsav7::{AccessPerms, CachePolicy, Config, MemAttr, Mpu, Region, RegionSize},
        register::Mpuir,
    };

    // How many regions?
    let mpuir = Mpuir::read();
    println!("PMSA-v7 MPUIR: {:?}", mpuir);

    // Make an MPU driver
    let mut mpu = unsafe { Mpu::new() };

    // Look at the existing config
    for idx in 0..mpu.num_iregions() {
        if let Some(region) = mpu.get_iregion(idx) {
            println!("IRegion {}: {:?}", idx, region);
        }
    }
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            println!("DRegion {}: {:?}", idx, region);
        }
    }

    // Load a config (but don't enable it)
    mpu.configure(&Config {
        background_config: true,
        dregions: &[Region {
            base: 0x2000_0000 as *mut u8,
            size: RegionSize::_16M,
            subregion_mask: 0x00,
            enabled: true,
            no_exec: false,
            mem_attr: MemAttr::Cacheable {
                inner: CachePolicy::WriteThroughNoWriteAlloc,
                outer: CachePolicy::NonCacheable,
                shareable: true,
            },
            access_perms: AccessPerms::ReadWrite,
        }],
        iregions: &[],
    })
    .unwrap();

    // Look at the new config
    for idx in 0..mpu.num_dregions() {
        if let Some(region) = mpu.get_dregion(idx) {
            println!("DRegion {}: {:?}", idx, region);
        }
    }
}

fn test_changing_sctlr() {
    println!(
        "{:?} before setting C, I and Z",
        aarch32_cpu::register::Sctlr::read()
    );
    aarch32_cpu::register::Sctlr::modify(|w| {
        w.set_c(true);
        w.set_i(true);
        w.set_z(true);
    });
    println!("{:?} after", aarch32_cpu::register::Sctlr::read());
}
