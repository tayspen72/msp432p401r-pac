#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Data Input for CRC32 Signature Computation"]
    pub crc32di: crate::Reg<crc32di::CRC32DI_SPEC>,
    _reserved1: [u8; 0x02],
    #[doc = "0x04 - Data In Reverse for CRC32 Computation"]
    pub crc32dirb: crate::Reg<crc32dirb::CRC32DIRB_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x08 - CRC32 Initialization and Result, lower 16 bits"]
    pub crc32inires_lo: crate::Reg<crc32inires_lo::CRC32INIRES_LO_SPEC>,
    #[doc = "0x0a - CRC32 Initialization and Result, upper 16 bits"]
    pub crc32inires_hi: crate::Reg<crc32inires_hi::CRC32INIRES_HI_SPEC>,
    #[doc = "0x0c - CRC32 Result Reverse, lower 16 bits"]
    pub crc32resr_lo: crate::Reg<crc32resr_lo::CRC32RESR_LO_SPEC>,
    #[doc = "0x0e - CRC32 Result Reverse, Upper 16 bits"]
    pub crc32resr_hi: crate::Reg<crc32resr_hi::CRC32RESR_HI_SPEC>,
    #[doc = "0x10 - Data Input for CRC16 computation"]
    pub crc16di: crate::Reg<crc16di::CRC16DI_SPEC>,
    _reserved7: [u8; 0x02],
    #[doc = "0x14 - CRC16 Data In Reverse"]
    pub crc16dirb: crate::Reg<crc16dirb::CRC16DIRB_SPEC>,
    _reserved8: [u8; 0x02],
    #[doc = "0x18 - CRC16 Initialization and Result register"]
    pub crc16inires: crate::Reg<crc16inires::CRC16INIRES_SPEC>,
    _reserved9: [u8; 0x04],
    #[doc = "0x1e - CRC16 Result Reverse"]
    pub crc16resr: crate::Reg<crc16resr::CRC16RESR_SPEC>,
}
#[doc = "CRC32DI register accessor: an alias for `Reg<CRC32DI_SPEC>`"]
pub type CRC32DI = crate::Reg<crc32di::CRC32DI_SPEC>;
#[doc = "Data Input for CRC32 Signature Computation"]
pub mod crc32di;
#[doc = "CRC32DIRB register accessor: an alias for `Reg<CRC32DIRB_SPEC>`"]
pub type CRC32DIRB = crate::Reg<crc32dirb::CRC32DIRB_SPEC>;
#[doc = "Data In Reverse for CRC32 Computation"]
pub mod crc32dirb;
#[doc = "CRC32INIRES_LO register accessor: an alias for `Reg<CRC32INIRES_LO_SPEC>`"]
pub type CRC32INIRES_LO = crate::Reg<crc32inires_lo::CRC32INIRES_LO_SPEC>;
#[doc = "CRC32 Initialization and Result, lower 16 bits"]
pub mod crc32inires_lo;
#[doc = "CRC32INIRES_HI register accessor: an alias for `Reg<CRC32INIRES_HI_SPEC>`"]
pub type CRC32INIRES_HI = crate::Reg<crc32inires_hi::CRC32INIRES_HI_SPEC>;
#[doc = "CRC32 Initialization and Result, upper 16 bits"]
pub mod crc32inires_hi;
#[doc = "CRC32RESR_LO register accessor: an alias for `Reg<CRC32RESR_LO_SPEC>`"]
pub type CRC32RESR_LO = crate::Reg<crc32resr_lo::CRC32RESR_LO_SPEC>;
#[doc = "CRC32 Result Reverse, lower 16 bits"]
pub mod crc32resr_lo;
#[doc = "CRC32RESR_HI register accessor: an alias for `Reg<CRC32RESR_HI_SPEC>`"]
pub type CRC32RESR_HI = crate::Reg<crc32resr_hi::CRC32RESR_HI_SPEC>;
#[doc = "CRC32 Result Reverse, Upper 16 bits"]
pub mod crc32resr_hi;
#[doc = "CRC16DI register accessor: an alias for `Reg<CRC16DI_SPEC>`"]
pub type CRC16DI = crate::Reg<crc16di::CRC16DI_SPEC>;
#[doc = "Data Input for CRC16 computation"]
pub mod crc16di;
#[doc = "CRC16DIRB register accessor: an alias for `Reg<CRC16DIRB_SPEC>`"]
pub type CRC16DIRB = crate::Reg<crc16dirb::CRC16DIRB_SPEC>;
#[doc = "CRC16 Data In Reverse"]
pub mod crc16dirb;
#[doc = "CRC16INIRES register accessor: an alias for `Reg<CRC16INIRES_SPEC>`"]
pub type CRC16INIRES = crate::Reg<crc16inires::CRC16INIRES_SPEC>;
#[doc = "CRC16 Initialization and Result register"]
pub mod crc16inires;
#[doc = "CRC16RESR register accessor: an alias for `Reg<CRC16RESR_SPEC>`"]
pub type CRC16RESR = crate::Reg<crc16resr::CRC16RESR_SPEC>;
#[doc = "CRC16 Result Reverse"]
pub mod crc16resr;
