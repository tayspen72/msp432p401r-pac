#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control 0 Register"]
    pub pcmctl0: crate::Reg<pcmctl0::PCMCTL0_SPEC>,
    #[doc = "0x04 - Control 1 Register"]
    pub pcmctl1: crate::Reg<pcmctl1::PCMCTL1_SPEC>,
    #[doc = "0x08 - Interrupt Enable Register"]
    pub pcmie: crate::Reg<pcmie::PCMIE_SPEC>,
    #[doc = "0x0c - Interrupt Flag Register"]
    pub pcmifg: crate::Reg<pcmifg::PCMIFG_SPEC>,
    #[doc = "0x10 - Clear Interrupt Flag Register"]
    pub pcmclrifg: crate::Reg<pcmclrifg::PCMCLRIFG_SPEC>,
}
#[doc = "PCMCTL0 register accessor: an alias for `Reg<PCMCTL0_SPEC>`"]
pub type PCMCTL0 = crate::Reg<pcmctl0::PCMCTL0_SPEC>;
#[doc = "Control 0 Register"]
pub mod pcmctl0;
#[doc = "PCMCTL1 register accessor: an alias for `Reg<PCMCTL1_SPEC>`"]
pub type PCMCTL1 = crate::Reg<pcmctl1::PCMCTL1_SPEC>;
#[doc = "Control 1 Register"]
pub mod pcmctl1;
#[doc = "PCMIE register accessor: an alias for `Reg<PCMIE_SPEC>`"]
pub type PCMIE = crate::Reg<pcmie::PCMIE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod pcmie;
#[doc = "PCMIFG register accessor: an alias for `Reg<PCMIFG_SPEC>`"]
pub type PCMIFG = crate::Reg<pcmifg::PCMIFG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod pcmifg;
#[doc = "PCMCLRIFG register accessor: an alias for `Reg<PCMCLRIFG_SPEC>`"]
pub type PCMCLRIFG = crate::Reg<pcmclrifg::PCMCLRIFG_SPEC>;
#[doc = "Clear Interrupt Flag Register"]
pub mod pcmclrifg;
