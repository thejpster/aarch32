//! Registers available on Armv8-R

pub mod ctr;
pub mod dfsr;
pub mod hmpuir;
pub mod hprbar;
pub mod hprbar0;
pub mod hprbar1;
pub mod hprbar10;
pub mod hprbar11;
pub mod hprbar12;
pub mod hprbar13;
pub mod hprbar14;
pub mod hprbar15;
pub mod hprbar2;
pub mod hprbar3;
pub mod hprbar4;
pub mod hprbar5;
pub mod hprbar6;
pub mod hprbar7;
pub mod hprbar8;
pub mod hprbar9;
pub mod hprenr;
pub mod hprlar;
pub mod hprlar0;
pub mod hprlar1;
pub mod hprlar10;
pub mod hprlar11;
pub mod hprlar12;
pub mod hprlar13;
pub mod hprlar14;
pub mod hprlar15;
pub mod hprlar2;
pub mod hprlar3;
pub mod hprlar4;
pub mod hprlar5;
pub mod hprlar6;
pub mod hprlar7;
pub mod hprlar8;
pub mod hprlar9;
pub mod hprselr;
pub mod ifsr;
pub mod prbar;
pub mod prbar0;
pub mod prbar1;
pub mod prbar10;
pub mod prbar11;
pub mod prbar12;
pub mod prbar13;
pub mod prbar14;
pub mod prbar15;
pub mod prbar2;
pub mod prbar3;
pub mod prbar4;
pub mod prbar5;
pub mod prbar6;
pub mod prbar7;
pub mod prbar8;
pub mod prbar9;
pub mod prlar;
pub mod prlar0;
pub mod prlar1;
pub mod prlar10;
pub mod prlar11;
pub mod prlar12;
pub mod prlar13;
pub mod prlar14;
pub mod prlar15;
pub mod prlar2;
pub mod prlar3;
pub mod prlar4;
pub mod prlar5;
pub mod prlar6;
pub mod prlar7;
pub mod prlar8;
pub mod prlar9;
pub mod prselr;

#[doc(inline)]
pub use ctr::Ctr;
#[doc(inline)]
pub use dfsr::Dfsr;
#[doc(inline)]
pub use hmpuir::Hmpuir;
#[doc(inline)]
pub use hprbar::Hprbar;
#[doc(inline)]
pub use hprbar0::Hprbar0;
#[doc(inline)]
pub use hprbar1::Hprbar1;
#[doc(inline)]
pub use hprbar10::Hprbar10;
#[doc(inline)]
pub use hprbar11::Hprbar11;
#[doc(inline)]
pub use hprbar12::Hprbar12;
#[doc(inline)]
pub use hprbar13::Hprbar13;
#[doc(inline)]
pub use hprbar14::Hprbar14;
#[doc(inline)]
pub use hprbar15::Hprbar15;
#[doc(inline)]
pub use hprbar2::Hprbar2;
#[doc(inline)]
pub use hprbar3::Hprbar3;
#[doc(inline)]
pub use hprbar4::Hprbar4;
#[doc(inline)]
pub use hprbar5::Hprbar5;
#[doc(inline)]
pub use hprbar6::Hprbar6;
#[doc(inline)]
pub use hprbar7::Hprbar7;
#[doc(inline)]
pub use hprbar8::Hprbar8;
#[doc(inline)]
pub use hprbar9::Hprbar9;
#[doc(inline)]
pub use hprenr::Hprenr;
#[doc(inline)]
pub use hprlar::Hprlar;
#[doc(inline)]
pub use hprlar0::Hprlar0;
#[doc(inline)]
pub use hprlar1::Hprlar1;
#[doc(inline)]
pub use hprlar10::Hprlar10;
#[doc(inline)]
pub use hprlar11::Hprlar11;
#[doc(inline)]
pub use hprlar12::Hprlar12;
#[doc(inline)]
pub use hprlar13::Hprlar13;
#[doc(inline)]
pub use hprlar14::Hprlar14;
#[doc(inline)]
pub use hprlar15::Hprlar15;
#[doc(inline)]
pub use hprlar2::Hprlar2;
#[doc(inline)]
pub use hprlar3::Hprlar3;
#[doc(inline)]
pub use hprlar4::Hprlar4;
#[doc(inline)]
pub use hprlar5::Hprlar5;
#[doc(inline)]
pub use hprlar6::Hprlar6;
#[doc(inline)]
pub use hprlar7::Hprlar7;
#[doc(inline)]
pub use hprlar8::Hprlar8;
#[doc(inline)]
pub use hprlar9::Hprlar9;
#[doc(inline)]
pub use hprselr::Hprselr;
#[doc(inline)]
pub use ifsr::Ifsr;
#[doc(inline)]
pub use prbar::Prbar;
#[doc(inline)]
pub use prbar0::Prbar0;
#[doc(inline)]
pub use prbar1::Prbar1;
#[doc(inline)]
pub use prbar10::Prbar10;
#[doc(inline)]
pub use prbar11::Prbar11;
#[doc(inline)]
pub use prbar12::Prbar12;
#[doc(inline)]
pub use prbar13::Prbar13;
#[doc(inline)]
pub use prbar14::Prbar14;
#[doc(inline)]
pub use prbar15::Prbar15;
#[doc(inline)]
pub use prbar2::Prbar2;
#[doc(inline)]
pub use prbar3::Prbar3;
#[doc(inline)]
pub use prbar4::Prbar4;
#[doc(inline)]
pub use prbar5::Prbar5;
#[doc(inline)]
pub use prbar6::Prbar6;
#[doc(inline)]
pub use prbar7::Prbar7;
#[doc(inline)]
pub use prbar8::Prbar8;
#[doc(inline)]
pub use prbar9::Prbar9;
#[doc(inline)]
pub use prlar::Prlar;
#[doc(inline)]
pub use prlar0::Prlar0;
#[doc(inline)]
pub use prlar1::Prlar1;
#[doc(inline)]
pub use prlar10::Prlar10;
#[doc(inline)]
pub use prlar11::Prlar11;
#[doc(inline)]
pub use prlar12::Prlar12;
#[doc(inline)]
pub use prlar13::Prlar13;
#[doc(inline)]
pub use prlar14::Prlar14;
#[doc(inline)]
pub use prlar15::Prlar15;
#[doc(inline)]
pub use prlar2::Prlar2;
#[doc(inline)]
pub use prlar3::Prlar3;
#[doc(inline)]
pub use prlar4::Prlar4;
#[doc(inline)]
pub use prlar5::Prlar5;
#[doc(inline)]
pub use prlar6::Prlar6;
#[doc(inline)]
pub use prlar7::Prlar7;
#[doc(inline)]
pub use prlar8::Prlar8;
#[doc(inline)]
pub use prlar9::Prlar9;
#[doc(inline)]
pub use prselr::Prselr;

#[doc(inline)]
pub use super::armv6::Midr;

pub use super::generic_timer::*;
pub use super::hyp::*;

// Export all the common registers that apply here (and sub-modules where useful)

pub use super::common::actlr::Actlr;
pub use super::common::actlr2::Actlr2;
pub use super::common::adfsr::Adfsr;
pub use super::common::aidr::Aidr;
pub use super::common::aifsr::Aifsr;
pub use super::common::amair0::Amair0;
pub use super::common::amair1::Amair1;
pub use super::common::bpiall::BpIAll;
pub use super::common::ccsidr::Ccsidr;
pub use super::common::clidr::Clidr;
pub use super::common::contextidr::Contextidr;
pub use super::common::cpacr::Cpacr;
pub use super::common::cpsr::{self, Cpsr};
pub use super::common::csselr::Csselr;
pub use super::common::dacr::{self, Dacr};
pub use super::common::dccimvac::Dccimvac;
pub use super::common::dccisw::Dccisw;
pub use super::common::dccmvac::Dccmvac;
pub use super::common::dccmvau::Dccmvau;
pub use super::common::dccsw::Dccsw;
pub use super::common::dcimvac::Dcimvac;
pub use super::common::dcisw::Dcisw;
pub use super::common::dfar::Dfar;
pub use super::common::dlr::Dlr;
pub use super::common::dracr::Dracr;
pub use super::common::drbar::Drbar;
pub use super::common::drsr::{self, Drsr};
pub use super::common::dspsr::Dspsr;
pub use super::common::fcseidr::Fcseidr;
pub use super::common::icc_pmr::IccPmr;
pub use super::common::iciallu::Iciallu;
pub use super::common::id_afr0::IdAfr0;
pub use super::common::id_dfr0::IdDfr0;
pub use super::common::id_isar0::IdIsar0;
pub use super::common::id_isar1::IdIsar1;
pub use super::common::id_isar2::IdIsar2;
pub use super::common::id_isar3::IdIsar3;
pub use super::common::id_isar4::IdIsar4;
pub use super::common::id_isar5::IdIsar5;
pub use super::common::id_mmfr0::IdMmfr0;
pub use super::common::id_mmfr1::IdMmfr1;
pub use super::common::id_mmfr2::IdMmfr2;
pub use super::common::id_mmfr3::IdMmfr3;
pub use super::common::id_mmfr4::IdMmfr4;
pub use super::common::id_pfr0::IdPfr0;
pub use super::common::id_pfr1::IdPfr1;
pub use super::common::ifar::Ifar;
pub use super::common::imp_atcmregionr::ImpAtcmregionr;
pub use super::common::imp_bpctlr::ImpBpctlr;
pub use super::common::imp_btcmregionr::ImpBtcmregionr;
pub use super::common::imp_buildoptr::ImpBuildoptr;
pub use super::common::imp_bustimeoutr::ImpBustimeoutr;
pub use super::common::imp_cbar::ImpCbar;
pub use super::common::imp_cdbgdcd::ImpCdbgdcd;
pub use super::common::imp_cdbgdci::ImpCdbgdci;
pub use super::common::imp_cdbgdct::ImpCdbgdct;
pub use super::common::imp_cdbgdr0::ImpCdbgdr0;
pub use super::common::imp_cdbgdr1::ImpCdbgdr1;
pub use super::common::imp_cdbgdr2::ImpCdbgdr2;
pub use super::common::imp_cdbgicd::ImpCdbgicd;
pub use super::common::imp_cdbgict::ImpCdbgict;
pub use super::common::imp_csctlr::ImpCsctlr;
pub use super::common::imp_ctcmregionr::ImpCtcmregionr;
pub use super::common::imp_dcerr0::ImpDcerr0;
pub use super::common::imp_dcerr1::ImpDcerr1;
pub use super::common::imp_flasherr0::ImpFlasherr0;
pub use super::common::imp_flasherr1::ImpFlasherr1;
pub use super::common::imp_flashifregionr::ImpFlashifregionr;
pub use super::common::imp_icerr0::ImpIcerr0;
pub use super::common::imp_icerr1::ImpIcerr1;
pub use super::common::imp_intmonr::ImpIntmonr;
pub use super::common::imp_memprotctlr::ImpMemprotctlr;
pub use super::common::imp_periphpregionr::ImpPeriphpregionr;
pub use super::common::imp_pinoptr::ImpPinoptr;
pub use super::common::imp_qosr::ImpQosr;
pub use super::common::imp_slavepctlr::ImpSlavepctlr;
pub use super::common::imp_tcmerr0::ImpTcmerr0;
pub use super::common::imp_tcmerr1::ImpTcmerr1;
pub use super::common::imp_tcmsyndr0::ImpTcmsyndr0;
pub use super::common::imp_tcmsyndr1::ImpTcmsyndr1;
pub use super::common::imp_testr0::ImpTestr0;
pub use super::common::iracr::Iracr;
pub use super::common::irbar::Irbar;
pub use super::common::irsr::Irsr;
pub use super::common::mair0::{Mair, Mair0};
pub use super::common::mair1::Mair1;
pub use super::common::mpidr::Mpidr;
pub use super::common::mpuir::Mpuir;
pub use super::common::nsacr::Nsacr;
pub use super::common::par::Par;
pub use super::common::pmccfiltr::Pmccfiltr;
pub use super::common::pmccntr::Pmccntr;
pub use super::common::pmceid0::Pmceid0;
pub use super::common::pmceid1::Pmceid1;
pub use super::common::pmcntenclr::Pmcntenclr;
pub use super::common::pmcntenset::Pmcntenset;
pub use super::common::pmcr::Pmcr;
pub use super::common::pmevcntr0::Pmevcntr0;
pub use super::common::pmevcntr1::Pmevcntr1;
pub use super::common::pmevcntr2::Pmevcntr2;
pub use super::common::pmevcntr3::Pmevcntr3;
pub use super::common::pmevtyper0::Pmevtyper0;
pub use super::common::pmevtyper1::Pmevtyper1;
pub use super::common::pmevtyper2::Pmevtyper2;
pub use super::common::pmevtyper3::Pmevtyper3;
pub use super::common::pmintenclr::Pmintenclr;
pub use super::common::pmintenset::Pmintenset;
pub use super::common::pmovsr::Pmovsr;
pub use super::common::pmovsset::Pmovsset;
pub use super::common::pmselr::Pmselr;
pub use super::common::pmswinc::Pmswinc;
pub use super::common::pmuserenr::Pmuserenr;
pub use super::common::pmxevcntr::Pmxevcntr;
pub use super::common::pmxevtyper::Pmxevtyper;
pub use super::common::revidr::Revidr;
pub use super::common::rgnr::Rgnr;
pub use super::common::rvbar::Rvbar;
pub use super::common::sctlr::Sctlr;
pub use super::common::tcmtr::Tcmtr;
pub use super::common::tlbiall::TlbIAll;
pub use super::common::tlbtr::Tlbtr;
pub use super::common::tpidrprw::Tpidrprw;
pub use super::common::tpidruro::Tpidruro;
pub use super::common::tpidrurw::Tpidrurw;
pub use super::common::ttbr0::Ttbr0;
pub use super::common::vbar::Vbar;
pub use super::common::vmpidr::Vmpidr;
pub use super::common::vpidr::Vpidr;
pub use super::common::vsctlr::Vsctlr;
