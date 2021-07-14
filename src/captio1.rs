#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0e],
    #[doc = "0x0e - Capacitive Touch IO x Control Register"]
    pub captiox_ctl: crate::Reg<captiox_ctl::CAPTIOXCTL_SPEC>,
}
#[doc = "CAPTIOxCTL register accessor: an alias for `Reg<CAPTIOXCTL_SPEC>`"]
pub type CAPTIOXCTL = crate::Reg<captiox_ctl::CAPTIOXCTL_SPEC>;
#[doc = "Capacitive Touch IO x Control Register"]
pub mod captiox_ctl;
