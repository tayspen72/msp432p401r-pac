#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - eUSCI_Bx Control Word Register 0"]
    pub ucbx_ctlw0: crate::Reg<ucbx_ctlw0::UCBXCTLW0_SPEC>,
    #[doc = "0x02 - eUSCI_Bx Control Word Register 1"]
    pub ucbx_ctlw1: crate::Reg<ucbx_ctlw1::UCBXCTLW1_SPEC>,
    _reserved2: [u8; 0x02],
    #[doc = "0x06 - eUSCI_Bx Baud Rate Control Word Register"]
    pub ucbx_brw: crate::Reg<ucbx_brw::UCBXBRW_SPEC>,
    #[doc = "0x08 - eUSCI_Bx Status Register"]
    pub ucbx_statw: crate::Reg<ucbx_statw::UCBXSTATW_SPEC>,
    #[doc = "0x0a - eUSCI_Bx Byte Counter Threshold Register"]
    pub ucbx_tbcnt: crate::Reg<ucbx_tbcnt::UCBXTBCNT_SPEC>,
    #[doc = "0x0c - eUSCI_Bx Receive Buffer Register"]
    pub ucbx_rxbuf: crate::Reg<ucbx_rxbuf::UCBXRXBUF_SPEC>,
    #[doc = "0x0e - eUSCI_Bx Transmit Buffer Register"]
    pub ucbx_txbuf: crate::Reg<ucbx_txbuf::UCBXTXBUF_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x14 - eUSCI_Bx I2C Own Address 0 Register"]
    pub ucbx_i2coa0: crate::Reg<ucbx_i2coa0::UCBXI2COA0_SPEC>,
    #[doc = "0x16 - eUSCI_Bx I2C Own Address 1 Register"]
    pub ucbx_i2coa1: crate::Reg<ucbx_i2coa1::UCBXI2COA1_SPEC>,
    #[doc = "0x18 - eUSCI_Bx I2C Own Address 2 Register"]
    pub ucbx_i2coa2: crate::Reg<ucbx_i2coa2::UCBXI2COA2_SPEC>,
    #[doc = "0x1a - eUSCI_Bx I2C Own Address 3 Register"]
    pub ucbx_i2coa3: crate::Reg<ucbx_i2coa3::UCBXI2COA3_SPEC>,
    #[doc = "0x1c - eUSCI_Bx I2C Received Address Register"]
    pub ucbx_addrx: crate::Reg<ucbx_addrx::UCBXADDRX_SPEC>,
    #[doc = "0x1e - eUSCI_Bx I2C Address Mask Register"]
    pub ucbx_addmask: crate::Reg<ucbx_addmask::UCBXADDMASK_SPEC>,
    #[doc = "0x20 - eUSCI_Bx I2C Slave Address Register"]
    pub ucbx_i2csa: crate::Reg<ucbx_i2csa::UCBXI2CSA_SPEC>,
    _reserved14: [u8; 0x08],
    #[doc = "0x2a - eUSCI_Bx Interrupt Enable Register"]
    pub ucbx_ie: crate::Reg<ucbx_ie::UCBXIE_SPEC>,
    #[doc = "0x2c - eUSCI_Bx Interrupt Flag Register"]
    pub ucbx_ifg: crate::Reg<ucbx_ifg::UCBXIFG_SPEC>,
    #[doc = "0x2e - eUSCI_Bx Interrupt Vector Register"]
    pub ucbx_iv: crate::Reg<ucbx_iv::UCBXIV_SPEC>,
}
#[doc = "UCBxCTLW0 register accessor: an alias for `Reg<UCBXCTLW0_SPEC>`"]
pub type UCBXCTLW0 = crate::Reg<ucbx_ctlw0::UCBXCTLW0_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 0"]
pub mod ucbx_ctlw0;
#[doc = "UCBxCTLW1 register accessor: an alias for `Reg<UCBXCTLW1_SPEC>`"]
pub type UCBXCTLW1 = crate::Reg<ucbx_ctlw1::UCBXCTLW1_SPEC>;
#[doc = "eUSCI_Bx Control Word Register 1"]
pub mod ucbx_ctlw1;
#[doc = "UCBxBRW register accessor: an alias for `Reg<UCBXBRW_SPEC>`"]
pub type UCBXBRW = crate::Reg<ucbx_brw::UCBXBRW_SPEC>;
#[doc = "eUSCI_Bx Baud Rate Control Word Register"]
pub mod ucbx_brw;
#[doc = "UCBxSTATW register accessor: an alias for `Reg<UCBXSTATW_SPEC>`"]
pub type UCBXSTATW = crate::Reg<ucbx_statw::UCBXSTATW_SPEC>;
#[doc = "eUSCI_Bx Status Register"]
pub mod ucbx_statw;
#[doc = "UCBxTBCNT register accessor: an alias for `Reg<UCBXTBCNT_SPEC>`"]
pub type UCBXTBCNT = crate::Reg<ucbx_tbcnt::UCBXTBCNT_SPEC>;
#[doc = "eUSCI_Bx Byte Counter Threshold Register"]
pub mod ucbx_tbcnt;
#[doc = "UCBxRXBUF register accessor: an alias for `Reg<UCBXRXBUF_SPEC>`"]
pub type UCBXRXBUF = crate::Reg<ucbx_rxbuf::UCBXRXBUF_SPEC>;
#[doc = "eUSCI_Bx Receive Buffer Register"]
pub mod ucbx_rxbuf;
#[doc = "UCBxTXBUF register accessor: an alias for `Reg<UCBXTXBUF_SPEC>`"]
pub type UCBXTXBUF = crate::Reg<ucbx_txbuf::UCBXTXBUF_SPEC>;
#[doc = "eUSCI_Bx Transmit Buffer Register"]
pub mod ucbx_txbuf;
#[doc = "UCBxI2COA0 register accessor: an alias for `Reg<UCBXI2COA0_SPEC>`"]
pub type UCBXI2COA0 = crate::Reg<ucbx_i2coa0::UCBXI2COA0_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 0 Register"]
pub mod ucbx_i2coa0;
#[doc = "UCBxI2COA1 register accessor: an alias for `Reg<UCBXI2COA1_SPEC>`"]
pub type UCBXI2COA1 = crate::Reg<ucbx_i2coa1::UCBXI2COA1_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 1 Register"]
pub mod ucbx_i2coa1;
#[doc = "UCBxI2COA2 register accessor: an alias for `Reg<UCBXI2COA2_SPEC>`"]
pub type UCBXI2COA2 = crate::Reg<ucbx_i2coa2::UCBXI2COA2_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 2 Register"]
pub mod ucbx_i2coa2;
#[doc = "UCBxI2COA3 register accessor: an alias for `Reg<UCBXI2COA3_SPEC>`"]
pub type UCBXI2COA3 = crate::Reg<ucbx_i2coa3::UCBXI2COA3_SPEC>;
#[doc = "eUSCI_Bx I2C Own Address 3 Register"]
pub mod ucbx_i2coa3;
#[doc = "UCBxADDRX register accessor: an alias for `Reg<UCBXADDRX_SPEC>`"]
pub type UCBXADDRX = crate::Reg<ucbx_addrx::UCBXADDRX_SPEC>;
#[doc = "eUSCI_Bx I2C Received Address Register"]
pub mod ucbx_addrx;
#[doc = "UCBxADDMASK register accessor: an alias for `Reg<UCBXADDMASK_SPEC>`"]
pub type UCBXADDMASK = crate::Reg<ucbx_addmask::UCBXADDMASK_SPEC>;
#[doc = "eUSCI_Bx I2C Address Mask Register"]
pub mod ucbx_addmask;
#[doc = "UCBxI2CSA register accessor: an alias for `Reg<UCBXI2CSA_SPEC>`"]
pub type UCBXI2CSA = crate::Reg<ucbx_i2csa::UCBXI2CSA_SPEC>;
#[doc = "eUSCI_Bx I2C Slave Address Register"]
pub mod ucbx_i2csa;
#[doc = "UCBxIE register accessor: an alias for `Reg<UCBXIE_SPEC>`"]
pub type UCBXIE = crate::Reg<ucbx_ie::UCBXIE_SPEC>;
#[doc = "eUSCI_Bx Interrupt Enable Register"]
pub mod ucbx_ie;
#[doc = "UCBxIFG register accessor: an alias for `Reg<UCBXIFG_SPEC>`"]
pub type UCBXIFG = crate::Reg<ucbx_ifg::UCBXIFG_SPEC>;
#[doc = "eUSCI_Bx Interrupt Flag Register"]
pub mod ucbx_ifg;
#[doc = "UCBxIV register accessor: an alias for `Reg<UCBXIV_SPEC>`"]
pub type UCBXIV = crate::Reg<ucbx_iv::UCBXIV_SPEC>;
#[doc = "eUSCI_Bx Interrupt Vector Register"]
pub mod ucbx_iv;
