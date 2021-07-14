#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - AES Accelerator Control Register 0"]
    pub aesactl0: crate::Reg<aesactl0::AESACTL0_SPEC>,
    #[doc = "0x02 - AES Accelerator Control Register 1"]
    pub aesactl1: crate::Reg<aesactl1::AESACTL1_SPEC>,
    #[doc = "0x04 - AES Accelerator Status Register"]
    pub aesastat: crate::Reg<aesastat::AESASTAT_SPEC>,
    #[doc = "0x06 - AES Accelerator Key Register"]
    pub aesakey: crate::Reg<aesakey::AESAKEY_SPEC>,
    #[doc = "0x08 - AES Accelerator Data In Register"]
    pub aesadin: crate::Reg<aesadin::AESADIN_SPEC>,
    #[doc = "0x0a - AES Accelerator Data Out Register"]
    pub aesadout: crate::Reg<aesadout::AESADOUT_SPEC>,
    #[doc = "0x0c - AES Accelerator XORed Data In Register"]
    pub aesaxdin: crate::Reg<aesaxdin::AESAXDIN_SPEC>,
    #[doc = "0x0e - AES Accelerator XORed Data In Register"]
    pub aesaxin: crate::Reg<aesaxin::AESAXIN_SPEC>,
}
#[doc = "AESACTL0 register accessor: an alias for `Reg<AESACTL0_SPEC>`"]
pub type AESACTL0 = crate::Reg<aesactl0::AESACTL0_SPEC>;
#[doc = "AES Accelerator Control Register 0"]
pub mod aesactl0;
#[doc = "AESACTL1 register accessor: an alias for `Reg<AESACTL1_SPEC>`"]
pub type AESACTL1 = crate::Reg<aesactl1::AESACTL1_SPEC>;
#[doc = "AES Accelerator Control Register 1"]
pub mod aesactl1;
#[doc = "AESASTAT register accessor: an alias for `Reg<AESASTAT_SPEC>`"]
pub type AESASTAT = crate::Reg<aesastat::AESASTAT_SPEC>;
#[doc = "AES Accelerator Status Register"]
pub mod aesastat;
#[doc = "AESAKEY register accessor: an alias for `Reg<AESAKEY_SPEC>`"]
pub type AESAKEY = crate::Reg<aesakey::AESAKEY_SPEC>;
#[doc = "AES Accelerator Key Register"]
pub mod aesakey;
#[doc = "AESADIN register accessor: an alias for `Reg<AESADIN_SPEC>`"]
pub type AESADIN = crate::Reg<aesadin::AESADIN_SPEC>;
#[doc = "AES Accelerator Data In Register"]
pub mod aesadin;
#[doc = "AESADOUT register accessor: an alias for `Reg<AESADOUT_SPEC>`"]
pub type AESADOUT = crate::Reg<aesadout::AESADOUT_SPEC>;
#[doc = "AES Accelerator Data Out Register"]
pub mod aesadout;
#[doc = "AESAXDIN register accessor: an alias for `Reg<AESAXDIN_SPEC>`"]
pub type AESAXDIN = crate::Reg<aesaxdin::AESAXDIN_SPEC>;
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxdin;
#[doc = "AESAXIN register accessor: an alias for `Reg<AESAXIN_SPEC>`"]
pub type AESAXIN = crate::Reg<aesaxin::AESAXIN_SPEC>;
#[doc = "AES Accelerator XORed Data In Register"]
pub mod aesaxin;
