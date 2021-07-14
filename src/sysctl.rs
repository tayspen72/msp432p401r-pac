#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Reboot Control Register"]
    pub sys_reboot_ctl: crate::Reg<sys_reboot_ctl::SYS_REBOOT_CTL_SPEC>,
    #[doc = "0x04 - NMI Control and Status Register"]
    pub sys_nmi_ctlstat: crate::Reg<sys_nmi_ctlstat::SYS_NMI_CTLSTAT_SPEC>,
    #[doc = "0x08 - Watchdog Reset Control Register"]
    pub sys_wdtreset_ctl: crate::Reg<sys_wdtreset_ctl::SYS_WDTRESET_CTL_SPEC>,
    #[doc = "0x0c - Peripheral Halt Control Register"]
    pub sys_perihalt_ctl: crate::Reg<sys_perihalt_ctl::SYS_PERIHALT_CTL_SPEC>,
    #[doc = "0x10 - SRAM Size Register"]
    pub sys_sram_size: crate::Reg<sys_sram_size::SYS_SRAM_SIZE_SPEC>,
    #[doc = "0x14 - SRAM Bank Enable Register"]
    pub sys_sram_banken: crate::Reg<sys_sram_banken::SYS_SRAM_BANKEN_SPEC>,
    #[doc = "0x18 - SRAM Bank Retention Control Register"]
    pub sys_sram_bankret: crate::Reg<sys_sram_bankret::SYS_SRAM_BANKRET_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - Flash Size Register"]
    pub sys_flash_size: crate::Reg<sys_flash_size::SYS_FLASH_SIZE_SPEC>,
    _reserved8: [u8; 0x0c],
    #[doc = "0x30 - Digital I/O Glitch Filter Control Register"]
    pub sys_dio_gltflt_ctl: crate::Reg<sys_dio_gltflt_ctl::SYS_DIO_GLTFLT_CTL_SPEC>,
    _reserved9: [u8; 0x0c],
    #[doc = "0x40 - IP Protected Secure Zone Data Access Unlock Register"]
    pub sys_secdata_unlock: crate::Reg<sys_secdata_unlock::SYS_SECDATA_UNLOCK_SPEC>,
    _reserved10: [u8; 0x0fbc],
    #[doc = "0x1000 - Master Unlock Register"]
    pub sys_master_unlock: crate::Reg<sys_master_unlock::SYS_MASTER_UNLOCK_SPEC>,
    #[doc = "0x1004..0x100c - Boot Override Request Register"]
    pub sys_bootover_req: [crate::Reg<sys_bootover_req::SYS_BOOTOVER_REQ_SPEC>; 2],
    #[doc = "0x100c - Boot Override Acknowledge Register"]
    pub sys_bootover_ack: crate::Reg<sys_bootover_ack::SYS_BOOTOVER_ACK_SPEC>,
    #[doc = "0x1010 - Reset Request Register"]
    pub sys_reset_req: crate::Reg<sys_reset_req::SYS_RESET_REQ_SPEC>,
    #[doc = "0x1014 - Reset Status and Override Register"]
    pub sys_reset_statover: crate::Reg<sys_reset_statover::SYS_RESET_STATOVER_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x1020 - System Status Register"]
    pub sys_system_stat: crate::Reg<sys_system_stat::SYS_SYSTEM_STAT_SPEC>,
}
#[doc = "SYS_REBOOT_CTL register accessor: an alias for `Reg<SYS_REBOOT_CTL_SPEC>`"]
pub type SYS_REBOOT_CTL = crate::Reg<sys_reboot_ctl::SYS_REBOOT_CTL_SPEC>;
#[doc = "Reboot Control Register"]
pub mod sys_reboot_ctl;
#[doc = "SYS_NMI_CTLSTAT register accessor: an alias for `Reg<SYS_NMI_CTLSTAT_SPEC>`"]
pub type SYS_NMI_CTLSTAT = crate::Reg<sys_nmi_ctlstat::SYS_NMI_CTLSTAT_SPEC>;
#[doc = "NMI Control and Status Register"]
pub mod sys_nmi_ctlstat;
#[doc = "SYS_WDTRESET_CTL register accessor: an alias for `Reg<SYS_WDTRESET_CTL_SPEC>`"]
pub type SYS_WDTRESET_CTL = crate::Reg<sys_wdtreset_ctl::SYS_WDTRESET_CTL_SPEC>;
#[doc = "Watchdog Reset Control Register"]
pub mod sys_wdtreset_ctl;
#[doc = "SYS_PERIHALT_CTL register accessor: an alias for `Reg<SYS_PERIHALT_CTL_SPEC>`"]
pub type SYS_PERIHALT_CTL = crate::Reg<sys_perihalt_ctl::SYS_PERIHALT_CTL_SPEC>;
#[doc = "Peripheral Halt Control Register"]
pub mod sys_perihalt_ctl;
#[doc = "SYS_SRAM_SIZE register accessor: an alias for `Reg<SYS_SRAM_SIZE_SPEC>`"]
pub type SYS_SRAM_SIZE = crate::Reg<sys_sram_size::SYS_SRAM_SIZE_SPEC>;
#[doc = "SRAM Size Register"]
pub mod sys_sram_size;
#[doc = "SYS_SRAM_BANKEN register accessor: an alias for `Reg<SYS_SRAM_BANKEN_SPEC>`"]
pub type SYS_SRAM_BANKEN = crate::Reg<sys_sram_banken::SYS_SRAM_BANKEN_SPEC>;
#[doc = "SRAM Bank Enable Register"]
pub mod sys_sram_banken;
#[doc = "SYS_SRAM_BANKRET register accessor: an alias for `Reg<SYS_SRAM_BANKRET_SPEC>`"]
pub type SYS_SRAM_BANKRET = crate::Reg<sys_sram_bankret::SYS_SRAM_BANKRET_SPEC>;
#[doc = "SRAM Bank Retention Control Register"]
pub mod sys_sram_bankret;
#[doc = "SYS_FLASH_SIZE register accessor: an alias for `Reg<SYS_FLASH_SIZE_SPEC>`"]
pub type SYS_FLASH_SIZE = crate::Reg<sys_flash_size::SYS_FLASH_SIZE_SPEC>;
#[doc = "Flash Size Register"]
pub mod sys_flash_size;
#[doc = "SYS_DIO_GLTFLT_CTL register accessor: an alias for `Reg<SYS_DIO_GLTFLT_CTL_SPEC>`"]
pub type SYS_DIO_GLTFLT_CTL = crate::Reg<sys_dio_gltflt_ctl::SYS_DIO_GLTFLT_CTL_SPEC>;
#[doc = "Digital I/O Glitch Filter Control Register"]
pub mod sys_dio_gltflt_ctl;
#[doc = "SYS_SECDATA_UNLOCK register accessor: an alias for `Reg<SYS_SECDATA_UNLOCK_SPEC>`"]
pub type SYS_SECDATA_UNLOCK = crate::Reg<sys_secdata_unlock::SYS_SECDATA_UNLOCK_SPEC>;
#[doc = "IP Protected Secure Zone Data Access Unlock Register"]
pub mod sys_secdata_unlock;
#[doc = "SYS_MASTER_UNLOCK register accessor: an alias for `Reg<SYS_MASTER_UNLOCK_SPEC>`"]
pub type SYS_MASTER_UNLOCK = crate::Reg<sys_master_unlock::SYS_MASTER_UNLOCK_SPEC>;
#[doc = "Master Unlock Register"]
pub mod sys_master_unlock;
#[doc = "SYS_BOOTOVER_REQ register accessor: an alias for `Reg<SYS_BOOTOVER_REQ_SPEC>`"]
pub type SYS_BOOTOVER_REQ = crate::Reg<sys_bootover_req::SYS_BOOTOVER_REQ_SPEC>;
#[doc = "Boot Override Request Register"]
pub mod sys_bootover_req;
#[doc = "SYS_BOOTOVER_ACK register accessor: an alias for `Reg<SYS_BOOTOVER_ACK_SPEC>`"]
pub type SYS_BOOTOVER_ACK = crate::Reg<sys_bootover_ack::SYS_BOOTOVER_ACK_SPEC>;
#[doc = "Boot Override Acknowledge Register"]
pub mod sys_bootover_ack;
#[doc = "SYS_RESET_REQ register accessor: an alias for `Reg<SYS_RESET_REQ_SPEC>`"]
pub type SYS_RESET_REQ = crate::Reg<sys_reset_req::SYS_RESET_REQ_SPEC>;
#[doc = "Reset Request Register"]
pub mod sys_reset_req;
#[doc = "SYS_RESET_STATOVER register accessor: an alias for `Reg<SYS_RESET_STATOVER_SPEC>`"]
pub type SYS_RESET_STATOVER = crate::Reg<sys_reset_statover::SYS_RESET_STATOVER_SPEC>;
#[doc = "Reset Status and Override Register"]
pub mod sys_reset_statover;
#[doc = "SYS_SYSTEM_STAT register accessor: an alias for `Reg<SYS_SYSTEM_STAT_SPEC>`"]
pub type SYS_SYSTEM_STAT = crate::Reg<sys_system_stat::SYS_SYSTEM_STAT_SPEC>;
#[doc = "System Status Register"]
pub mod sys_system_stat;
