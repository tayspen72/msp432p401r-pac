#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Ax Control Word Register 0"]
    pub ucax_ctlw0: crate::Reg<ucax_ctlw0::UCAXCTLW0_SPEC>,
    #[doc = "0x02 - eUSCI_Ax Control Word Register 1"]
    pub ucax_ctlw1: crate::Reg<ucax_ctlw1::UCAXCTLW1_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - eUSCI_Ax Baud Rate Control Word Register"]
    pub ucax_brw: crate::Reg<ucax_brw::UCAXBRW_SPEC>,
    #[doc = "0x08 - eUSCI_Ax Modulation Control Word Register"]
    pub ucax_mctlw: crate::Reg<ucax_mctlw::UCAXMCTLW_SPEC>,
    #[doc = "0x0a - eUSCI_Ax Status Register"]
    pub ucax_statw: crate::Reg<ucax_statw::UCAXSTATW_SPEC>,
    #[doc = "0x0c - eUSCI_Ax Receive Buffer Register"]
    pub ucax_rxbuf: crate::Reg<ucax_rxbuf::UCAXRXBUF_SPEC>,
    #[doc = "0x0e - eUSCI_Ax Transmit Buffer Register"]
    pub ucax_txbuf: crate::Reg<ucax_txbuf::UCAXTXBUF_SPEC>,
    #[doc = "0x10 - eUSCI_Ax Auto Baud Rate Control Register"]
    pub ucax_abctl: crate::Reg<ucax_abctl::UCAXABCTL_SPEC>,
    #[doc = "0x12 - eUSCI_Ax IrDA Control Word Register"]
    pub ucax_irctl: crate::Reg<ucax_irctl::UCAXIRCTL_SPEC>,
    _reserved9: [u8; 0x06],
    #[doc = "0x1a - eUSCI_Ax Interrupt Enable Register"]
    pub ucax_ie: crate::Reg<ucax_ie::UCAXIE_SPEC>,
    #[doc = "0x1c - eUSCI_Ax Interrupt Flag Register"]
    pub ucax_ifg: crate::Reg<ucax_ifg::UCAXIFG_SPEC>,
    #[doc = "0x1e - eUSCI_Ax Interrupt Vector Register"]
    pub ucax_iv: crate::Reg<ucax_iv::UCAXIV_SPEC>,
}
#[doc = "UCAxCTLW0 register accessor: an alias for `Reg<UCAXCTLW0_SPEC>`"]
pub type UCAXCTLW0 = crate::Reg<ucax_ctlw0::UCAXCTLW0_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 0"]
pub mod ucax_ctlw0;
#[doc = "UCAxCTLW1 register accessor: an alias for `Reg<UCAXCTLW1_SPEC>`"]
pub type UCAXCTLW1 = crate::Reg<ucax_ctlw1::UCAXCTLW1_SPEC>;
#[doc = "eUSCI_Ax Control Word Register 1"]
pub mod ucax_ctlw1;
#[doc = "UCAxBRW register accessor: an alias for `Reg<UCAXBRW_SPEC>`"]
pub type UCAXBRW = crate::Reg<ucax_brw::UCAXBRW_SPEC>;
#[doc = "eUSCI_Ax Baud Rate Control Word Register"]
pub mod ucax_brw;
#[doc = "UCAxMCTLW register accessor: an alias for `Reg<UCAXMCTLW_SPEC>`"]
pub type UCAXMCTLW = crate::Reg<ucax_mctlw::UCAXMCTLW_SPEC>;
#[doc = "eUSCI_Ax Modulation Control Word Register"]
pub mod ucax_mctlw;
#[doc = "UCAxSTATW register accessor: an alias for `Reg<UCAXSTATW_SPEC>`"]
pub type UCAXSTATW = crate::Reg<ucax_statw::UCAXSTATW_SPEC>;
#[doc = "eUSCI_Ax Status Register"]
pub mod ucax_statw;
#[doc = "UCAxRXBUF register accessor: an alias for `Reg<UCAXRXBUF_SPEC>`"]
pub type UCAXRXBUF = crate::Reg<ucax_rxbuf::UCAXRXBUF_SPEC>;
#[doc = "eUSCI_Ax Receive Buffer Register"]
pub mod ucax_rxbuf;
#[doc = "UCAxTXBUF register accessor: an alias for `Reg<UCAXTXBUF_SPEC>`"]
pub type UCAXTXBUF = crate::Reg<ucax_txbuf::UCAXTXBUF_SPEC>;
#[doc = "eUSCI_Ax Transmit Buffer Register"]
pub mod ucax_txbuf;
#[doc = "UCAxABCTL register accessor: an alias for `Reg<UCAXABCTL_SPEC>`"]
pub type UCAXABCTL = crate::Reg<ucax_abctl::UCAXABCTL_SPEC>;
#[doc = "eUSCI_Ax Auto Baud Rate Control Register"]
pub mod ucax_abctl;
#[doc = "UCAxIRCTL register accessor: an alias for `Reg<UCAXIRCTL_SPEC>`"]
pub type UCAXIRCTL = crate::Reg<ucax_irctl::UCAXIRCTL_SPEC>;
#[doc = "eUSCI_Ax IrDA Control Word Register"]
pub mod ucax_irctl;
#[doc = "UCAxIE register accessor: an alias for `Reg<UCAXIE_SPEC>`"]
pub type UCAXIE = crate::Reg<ucax_ie::UCAXIE_SPEC>;
#[doc = "eUSCI_Ax Interrupt Enable Register"]
pub mod ucax_ie;
#[doc = "UCAxIFG register accessor: an alias for `Reg<UCAXIFG_SPEC>`"]
pub type UCAXIFG = crate::Reg<ucax_ifg::UCAXIFG_SPEC>;
#[doc = "eUSCI_Ax Interrupt Flag Register"]
pub mod ucax_ifg;
#[doc = "UCAxIV register accessor: an alias for `Reg<UCAXIV_SPEC>`"]
pub type UCAXIV = crate::Reg<ucax_iv::UCAXIV_SPEC>;
#[doc = "eUSCI_Ax Interrupt Vector Register"]
pub mod ucax_iv;
