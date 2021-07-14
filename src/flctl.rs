#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Power Status Register"]
    pub flctl_power_stat: crate::Reg<flctl_power_stat::FLCTL_POWER_STAT_SPEC>,
    _reserved1: [u8; 0x0c],
    #[doc = "0x10 - Bank0 Read Control Register"]
    pub flctl_bank0_rdctl: crate::Reg<flctl_bank0_rdctl::FLCTL_BANK0_RDCTL_SPEC>,
    #[doc = "0x14 - Bank1 Read Control Register"]
    pub flctl_bank1_rdctl: crate::Reg<flctl_bank1_rdctl::FLCTL_BANK1_RDCTL_SPEC>,
    _reserved3: [u8; 0x08],
    #[doc = "0x20 - Read Burst/Compare Control and Status Register"]
    pub flctl_rdbrst_ctlstat: crate::Reg<flctl_rdbrst_ctlstat::FLCTL_RDBRST_CTLSTAT_SPEC>,
    #[doc = "0x24 - Read Burst/Compare Start Address Register"]
    pub flctl_rdbrst_startaddr: crate::Reg<flctl_rdbrst_startaddr::FLCTL_RDBRST_STARTADDR_SPEC>,
    #[doc = "0x28 - Read Burst/Compare Length Register"]
    pub flctl_rdbrst_len: crate::Reg<flctl_rdbrst_len::FLCTL_RDBRST_LEN_SPEC>,
    _reserved6: [u8; 0x10],
    #[doc = "0x3c - Read Burst/Compare Fail Address Register"]
    pub flctl_rdbrst_failaddr: crate::Reg<flctl_rdbrst_failaddr::FLCTL_RDBRST_FAILADDR_SPEC>,
    #[doc = "0x40 - Read Burst/Compare Fail Count Register"]
    pub flctl_rdbrst_failcnt: crate::Reg<flctl_rdbrst_failcnt::FLCTL_RDBRST_FAILCNT_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x50 - Program Control and Status Register"]
    pub flctl_prg_ctlstat: crate::Reg<flctl_prg_ctlstat::FLCTL_PRG_CTLSTAT_SPEC>,
    #[doc = "0x54 - Program Burst Control and Status Register"]
    pub flctl_prgbrst_ctlstat: crate::Reg<flctl_prgbrst_ctlstat::FLCTL_PRGBRST_CTLSTAT_SPEC>,
    #[doc = "0x58 - Program Burst Start Address Register"]
    pub flctl_prgbrst_startaddr: crate::Reg<flctl_prgbrst_startaddr::FLCTL_PRGBRST_STARTADDR_SPEC>,
    _reserved11: [u8; 0x04],
    #[doc = "0x60 - Program Burst Data0 Register0"]
    pub flctl_prgbrst_data0_0: crate::Reg<flctl_prgbrst_data0_0::FLCTL_PRGBRST_DATA0_0_SPEC>,
    #[doc = "0x64 - Program Burst Data0 Register1"]
    pub flctl_prgbrst_data0_1: crate::Reg<flctl_prgbrst_data0_1::FLCTL_PRGBRST_DATA0_1_SPEC>,
    #[doc = "0x68 - Program Burst Data0 Register2"]
    pub flctl_prgbrst_data0_2: crate::Reg<flctl_prgbrst_data0_2::FLCTL_PRGBRST_DATA0_2_SPEC>,
    #[doc = "0x6c - Program Burst Data0 Register3"]
    pub flctl_prgbrst_data0_3: crate::Reg<flctl_prgbrst_data0_3::FLCTL_PRGBRST_DATA0_3_SPEC>,
    #[doc = "0x70 - Program Burst Data1 Register0"]
    pub flctl_prgbrst_data1_0: crate::Reg<flctl_prgbrst_data1_0::FLCTL_PRGBRST_DATA1_0_SPEC>,
    #[doc = "0x74 - Program Burst Data1 Register1"]
    pub flctl_prgbrst_data1_1: crate::Reg<flctl_prgbrst_data1_1::FLCTL_PRGBRST_DATA1_1_SPEC>,
    #[doc = "0x78 - Program Burst Data1 Register2"]
    pub flctl_prgbrst_data1_2: crate::Reg<flctl_prgbrst_data1_2::FLCTL_PRGBRST_DATA1_2_SPEC>,
    #[doc = "0x7c - Program Burst Data1 Register3"]
    pub flctl_prgbrst_data1_3: crate::Reg<flctl_prgbrst_data1_3::FLCTL_PRGBRST_DATA1_3_SPEC>,
    #[doc = "0x80 - Program Burst Data2 Register0"]
    pub flctl_prgbrst_data2_0: crate::Reg<flctl_prgbrst_data2_0::FLCTL_PRGBRST_DATA2_0_SPEC>,
    #[doc = "0x84 - Program Burst Data2 Register1"]
    pub flctl_prgbrst_data2_1: crate::Reg<flctl_prgbrst_data2_1::FLCTL_PRGBRST_DATA2_1_SPEC>,
    #[doc = "0x88 - Program Burst Data2 Register2"]
    pub flctl_prgbrst_data2_2: crate::Reg<flctl_prgbrst_data2_2::FLCTL_PRGBRST_DATA2_2_SPEC>,
    #[doc = "0x8c - Program Burst Data2 Register3"]
    pub flctl_prgbrst_data2_3: crate::Reg<flctl_prgbrst_data2_3::FLCTL_PRGBRST_DATA2_3_SPEC>,
    #[doc = "0x90 - Program Burst Data3 Register0"]
    pub flctl_prgbrst_data3_0: crate::Reg<flctl_prgbrst_data3_0::FLCTL_PRGBRST_DATA3_0_SPEC>,
    #[doc = "0x94 - Program Burst Data3 Register1"]
    pub flctl_prgbrst_data3_1: crate::Reg<flctl_prgbrst_data3_1::FLCTL_PRGBRST_DATA3_1_SPEC>,
    #[doc = "0x98 - Program Burst Data3 Register2"]
    pub flctl_prgbrst_data3_2: crate::Reg<flctl_prgbrst_data3_2::FLCTL_PRGBRST_DATA3_2_SPEC>,
    #[doc = "0x9c - Program Burst Data3 Register3"]
    pub flctl_prgbrst_data3_3: crate::Reg<flctl_prgbrst_data3_3::FLCTL_PRGBRST_DATA3_3_SPEC>,
    #[doc = "0xa0 - Erase Control and Status Register"]
    pub flctl_erase_ctlstat: crate::Reg<flctl_erase_ctlstat::FLCTL_ERASE_CTLSTAT_SPEC>,
    #[doc = "0xa4 - Erase Sector Address Register"]
    pub flctl_erase_sectaddr: crate::Reg<flctl_erase_sectaddr::FLCTL_ERASE_SECTADDR_SPEC>,
    _reserved29: [u8; 0x08],
    #[doc = "0xb0 - Information Memory Bank0 Write/Erase Protection Register"]
    pub flctl_bank0_info_weprot: crate::Reg<flctl_bank0_info_weprot::FLCTL_BANK0_INFO_WEPROT_SPEC>,
    #[doc = "0xb4 - Main Memory Bank0 Write/Erase Protection Register"]
    pub flctl_bank0_main_weprot: crate::Reg<flctl_bank0_main_weprot::FLCTL_BANK0_MAIN_WEPROT_SPEC>,
    _reserved31: [u8; 0x08],
    #[doc = "0xc0 - Information Memory Bank1 Write/Erase Protection Register"]
    pub flctl_bank1_info_weprot: crate::Reg<flctl_bank1_info_weprot::FLCTL_BANK1_INFO_WEPROT_SPEC>,
    #[doc = "0xc4 - Main Memory Bank1 Write/Erase Protection Register"]
    pub flctl_bank1_main_weprot: crate::Reg<flctl_bank1_main_weprot::FLCTL_BANK1_MAIN_WEPROT_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0xd0 - Benchmark Control and Status Register"]
    pub flctl_bmrk_ctlstat: crate::Reg<flctl_bmrk_ctlstat::FLCTL_BMRK_CTLSTAT_SPEC>,
    #[doc = "0xd4 - Benchmark Instruction Fetch Count Register"]
    pub flctl_bmrk_ifetch: crate::Reg<flctl_bmrk_ifetch::FLCTL_BMRK_IFETCH_SPEC>,
    #[doc = "0xd8 - Benchmark Data Read Count Register"]
    pub flctl_bmrk_dread: crate::Reg<flctl_bmrk_dread::FLCTL_BMRK_DREAD_SPEC>,
    #[doc = "0xdc - Benchmark Count Compare Register"]
    pub flctl_bmrk_cmp: crate::Reg<flctl_bmrk_cmp::FLCTL_BMRK_CMP_SPEC>,
    _reserved37: [u8; 0x10],
    #[doc = "0xf0 - Interrupt Flag Register"]
    pub flctl_ifg: crate::Reg<flctl_ifg::FLCTL_IFG_SPEC>,
    #[doc = "0xf4 - Interrupt Enable Register"]
    pub flctl_ie: crate::Reg<flctl_ie::FLCTL_IE_SPEC>,
    #[doc = "0xf8 - Clear Interrupt Flag Register"]
    pub flctl_clrifg: crate::Reg<flctl_clrifg::FLCTL_CLRIFG_SPEC>,
    #[doc = "0xfc - Set Interrupt Flag Register"]
    pub flctl_setifg: crate::Reg<flctl_setifg::FLCTL_SETIFG_SPEC>,
    #[doc = "0x100 - Read Timing Control Register"]
    pub flctl_read_timctl: crate::Reg<flctl_read_timctl::FLCTL_READ_TIMCTL_SPEC>,
    #[doc = "0x104 - Read Margin Timing Control Register"]
    pub flctl_readmargin_timctl: crate::Reg<flctl_readmargin_timctl::FLCTL_READMARGIN_TIMCTL_SPEC>,
    #[doc = "0x108 - Program Verify Timing Control Register"]
    pub flctl_prgver_timctl: crate::Reg<flctl_prgver_timctl::FLCTL_PRGVER_TIMCTL_SPEC>,
    #[doc = "0x10c - Erase Verify Timing Control Register"]
    pub flctl_ersver_timctl: crate::Reg<flctl_ersver_timctl::FLCTL_ERSVER_TIMCTL_SPEC>,
    #[doc = "0x110 - Leakage Verify Timing Control Register"]
    pub flctl_lkgver_timctl: crate::Reg<flctl_lkgver_timctl::FLCTL_LKGVER_TIMCTL_SPEC>,
    #[doc = "0x114 - Program Timing Control Register"]
    pub flctl_program_timctl: crate::Reg<flctl_program_timctl::FLCTL_PROGRAM_TIMCTL_SPEC>,
    #[doc = "0x118 - Erase Timing Control Register"]
    pub flctl_erase_timctl: crate::Reg<flctl_erase_timctl::FLCTL_ERASE_TIMCTL_SPEC>,
    #[doc = "0x11c - Mass Erase Timing Control Register"]
    pub flctl_masserase_timctl: crate::Reg<flctl_masserase_timctl::FLCTL_MASSERASE_TIMCTL_SPEC>,
    #[doc = "0x120 - Burst Program Timing Control Register"]
    pub flctl_burstprg_timctl: crate::Reg<flctl_burstprg_timctl::FLCTL_BURSTPRG_TIMCTL_SPEC>,
}
#[doc = "FLCTL_POWER_STAT register accessor: an alias for `Reg<FLCTL_POWER_STAT_SPEC>`"]
pub type FLCTL_POWER_STAT = crate::Reg<flctl_power_stat::FLCTL_POWER_STAT_SPEC>;
#[doc = "Power Status Register"]
pub mod flctl_power_stat;
#[doc = "FLCTL_BANK0_RDCTL register accessor: an alias for `Reg<FLCTL_BANK0_RDCTL_SPEC>`"]
pub type FLCTL_BANK0_RDCTL = crate::Reg<flctl_bank0_rdctl::FLCTL_BANK0_RDCTL_SPEC>;
#[doc = "Bank0 Read Control Register"]
pub mod flctl_bank0_rdctl;
#[doc = "FLCTL_BANK1_RDCTL register accessor: an alias for `Reg<FLCTL_BANK1_RDCTL_SPEC>`"]
pub type FLCTL_BANK1_RDCTL = crate::Reg<flctl_bank1_rdctl::FLCTL_BANK1_RDCTL_SPEC>;
#[doc = "Bank1 Read Control Register"]
pub mod flctl_bank1_rdctl;
#[doc = "FLCTL_RDBRST_CTLSTAT register accessor: an alias for `Reg<FLCTL_RDBRST_CTLSTAT_SPEC>`"]
pub type FLCTL_RDBRST_CTLSTAT = crate::Reg<flctl_rdbrst_ctlstat::FLCTL_RDBRST_CTLSTAT_SPEC>;
#[doc = "Read Burst/Compare Control and Status Register"]
pub mod flctl_rdbrst_ctlstat;
#[doc = "FLCTL_RDBRST_STARTADDR register accessor: an alias for `Reg<FLCTL_RDBRST_STARTADDR_SPEC>`"]
pub type FLCTL_RDBRST_STARTADDR = crate::Reg<flctl_rdbrst_startaddr::FLCTL_RDBRST_STARTADDR_SPEC>;
#[doc = "Read Burst/Compare Start Address Register"]
pub mod flctl_rdbrst_startaddr;
#[doc = "FLCTL_RDBRST_LEN register accessor: an alias for `Reg<FLCTL_RDBRST_LEN_SPEC>`"]
pub type FLCTL_RDBRST_LEN = crate::Reg<flctl_rdbrst_len::FLCTL_RDBRST_LEN_SPEC>;
#[doc = "Read Burst/Compare Length Register"]
pub mod flctl_rdbrst_len;
#[doc = "FLCTL_RDBRST_FAILADDR register accessor: an alias for `Reg<FLCTL_RDBRST_FAILADDR_SPEC>`"]
pub type FLCTL_RDBRST_FAILADDR = crate::Reg<flctl_rdbrst_failaddr::FLCTL_RDBRST_FAILADDR_SPEC>;
#[doc = "Read Burst/Compare Fail Address Register"]
pub mod flctl_rdbrst_failaddr;
#[doc = "FLCTL_RDBRST_FAILCNT register accessor: an alias for `Reg<FLCTL_RDBRST_FAILCNT_SPEC>`"]
pub type FLCTL_RDBRST_FAILCNT = crate::Reg<flctl_rdbrst_failcnt::FLCTL_RDBRST_FAILCNT_SPEC>;
#[doc = "Read Burst/Compare Fail Count Register"]
pub mod flctl_rdbrst_failcnt;
#[doc = "FLCTL_PRG_CTLSTAT register accessor: an alias for `Reg<FLCTL_PRG_CTLSTAT_SPEC>`"]
pub type FLCTL_PRG_CTLSTAT = crate::Reg<flctl_prg_ctlstat::FLCTL_PRG_CTLSTAT_SPEC>;
#[doc = "Program Control and Status Register"]
pub mod flctl_prg_ctlstat;
#[doc = "FLCTL_PRGBRST_CTLSTAT register accessor: an alias for `Reg<FLCTL_PRGBRST_CTLSTAT_SPEC>`"]
pub type FLCTL_PRGBRST_CTLSTAT = crate::Reg<flctl_prgbrst_ctlstat::FLCTL_PRGBRST_CTLSTAT_SPEC>;
#[doc = "Program Burst Control and Status Register"]
pub mod flctl_prgbrst_ctlstat;
#[doc = "FLCTL_PRGBRST_STARTADDR register accessor: an alias for `Reg<FLCTL_PRGBRST_STARTADDR_SPEC>`"]
pub type FLCTL_PRGBRST_STARTADDR =
    crate::Reg<flctl_prgbrst_startaddr::FLCTL_PRGBRST_STARTADDR_SPEC>;
#[doc = "Program Burst Start Address Register"]
pub mod flctl_prgbrst_startaddr;
#[doc = "FLCTL_PRGBRST_DATA0_0 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA0_0_SPEC>`"]
pub type FLCTL_PRGBRST_DATA0_0 = crate::Reg<flctl_prgbrst_data0_0::FLCTL_PRGBRST_DATA0_0_SPEC>;
#[doc = "Program Burst Data0 Register0"]
pub mod flctl_prgbrst_data0_0;
#[doc = "FLCTL_PRGBRST_DATA0_1 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA0_1_SPEC>`"]
pub type FLCTL_PRGBRST_DATA0_1 = crate::Reg<flctl_prgbrst_data0_1::FLCTL_PRGBRST_DATA0_1_SPEC>;
#[doc = "Program Burst Data0 Register1"]
pub mod flctl_prgbrst_data0_1;
#[doc = "FLCTL_PRGBRST_DATA0_2 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA0_2_SPEC>`"]
pub type FLCTL_PRGBRST_DATA0_2 = crate::Reg<flctl_prgbrst_data0_2::FLCTL_PRGBRST_DATA0_2_SPEC>;
#[doc = "Program Burst Data0 Register2"]
pub mod flctl_prgbrst_data0_2;
#[doc = "FLCTL_PRGBRST_DATA0_3 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA0_3_SPEC>`"]
pub type FLCTL_PRGBRST_DATA0_3 = crate::Reg<flctl_prgbrst_data0_3::FLCTL_PRGBRST_DATA0_3_SPEC>;
#[doc = "Program Burst Data0 Register3"]
pub mod flctl_prgbrst_data0_3;
#[doc = "FLCTL_PRGBRST_DATA1_0 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA1_0_SPEC>`"]
pub type FLCTL_PRGBRST_DATA1_0 = crate::Reg<flctl_prgbrst_data1_0::FLCTL_PRGBRST_DATA1_0_SPEC>;
#[doc = "Program Burst Data1 Register0"]
pub mod flctl_prgbrst_data1_0;
#[doc = "FLCTL_PRGBRST_DATA1_1 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA1_1_SPEC>`"]
pub type FLCTL_PRGBRST_DATA1_1 = crate::Reg<flctl_prgbrst_data1_1::FLCTL_PRGBRST_DATA1_1_SPEC>;
#[doc = "Program Burst Data1 Register1"]
pub mod flctl_prgbrst_data1_1;
#[doc = "FLCTL_PRGBRST_DATA1_2 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA1_2_SPEC>`"]
pub type FLCTL_PRGBRST_DATA1_2 = crate::Reg<flctl_prgbrst_data1_2::FLCTL_PRGBRST_DATA1_2_SPEC>;
#[doc = "Program Burst Data1 Register2"]
pub mod flctl_prgbrst_data1_2;
#[doc = "FLCTL_PRGBRST_DATA1_3 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA1_3_SPEC>`"]
pub type FLCTL_PRGBRST_DATA1_3 = crate::Reg<flctl_prgbrst_data1_3::FLCTL_PRGBRST_DATA1_3_SPEC>;
#[doc = "Program Burst Data1 Register3"]
pub mod flctl_prgbrst_data1_3;
#[doc = "FLCTL_PRGBRST_DATA2_0 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA2_0_SPEC>`"]
pub type FLCTL_PRGBRST_DATA2_0 = crate::Reg<flctl_prgbrst_data2_0::FLCTL_PRGBRST_DATA2_0_SPEC>;
#[doc = "Program Burst Data2 Register0"]
pub mod flctl_prgbrst_data2_0;
#[doc = "FLCTL_PRGBRST_DATA2_1 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA2_1_SPEC>`"]
pub type FLCTL_PRGBRST_DATA2_1 = crate::Reg<flctl_prgbrst_data2_1::FLCTL_PRGBRST_DATA2_1_SPEC>;
#[doc = "Program Burst Data2 Register1"]
pub mod flctl_prgbrst_data2_1;
#[doc = "FLCTL_PRGBRST_DATA2_2 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA2_2_SPEC>`"]
pub type FLCTL_PRGBRST_DATA2_2 = crate::Reg<flctl_prgbrst_data2_2::FLCTL_PRGBRST_DATA2_2_SPEC>;
#[doc = "Program Burst Data2 Register2"]
pub mod flctl_prgbrst_data2_2;
#[doc = "FLCTL_PRGBRST_DATA2_3 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA2_3_SPEC>`"]
pub type FLCTL_PRGBRST_DATA2_3 = crate::Reg<flctl_prgbrst_data2_3::FLCTL_PRGBRST_DATA2_3_SPEC>;
#[doc = "Program Burst Data2 Register3"]
pub mod flctl_prgbrst_data2_3;
#[doc = "FLCTL_PRGBRST_DATA3_0 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA3_0_SPEC>`"]
pub type FLCTL_PRGBRST_DATA3_0 = crate::Reg<flctl_prgbrst_data3_0::FLCTL_PRGBRST_DATA3_0_SPEC>;
#[doc = "Program Burst Data3 Register0"]
pub mod flctl_prgbrst_data3_0;
#[doc = "FLCTL_PRGBRST_DATA3_1 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA3_1_SPEC>`"]
pub type FLCTL_PRGBRST_DATA3_1 = crate::Reg<flctl_prgbrst_data3_1::FLCTL_PRGBRST_DATA3_1_SPEC>;
#[doc = "Program Burst Data3 Register1"]
pub mod flctl_prgbrst_data3_1;
#[doc = "FLCTL_PRGBRST_DATA3_2 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA3_2_SPEC>`"]
pub type FLCTL_PRGBRST_DATA3_2 = crate::Reg<flctl_prgbrst_data3_2::FLCTL_PRGBRST_DATA3_2_SPEC>;
#[doc = "Program Burst Data3 Register2"]
pub mod flctl_prgbrst_data3_2;
#[doc = "FLCTL_PRGBRST_DATA3_3 register accessor: an alias for `Reg<FLCTL_PRGBRST_DATA3_3_SPEC>`"]
pub type FLCTL_PRGBRST_DATA3_3 = crate::Reg<flctl_prgbrst_data3_3::FLCTL_PRGBRST_DATA3_3_SPEC>;
#[doc = "Program Burst Data3 Register3"]
pub mod flctl_prgbrst_data3_3;
#[doc = "FLCTL_ERASE_CTLSTAT register accessor: an alias for `Reg<FLCTL_ERASE_CTLSTAT_SPEC>`"]
pub type FLCTL_ERASE_CTLSTAT = crate::Reg<flctl_erase_ctlstat::FLCTL_ERASE_CTLSTAT_SPEC>;
#[doc = "Erase Control and Status Register"]
pub mod flctl_erase_ctlstat;
#[doc = "FLCTL_ERASE_SECTADDR register accessor: an alias for `Reg<FLCTL_ERASE_SECTADDR_SPEC>`"]
pub type FLCTL_ERASE_SECTADDR = crate::Reg<flctl_erase_sectaddr::FLCTL_ERASE_SECTADDR_SPEC>;
#[doc = "Erase Sector Address Register"]
pub mod flctl_erase_sectaddr;
#[doc = "FLCTL_BANK0_INFO_WEPROT register accessor: an alias for `Reg<FLCTL_BANK0_INFO_WEPROT_SPEC>`"]
pub type FLCTL_BANK0_INFO_WEPROT =
    crate::Reg<flctl_bank0_info_weprot::FLCTL_BANK0_INFO_WEPROT_SPEC>;
#[doc = "Information Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_info_weprot;
#[doc = "FLCTL_BANK0_MAIN_WEPROT register accessor: an alias for `Reg<FLCTL_BANK0_MAIN_WEPROT_SPEC>`"]
pub type FLCTL_BANK0_MAIN_WEPROT =
    crate::Reg<flctl_bank0_main_weprot::FLCTL_BANK0_MAIN_WEPROT_SPEC>;
#[doc = "Main Memory Bank0 Write/Erase Protection Register"]
pub mod flctl_bank0_main_weprot;
#[doc = "FLCTL_BANK1_INFO_WEPROT register accessor: an alias for `Reg<FLCTL_BANK1_INFO_WEPROT_SPEC>`"]
pub type FLCTL_BANK1_INFO_WEPROT =
    crate::Reg<flctl_bank1_info_weprot::FLCTL_BANK1_INFO_WEPROT_SPEC>;
#[doc = "Information Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_info_weprot;
#[doc = "FLCTL_BANK1_MAIN_WEPROT register accessor: an alias for `Reg<FLCTL_BANK1_MAIN_WEPROT_SPEC>`"]
pub type FLCTL_BANK1_MAIN_WEPROT =
    crate::Reg<flctl_bank1_main_weprot::FLCTL_BANK1_MAIN_WEPROT_SPEC>;
#[doc = "Main Memory Bank1 Write/Erase Protection Register"]
pub mod flctl_bank1_main_weprot;
#[doc = "FLCTL_BMRK_CTLSTAT register accessor: an alias for `Reg<FLCTL_BMRK_CTLSTAT_SPEC>`"]
pub type FLCTL_BMRK_CTLSTAT = crate::Reg<flctl_bmrk_ctlstat::FLCTL_BMRK_CTLSTAT_SPEC>;
#[doc = "Benchmark Control and Status Register"]
pub mod flctl_bmrk_ctlstat;
#[doc = "FLCTL_BMRK_IFETCH register accessor: an alias for `Reg<FLCTL_BMRK_IFETCH_SPEC>`"]
pub type FLCTL_BMRK_IFETCH = crate::Reg<flctl_bmrk_ifetch::FLCTL_BMRK_IFETCH_SPEC>;
#[doc = "Benchmark Instruction Fetch Count Register"]
pub mod flctl_bmrk_ifetch;
#[doc = "FLCTL_BMRK_DREAD register accessor: an alias for `Reg<FLCTL_BMRK_DREAD_SPEC>`"]
pub type FLCTL_BMRK_DREAD = crate::Reg<flctl_bmrk_dread::FLCTL_BMRK_DREAD_SPEC>;
#[doc = "Benchmark Data Read Count Register"]
pub mod flctl_bmrk_dread;
#[doc = "FLCTL_BMRK_CMP register accessor: an alias for `Reg<FLCTL_BMRK_CMP_SPEC>`"]
pub type FLCTL_BMRK_CMP = crate::Reg<flctl_bmrk_cmp::FLCTL_BMRK_CMP_SPEC>;
#[doc = "Benchmark Count Compare Register"]
pub mod flctl_bmrk_cmp;
#[doc = "FLCTL_IFG register accessor: an alias for `Reg<FLCTL_IFG_SPEC>`"]
pub type FLCTL_IFG = crate::Reg<flctl_ifg::FLCTL_IFG_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod flctl_ifg;
#[doc = "FLCTL_IE register accessor: an alias for `Reg<FLCTL_IE_SPEC>`"]
pub type FLCTL_IE = crate::Reg<flctl_ie::FLCTL_IE_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod flctl_ie;
#[doc = "FLCTL_CLRIFG register accessor: an alias for `Reg<FLCTL_CLRIFG_SPEC>`"]
pub type FLCTL_CLRIFG = crate::Reg<flctl_clrifg::FLCTL_CLRIFG_SPEC>;
#[doc = "Clear Interrupt Flag Register"]
pub mod flctl_clrifg;
#[doc = "FLCTL_SETIFG register accessor: an alias for `Reg<FLCTL_SETIFG_SPEC>`"]
pub type FLCTL_SETIFG = crate::Reg<flctl_setifg::FLCTL_SETIFG_SPEC>;
#[doc = "Set Interrupt Flag Register"]
pub mod flctl_setifg;
#[doc = "FLCTL_READ_TIMCTL register accessor: an alias for `Reg<FLCTL_READ_TIMCTL_SPEC>`"]
pub type FLCTL_READ_TIMCTL = crate::Reg<flctl_read_timctl::FLCTL_READ_TIMCTL_SPEC>;
#[doc = "Read Timing Control Register"]
pub mod flctl_read_timctl;
#[doc = "FLCTL_READMARGIN_TIMCTL register accessor: an alias for `Reg<FLCTL_READMARGIN_TIMCTL_SPEC>`"]
pub type FLCTL_READMARGIN_TIMCTL =
    crate::Reg<flctl_readmargin_timctl::FLCTL_READMARGIN_TIMCTL_SPEC>;
#[doc = "Read Margin Timing Control Register"]
pub mod flctl_readmargin_timctl;
#[doc = "FLCTL_PRGVER_TIMCTL register accessor: an alias for `Reg<FLCTL_PRGVER_TIMCTL_SPEC>`"]
pub type FLCTL_PRGVER_TIMCTL = crate::Reg<flctl_prgver_timctl::FLCTL_PRGVER_TIMCTL_SPEC>;
#[doc = "Program Verify Timing Control Register"]
pub mod flctl_prgver_timctl;
#[doc = "FLCTL_ERSVER_TIMCTL register accessor: an alias for `Reg<FLCTL_ERSVER_TIMCTL_SPEC>`"]
pub type FLCTL_ERSVER_TIMCTL = crate::Reg<flctl_ersver_timctl::FLCTL_ERSVER_TIMCTL_SPEC>;
#[doc = "Erase Verify Timing Control Register"]
pub mod flctl_ersver_timctl;
#[doc = "FLCTL_LKGVER_TIMCTL register accessor: an alias for `Reg<FLCTL_LKGVER_TIMCTL_SPEC>`"]
pub type FLCTL_LKGVER_TIMCTL = crate::Reg<flctl_lkgver_timctl::FLCTL_LKGVER_TIMCTL_SPEC>;
#[doc = "Leakage Verify Timing Control Register"]
pub mod flctl_lkgver_timctl;
#[doc = "FLCTL_PROGRAM_TIMCTL register accessor: an alias for `Reg<FLCTL_PROGRAM_TIMCTL_SPEC>`"]
pub type FLCTL_PROGRAM_TIMCTL = crate::Reg<flctl_program_timctl::FLCTL_PROGRAM_TIMCTL_SPEC>;
#[doc = "Program Timing Control Register"]
pub mod flctl_program_timctl;
#[doc = "FLCTL_ERASE_TIMCTL register accessor: an alias for `Reg<FLCTL_ERASE_TIMCTL_SPEC>`"]
pub type FLCTL_ERASE_TIMCTL = crate::Reg<flctl_erase_timctl::FLCTL_ERASE_TIMCTL_SPEC>;
#[doc = "Erase Timing Control Register"]
pub mod flctl_erase_timctl;
#[doc = "FLCTL_MASSERASE_TIMCTL register accessor: an alias for `Reg<FLCTL_MASSERASE_TIMCTL_SPEC>`"]
pub type FLCTL_MASSERASE_TIMCTL = crate::Reg<flctl_masserase_timctl::FLCTL_MASSERASE_TIMCTL_SPEC>;
#[doc = "Mass Erase Timing Control Register"]
pub mod flctl_masserase_timctl;
#[doc = "FLCTL_BURSTPRG_TIMCTL register accessor: an alias for `Reg<FLCTL_BURSTPRG_TIMCTL_SPEC>`"]
pub type FLCTL_BURSTPRG_TIMCTL = crate::Reg<flctl_burstprg_timctl::FLCTL_BURSTPRG_TIMCTL_SPEC>;
#[doc = "Burst Program Timing Control Register"]
pub mod flctl_burstprg_timctl;
