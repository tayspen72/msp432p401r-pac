#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reset Request Register"]
    pub rstctl_reset_req: crate::Reg<rstctl_reset_req::RSTCTL_RESET_REQ_SPEC>,
    #[doc = "0x04 - Hard Reset Status Register"]
    pub rstctl_hardreset_stat: crate::Reg<rstctl_hardreset_stat::RSTCTL_HARDRESET_STAT_SPEC>,
    #[doc = "0x08 - Hard Reset Status Clear Register"]
    pub rstctl_hardreset_clr: crate::Reg<rstctl_hardreset_clr::RSTCTL_HARDRESET_CLR_SPEC>,
    #[doc = "0x0c - Hard Reset Status Set Register"]
    pub rstctl_hardreset_set: crate::Reg<rstctl_hardreset_set::RSTCTL_HARDRESET_SET_SPEC>,
    #[doc = "0x10 - Soft Reset Status Register"]
    pub rstctl_softreset_stat: crate::Reg<rstctl_softreset_stat::RSTCTL_SOFTRESET_STAT_SPEC>,
    #[doc = "0x14 - Soft Reset Status Clear Register"]
    pub rstctl_softreset_clr: crate::Reg<rstctl_softreset_clr::RSTCTL_SOFTRESET_CLR_SPEC>,
    #[doc = "0x18 - Soft Reset Status Set Register"]
    pub rstctl_softreset_set: crate::Reg<rstctl_softreset_set::RSTCTL_SOFTRESET_SET_SPEC>,
    _reserved7: [u8; 0xe4],
    #[doc = "0x100 - PSS Reset Status Register"]
    pub rstctl_pssreset_stat: crate::Reg<rstctl_pssreset_stat::RSTCTL_PSSRESET_STAT_SPEC>,
    #[doc = "0x104 - PSS Reset Status Clear Register"]
    pub rstctl_pssreset_clr: crate::Reg<rstctl_pssreset_clr::RSTCTL_PSSRESET_CLR_SPEC>,
    #[doc = "0x108 - PCM Reset Status Register"]
    pub rstctl_pcmreset_stat: crate::Reg<rstctl_pcmreset_stat::RSTCTL_PCMRESET_STAT_SPEC>,
    #[doc = "0x10c - PCM Reset Status Clear Register"]
    pub rstctl_pcmreset_clr: crate::Reg<rstctl_pcmreset_clr::RSTCTL_PCMRESET_CLR_SPEC>,
    #[doc = "0x110 - Pin Reset Status Register"]
    pub rstctl_pinreset_stat: crate::Reg<rstctl_pinreset_stat::RSTCTL_PINRESET_STAT_SPEC>,
    #[doc = "0x114 - Pin Reset Status Clear Register"]
    pub rstctl_pinreset_clr: crate::Reg<rstctl_pinreset_clr::RSTCTL_PINRESET_CLR_SPEC>,
    #[doc = "0x118 - Reboot Reset Status Register"]
    pub rstctl_rebootreset_stat: crate::Reg<rstctl_rebootreset_stat::RSTCTL_REBOOTRESET_STAT_SPEC>,
    #[doc = "0x11c - Reboot Reset Status Clear Register"]
    pub rstctl_rebootreset_clr: crate::Reg<rstctl_rebootreset_clr::RSTCTL_REBOOTRESET_CLR_SPEC>,
    #[doc = "0x120 - CS Reset Status Register"]
    pub rstctl_csreset_stat: crate::Reg<rstctl_csreset_stat::RSTCTL_CSRESET_STAT_SPEC>,
    #[doc = "0x124 - CS Reset Status Clear Register"]
    pub rstctl_csreset_clr: crate::Reg<rstctl_csreset_clr::RSTCTL_CSRESET_CLR_SPEC>,
}
#[doc = "RSTCTL_RESET_REQ register accessor: an alias for `Reg<RSTCTL_RESET_REQ_SPEC>`"]
pub type RSTCTL_RESET_REQ = crate::Reg<rstctl_reset_req::RSTCTL_RESET_REQ_SPEC>;
#[doc = "Reset Request Register"]
pub mod rstctl_reset_req;
#[doc = "RSTCTL_HARDRESET_STAT register accessor: an alias for `Reg<RSTCTL_HARDRESET_STAT_SPEC>`"]
pub type RSTCTL_HARDRESET_STAT = crate::Reg<rstctl_hardreset_stat::RSTCTL_HARDRESET_STAT_SPEC>;
#[doc = "Hard Reset Status Register"]
pub mod rstctl_hardreset_stat;
#[doc = "RSTCTL_HARDRESET_CLR register accessor: an alias for `Reg<RSTCTL_HARDRESET_CLR_SPEC>`"]
pub type RSTCTL_HARDRESET_CLR = crate::Reg<rstctl_hardreset_clr::RSTCTL_HARDRESET_CLR_SPEC>;
#[doc = "Hard Reset Status Clear Register"]
pub mod rstctl_hardreset_clr;
#[doc = "RSTCTL_HARDRESET_SET register accessor: an alias for `Reg<RSTCTL_HARDRESET_SET_SPEC>`"]
pub type RSTCTL_HARDRESET_SET = crate::Reg<rstctl_hardreset_set::RSTCTL_HARDRESET_SET_SPEC>;
#[doc = "Hard Reset Status Set Register"]
pub mod rstctl_hardreset_set;
#[doc = "RSTCTL_SOFTRESET_STAT register accessor: an alias for `Reg<RSTCTL_SOFTRESET_STAT_SPEC>`"]
pub type RSTCTL_SOFTRESET_STAT = crate::Reg<rstctl_softreset_stat::RSTCTL_SOFTRESET_STAT_SPEC>;
#[doc = "Soft Reset Status Register"]
pub mod rstctl_softreset_stat;
#[doc = "RSTCTL_SOFTRESET_CLR register accessor: an alias for `Reg<RSTCTL_SOFTRESET_CLR_SPEC>`"]
pub type RSTCTL_SOFTRESET_CLR = crate::Reg<rstctl_softreset_clr::RSTCTL_SOFTRESET_CLR_SPEC>;
#[doc = "Soft Reset Status Clear Register"]
pub mod rstctl_softreset_clr;
#[doc = "RSTCTL_SOFTRESET_SET register accessor: an alias for `Reg<RSTCTL_SOFTRESET_SET_SPEC>`"]
pub type RSTCTL_SOFTRESET_SET = crate::Reg<rstctl_softreset_set::RSTCTL_SOFTRESET_SET_SPEC>;
#[doc = "Soft Reset Status Set Register"]
pub mod rstctl_softreset_set;
#[doc = "RSTCTL_PSSRESET_STAT register accessor: an alias for `Reg<RSTCTL_PSSRESET_STAT_SPEC>`"]
pub type RSTCTL_PSSRESET_STAT = crate::Reg<rstctl_pssreset_stat::RSTCTL_PSSRESET_STAT_SPEC>;
#[doc = "PSS Reset Status Register"]
pub mod rstctl_pssreset_stat;
#[doc = "RSTCTL_PSSRESET_CLR register accessor: an alias for `Reg<RSTCTL_PSSRESET_CLR_SPEC>`"]
pub type RSTCTL_PSSRESET_CLR = crate::Reg<rstctl_pssreset_clr::RSTCTL_PSSRESET_CLR_SPEC>;
#[doc = "PSS Reset Status Clear Register"]
pub mod rstctl_pssreset_clr;
#[doc = "RSTCTL_PCMRESET_STAT register accessor: an alias for `Reg<RSTCTL_PCMRESET_STAT_SPEC>`"]
pub type RSTCTL_PCMRESET_STAT = crate::Reg<rstctl_pcmreset_stat::RSTCTL_PCMRESET_STAT_SPEC>;
#[doc = "PCM Reset Status Register"]
pub mod rstctl_pcmreset_stat;
#[doc = "RSTCTL_PCMRESET_CLR register accessor: an alias for `Reg<RSTCTL_PCMRESET_CLR_SPEC>`"]
pub type RSTCTL_PCMRESET_CLR = crate::Reg<rstctl_pcmreset_clr::RSTCTL_PCMRESET_CLR_SPEC>;
#[doc = "PCM Reset Status Clear Register"]
pub mod rstctl_pcmreset_clr;
#[doc = "RSTCTL_PINRESET_STAT register accessor: an alias for `Reg<RSTCTL_PINRESET_STAT_SPEC>`"]
pub type RSTCTL_PINRESET_STAT = crate::Reg<rstctl_pinreset_stat::RSTCTL_PINRESET_STAT_SPEC>;
#[doc = "Pin Reset Status Register"]
pub mod rstctl_pinreset_stat;
#[doc = "RSTCTL_PINRESET_CLR register accessor: an alias for `Reg<RSTCTL_PINRESET_CLR_SPEC>`"]
pub type RSTCTL_PINRESET_CLR = crate::Reg<rstctl_pinreset_clr::RSTCTL_PINRESET_CLR_SPEC>;
#[doc = "Pin Reset Status Clear Register"]
pub mod rstctl_pinreset_clr;
#[doc = "RSTCTL_REBOOTRESET_STAT register accessor: an alias for `Reg<RSTCTL_REBOOTRESET_STAT_SPEC>`"]
pub type RSTCTL_REBOOTRESET_STAT =
    crate::Reg<rstctl_rebootreset_stat::RSTCTL_REBOOTRESET_STAT_SPEC>;
#[doc = "Reboot Reset Status Register"]
pub mod rstctl_rebootreset_stat;
#[doc = "RSTCTL_REBOOTRESET_CLR register accessor: an alias for `Reg<RSTCTL_REBOOTRESET_CLR_SPEC>`"]
pub type RSTCTL_REBOOTRESET_CLR = crate::Reg<rstctl_rebootreset_clr::RSTCTL_REBOOTRESET_CLR_SPEC>;
#[doc = "Reboot Reset Status Clear Register"]
pub mod rstctl_rebootreset_clr;
#[doc = "RSTCTL_CSRESET_STAT register accessor: an alias for `Reg<RSTCTL_CSRESET_STAT_SPEC>`"]
pub type RSTCTL_CSRESET_STAT = crate::Reg<rstctl_csreset_stat::RSTCTL_CSRESET_STAT_SPEC>;
#[doc = "CS Reset Status Register"]
pub mod rstctl_csreset_stat;
#[doc = "RSTCTL_CSRESET_CLR register accessor: an alias for `Reg<RSTCTL_CSRESET_CLR_SPEC>`"]
pub type RSTCTL_CSRESET_CLR = crate::Reg<rstctl_csreset_clr::RSTCTL_CSRESET_CLR_SPEC>;
#[doc = "CS Reset Status Clear Register"]
pub mod rstctl_csreset_clr;
