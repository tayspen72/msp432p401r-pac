#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Comparator Control Register 0"]
    pub cex_ctl0: crate::Reg<cex_ctl0::CEXCTL0_SPEC>,
    #[doc = "0x02 - Comparator Control Register 1"]
    pub cex_ctl1: crate::Reg<cex_ctl1::CEXCTL1_SPEC>,
    #[doc = "0x04 - Comparator Control Register 2"]
    pub cex_ctl2: crate::Reg<cex_ctl2::CEXCTL2_SPEC>,
    #[doc = "0x06 - Comparator Control Register 3"]
    pub cex_ctl3: crate::Reg<cex_ctl3::CEXCTL3_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x0c - Comparator Interrupt Control Register"]
    pub cex_int: crate::Reg<cex_int::CEXINT_SPEC>,
    #[doc = "0x0e - Comparator Interrupt Vector Word Register"]
    pub cex_iv: crate::Reg<cex_iv::CEXIV_SPEC>,
}
#[doc = "CExCTL0 register accessor: an alias for `Reg<CEXCTL0_SPEC>`"]
pub type CEXCTL0 = crate::Reg<cex_ctl0::CEXCTL0_SPEC>;
#[doc = "Comparator Control Register 0"]
pub mod cex_ctl0;
#[doc = "CExCTL1 register accessor: an alias for `Reg<CEXCTL1_SPEC>`"]
pub type CEXCTL1 = crate::Reg<cex_ctl1::CEXCTL1_SPEC>;
#[doc = "Comparator Control Register 1"]
pub mod cex_ctl1;
#[doc = "CExCTL2 register accessor: an alias for `Reg<CEXCTL2_SPEC>`"]
pub type CEXCTL2 = crate::Reg<cex_ctl2::CEXCTL2_SPEC>;
#[doc = "Comparator Control Register 2"]
pub mod cex_ctl2;
#[doc = "CExCTL3 register accessor: an alias for `Reg<CEXCTL3_SPEC>`"]
pub type CEXCTL3 = crate::Reg<cex_ctl3::CEXCTL3_SPEC>;
#[doc = "Comparator Control Register 3"]
pub mod cex_ctl3;
#[doc = "CExINT register accessor: an alias for `Reg<CEXINT_SPEC>`"]
pub type CEXINT = crate::Reg<cex_int::CEXINT_SPEC>;
#[doc = "Comparator Interrupt Control Register"]
pub mod cex_int;
#[doc = "CExIV register accessor: an alias for `Reg<CEXIV_SPEC>`"]
pub type CEXIV = crate::Reg<cex_iv::CEXIV_SPEC>;
#[doc = "Comparator Interrupt Vector Word Register"]
pub mod cex_iv;
