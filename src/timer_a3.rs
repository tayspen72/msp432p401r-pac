#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - TimerAx Control Register"]
    pub tax_ctl: crate::Reg<tax_ctl::TAXCTL_SPEC>,
    #[doc = "0x02..0x0c - Timer_A Capture/Compare Control Register"]
    pub tax_cctl: [crate::Reg<tax_cctl::TAXCCTL_SPEC>; 5],
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - TimerA register"]
    pub tax_r: crate::Reg<tax_r::TAXR_SPEC>,
    #[doc = "0x12..0x1c - Timer_A Capture/Compare Register"]
    pub tax_ccr: [crate::Reg<tax_ccr::TAXCCR_SPEC>; 5],
    _reserved4: [u8; 0x04],
    #[doc = "0x20 - TimerAx Expansion 0 Register"]
    pub tax_ex0: crate::Reg<tax_ex0::TAXEX0_SPEC>,
    _reserved5: [u8; 0x0c],
    #[doc = "0x2e - TimerAx Interrupt Vector Register"]
    pub tax_iv: crate::Reg<tax_iv::TAXIV_SPEC>,
}
#[doc = "TAxCTL register accessor: an alias for `Reg<TAXCTL_SPEC>`"]
pub type TAXCTL = crate::Reg<tax_ctl::TAXCTL_SPEC>;
#[doc = "TimerAx Control Register"]
pub mod tax_ctl;
#[doc = "TAxCCTL register accessor: an alias for `Reg<TAXCCTL_SPEC>`"]
pub type TAXCCTL = crate::Reg<tax_cctl::TAXCCTL_SPEC>;
#[doc = "Timer_A Capture/Compare Control Register"]
pub mod tax_cctl;
#[doc = "TAxR register accessor: an alias for `Reg<TAXR_SPEC>`"]
pub type TAXR = crate::Reg<tax_r::TAXR_SPEC>;
#[doc = "TimerA register"]
pub mod tax_r;
#[doc = "TAxCCR register accessor: an alias for `Reg<TAXCCR_SPEC>`"]
pub type TAXCCR = crate::Reg<tax_ccr::TAXCCR_SPEC>;
#[doc = "Timer_A Capture/Compare Register"]
pub mod tax_ccr;
#[doc = "TAxEX0 register accessor: an alias for `Reg<TAXEX0_SPEC>`"]
pub type TAXEX0 = crate::Reg<tax_ex0::TAXEX0_SPEC>;
#[doc = "TimerAx Expansion 0 Register"]
pub mod tax_ex0;
#[doc = "TAxIV register accessor: an alias for `Reg<TAXIV_SPEC>`"]
pub type TAXIV = crate::Reg<tax_iv::TAXIV_SPEC>;
#[doc = "TimerAx Interrupt Vector Register"]
pub mod tax_iv;
