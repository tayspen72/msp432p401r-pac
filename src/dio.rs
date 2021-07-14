#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port A Input"]
    pub pain: crate::Reg<pain::PAIN_SPEC>,
    #[doc = "0x02 - Port A Output"]
    pub paout: crate::Reg<paout::PAOUT_SPEC>,
    #[doc = "0x04 - Port A Direction"]
    pub padir: crate::Reg<padir::PADIR_SPEC>,
    #[doc = "0x06 - Port A Resistor Enable"]
    pub paren: crate::Reg<paren::PAREN_SPEC>,
    #[doc = "0x08 - Port A Drive Strength"]
    pub pads: crate::Reg<pads::PADS_SPEC>,
    #[doc = "0x0a - Port A Select 0"]
    pub pasel0: crate::Reg<pasel0::PASEL0_SPEC>,
    #[doc = "0x0c - Port A Select 1"]
    pub pasel1: crate::Reg<pasel1::PASEL1_SPEC>,
    #[doc = "0x0e - Port 1 Interrupt Vector Register"]
    pub p1iv: crate::Reg<p1iv::P1IV_SPEC>,
    _reserved8: [u8; 0x06],
    #[doc = "0x16 - Port A Complement Select"]
    pub paselc: crate::Reg<paselc::PASELC_SPEC>,
    #[doc = "0x18 - Port A Interrupt Edge Select"]
    pub paies: crate::Reg<paies::PAIES_SPEC>,
    #[doc = "0x1a - Port A Interrupt Enable"]
    pub paie: crate::Reg<paie::PAIE_SPEC>,
    #[doc = "0x1c - Port A Interrupt Flag"]
    pub paifg: crate::Reg<paifg::PAIFG_SPEC>,
    #[doc = "0x1e - Port 2 Interrupt Vector Register"]
    pub p2iv: crate::Reg<p2iv::P2IV_SPEC>,
    #[doc = "0x20 - Port B Input"]
    pub pbin: crate::Reg<pbin::PBIN_SPEC>,
    #[doc = "0x22 - Port B Output"]
    pub pbout: crate::Reg<pbout::PBOUT_SPEC>,
    #[doc = "0x24 - Port B Direction"]
    pub pbdir: crate::Reg<pbdir::PBDIR_SPEC>,
    #[doc = "0x26 - Port B Resistor Enable"]
    pub pbren: crate::Reg<pbren::PBREN_SPEC>,
    #[doc = "0x28 - Port B Drive Strength"]
    pub pbds: crate::Reg<pbds::PBDS_SPEC>,
    #[doc = "0x2a - Port B Select 0"]
    pub pbsel0: crate::Reg<pbsel0::PBSEL0_SPEC>,
    #[doc = "0x2c - Port B Select 1"]
    pub pbsel1: crate::Reg<pbsel1::PBSEL1_SPEC>,
    #[doc = "0x2e - Port 3 Interrupt Vector Register"]
    pub p3iv: crate::Reg<p3iv::P3IV_SPEC>,
    _reserved21: [u8; 0x06],
    #[doc = "0x36 - Port B Complement Select"]
    pub pbselc: crate::Reg<pbselc::PBSELC_SPEC>,
    #[doc = "0x38 - Port B Interrupt Edge Select"]
    pub pbies: crate::Reg<pbies::PBIES_SPEC>,
    #[doc = "0x3a - Port B Interrupt Enable"]
    pub pbie: crate::Reg<pbie::PBIE_SPEC>,
    #[doc = "0x3c - Port B Interrupt Flag"]
    pub pbifg: crate::Reg<pbifg::PBIFG_SPEC>,
    #[doc = "0x3e - Port 4 Interrupt Vector Register"]
    pub p4iv: crate::Reg<p4iv::P4IV_SPEC>,
    #[doc = "0x40 - Port C Input"]
    pub pcin: crate::Reg<pcin::PCIN_SPEC>,
    #[doc = "0x42 - Port C Output"]
    pub pcout: crate::Reg<pcout::PCOUT_SPEC>,
    #[doc = "0x44 - Port C Direction"]
    pub pcdir: crate::Reg<pcdir::PCDIR_SPEC>,
    #[doc = "0x46 - Port C Resistor Enable"]
    pub pcren: crate::Reg<pcren::PCREN_SPEC>,
    #[doc = "0x48 - Port C Drive Strength"]
    pub pcds: crate::Reg<pcds::PCDS_SPEC>,
    #[doc = "0x4a - Port C Select 0"]
    pub pcsel0: crate::Reg<pcsel0::PCSEL0_SPEC>,
    #[doc = "0x4c - Port C Select 1"]
    pub pcsel1: crate::Reg<pcsel1::PCSEL1_SPEC>,
    #[doc = "0x4e - Port 5 Interrupt Vector Register"]
    pub p5iv: crate::Reg<p5iv::P5IV_SPEC>,
    _reserved34: [u8; 0x06],
    #[doc = "0x56 - Port C Complement Select"]
    pub pcselc: crate::Reg<pcselc::PCSELC_SPEC>,
    #[doc = "0x58 - Port C Interrupt Edge Select"]
    pub pcies: crate::Reg<pcies::PCIES_SPEC>,
    #[doc = "0x5a - Port C Interrupt Enable"]
    pub pcie: crate::Reg<pcie::PCIE_SPEC>,
    #[doc = "0x5c - Port C Interrupt Flag"]
    pub pcifg: crate::Reg<pcifg::PCIFG_SPEC>,
    #[doc = "0x5e - Port 6 Interrupt Vector Register"]
    pub p6iv: crate::Reg<p6iv::P6IV_SPEC>,
    #[doc = "0x60 - Port D Input"]
    pub pdin: crate::Reg<pdin::PDIN_SPEC>,
    #[doc = "0x62 - Port D Output"]
    pub pdout: crate::Reg<pdout::PDOUT_SPEC>,
    #[doc = "0x64 - Port D Direction"]
    pub pddir: crate::Reg<pddir::PDDIR_SPEC>,
    #[doc = "0x66 - Port D Resistor Enable"]
    pub pdren: crate::Reg<pdren::PDREN_SPEC>,
    #[doc = "0x68 - Port D Drive Strength"]
    pub pdds: crate::Reg<pdds::PDDS_SPEC>,
    #[doc = "0x6a - Port D Select 0"]
    pub pdsel0: crate::Reg<pdsel0::PDSEL0_SPEC>,
    #[doc = "0x6c - Port D Select 1"]
    pub pdsel1: crate::Reg<pdsel1::PDSEL1_SPEC>,
    #[doc = "0x6e - Port 7 Interrupt Vector Register"]
    pub p7iv: crate::Reg<p7iv::P7IV_SPEC>,
    _reserved47: [u8; 0x06],
    #[doc = "0x76 - Port D Complement Select"]
    pub pdselc: crate::Reg<pdselc::PDSELC_SPEC>,
    #[doc = "0x78 - Port D Interrupt Edge Select"]
    pub pdies: crate::Reg<pdies::PDIES_SPEC>,
    #[doc = "0x7a - Port D Interrupt Enable"]
    pub pdie: crate::Reg<pdie::PDIE_SPEC>,
    #[doc = "0x7c - Port D Interrupt Flag"]
    pub pdifg: crate::Reg<pdifg::PDIFG_SPEC>,
    #[doc = "0x7e - Port 8 Interrupt Vector Register"]
    pub p8iv: crate::Reg<p8iv::P8IV_SPEC>,
    #[doc = "0x80 - Port E Input"]
    pub pein: crate::Reg<pein::PEIN_SPEC>,
    #[doc = "0x82 - Port E Output"]
    pub peout: crate::Reg<peout::PEOUT_SPEC>,
    #[doc = "0x84 - Port E Direction"]
    pub pedir: crate::Reg<pedir::PEDIR_SPEC>,
    #[doc = "0x86 - Port E Resistor Enable"]
    pub peren: crate::Reg<peren::PEREN_SPEC>,
    #[doc = "0x88 - Port E Drive Strength"]
    pub peds: crate::Reg<peds::PEDS_SPEC>,
    #[doc = "0x8a - Port E Select 0"]
    pub pesel0: crate::Reg<pesel0::PESEL0_SPEC>,
    #[doc = "0x8c - Port E Select 1"]
    pub pesel1: crate::Reg<pesel1::PESEL1_SPEC>,
    #[doc = "0x8e - Port 9 Interrupt Vector Register"]
    pub p9iv: crate::Reg<p9iv::P9IV_SPEC>,
    _reserved60: [u8; 0x06],
    #[doc = "0x96 - Port E Complement Select"]
    pub peselc: crate::Reg<peselc::PESELC_SPEC>,
    #[doc = "0x98 - Port E Interrupt Edge Select"]
    pub peies: crate::Reg<peies::PEIES_SPEC>,
    #[doc = "0x9a - Port E Interrupt Enable"]
    pub peie: crate::Reg<peie::PEIE_SPEC>,
    #[doc = "0x9c - Port E Interrupt Flag"]
    pub peifg: crate::Reg<peifg::PEIFG_SPEC>,
    #[doc = "0x9e - Port 10 Interrupt Vector Register"]
    pub p10iv: crate::Reg<p10iv::P10IV_SPEC>,
    _reserved65: [u8; 0x80],
    #[doc = "0x120 - Port J Input"]
    pub pjin: crate::Reg<pjin::PJIN_SPEC>,
    #[doc = "0x122 - Port J Output"]
    pub pjout: crate::Reg<pjout::PJOUT_SPEC>,
    #[doc = "0x124 - Port J Direction"]
    pub pjdir: crate::Reg<pjdir::PJDIR_SPEC>,
    #[doc = "0x126 - Port J Resistor Enable"]
    pub pjren: crate::Reg<pjren::PJREN_SPEC>,
    #[doc = "0x128 - Port J Drive Strength"]
    pub pjds: crate::Reg<pjds::PJDS_SPEC>,
    #[doc = "0x12a - Port J Select 0"]
    pub pjsel0: crate::Reg<pjsel0::PJSEL0_SPEC>,
    #[doc = "0x12c - Port J Select 1"]
    pub pjsel1: crate::Reg<pjsel1::PJSEL1_SPEC>,
    _reserved72: [u8; 0x08],
    #[doc = "0x136 - Port J Complement Select"]
    pub pjselc: crate::Reg<pjselc::PJSELC_SPEC>,
}
#[doc = "PAIN register accessor: an alias for `Reg<PAIN_SPEC>`"]
pub type PAIN = crate::Reg<pain::PAIN_SPEC>;
#[doc = "Port A Input"]
pub mod pain;
#[doc = "PAOUT register accessor: an alias for `Reg<PAOUT_SPEC>`"]
pub type PAOUT = crate::Reg<paout::PAOUT_SPEC>;
#[doc = "Port A Output"]
pub mod paout;
#[doc = "PADIR register accessor: an alias for `Reg<PADIR_SPEC>`"]
pub type PADIR = crate::Reg<padir::PADIR_SPEC>;
#[doc = "Port A Direction"]
pub mod padir;
#[doc = "PAREN register accessor: an alias for `Reg<PAREN_SPEC>`"]
pub type PAREN = crate::Reg<paren::PAREN_SPEC>;
#[doc = "Port A Resistor Enable"]
pub mod paren;
#[doc = "PADS register accessor: an alias for `Reg<PADS_SPEC>`"]
pub type PADS = crate::Reg<pads::PADS_SPEC>;
#[doc = "Port A Drive Strength"]
pub mod pads;
#[doc = "PASEL0 register accessor: an alias for `Reg<PASEL0_SPEC>`"]
pub type PASEL0 = crate::Reg<pasel0::PASEL0_SPEC>;
#[doc = "Port A Select 0"]
pub mod pasel0;
#[doc = "PASEL1 register accessor: an alias for `Reg<PASEL1_SPEC>`"]
pub type PASEL1 = crate::Reg<pasel1::PASEL1_SPEC>;
#[doc = "Port A Select 1"]
pub mod pasel1;
#[doc = "P1IV register accessor: an alias for `Reg<P1IV_SPEC>`"]
pub type P1IV = crate::Reg<p1iv::P1IV_SPEC>;
#[doc = "Port 1 Interrupt Vector Register"]
pub mod p1iv;
#[doc = "PASELC register accessor: an alias for `Reg<PASELC_SPEC>`"]
pub type PASELC = crate::Reg<paselc::PASELC_SPEC>;
#[doc = "Port A Complement Select"]
pub mod paselc;
#[doc = "PAIES register accessor: an alias for `Reg<PAIES_SPEC>`"]
pub type PAIES = crate::Reg<paies::PAIES_SPEC>;
#[doc = "Port A Interrupt Edge Select"]
pub mod paies;
#[doc = "PAIE register accessor: an alias for `Reg<PAIE_SPEC>`"]
pub type PAIE = crate::Reg<paie::PAIE_SPEC>;
#[doc = "Port A Interrupt Enable"]
pub mod paie;
#[doc = "PAIFG register accessor: an alias for `Reg<PAIFG_SPEC>`"]
pub type PAIFG = crate::Reg<paifg::PAIFG_SPEC>;
#[doc = "Port A Interrupt Flag"]
pub mod paifg;
#[doc = "P2IV register accessor: an alias for `Reg<P2IV_SPEC>`"]
pub type P2IV = crate::Reg<p2iv::P2IV_SPEC>;
#[doc = "Port 2 Interrupt Vector Register"]
pub mod p2iv;
#[doc = "PBIN register accessor: an alias for `Reg<PBIN_SPEC>`"]
pub type PBIN = crate::Reg<pbin::PBIN_SPEC>;
#[doc = "Port B Input"]
pub mod pbin;
#[doc = "PBOUT register accessor: an alias for `Reg<PBOUT_SPEC>`"]
pub type PBOUT = crate::Reg<pbout::PBOUT_SPEC>;
#[doc = "Port B Output"]
pub mod pbout;
#[doc = "PBDIR register accessor: an alias for `Reg<PBDIR_SPEC>`"]
pub type PBDIR = crate::Reg<pbdir::PBDIR_SPEC>;
#[doc = "Port B Direction"]
pub mod pbdir;
#[doc = "PBREN register accessor: an alias for `Reg<PBREN_SPEC>`"]
pub type PBREN = crate::Reg<pbren::PBREN_SPEC>;
#[doc = "Port B Resistor Enable"]
pub mod pbren;
#[doc = "PBDS register accessor: an alias for `Reg<PBDS_SPEC>`"]
pub type PBDS = crate::Reg<pbds::PBDS_SPEC>;
#[doc = "Port B Drive Strength"]
pub mod pbds;
#[doc = "PBSEL0 register accessor: an alias for `Reg<PBSEL0_SPEC>`"]
pub type PBSEL0 = crate::Reg<pbsel0::PBSEL0_SPEC>;
#[doc = "Port B Select 0"]
pub mod pbsel0;
#[doc = "PBSEL1 register accessor: an alias for `Reg<PBSEL1_SPEC>`"]
pub type PBSEL1 = crate::Reg<pbsel1::PBSEL1_SPEC>;
#[doc = "Port B Select 1"]
pub mod pbsel1;
#[doc = "P3IV register accessor: an alias for `Reg<P3IV_SPEC>`"]
pub type P3IV = crate::Reg<p3iv::P3IV_SPEC>;
#[doc = "Port 3 Interrupt Vector Register"]
pub mod p3iv;
#[doc = "PBSELC register accessor: an alias for `Reg<PBSELC_SPEC>`"]
pub type PBSELC = crate::Reg<pbselc::PBSELC_SPEC>;
#[doc = "Port B Complement Select"]
pub mod pbselc;
#[doc = "PBIES register accessor: an alias for `Reg<PBIES_SPEC>`"]
pub type PBIES = crate::Reg<pbies::PBIES_SPEC>;
#[doc = "Port B Interrupt Edge Select"]
pub mod pbies;
#[doc = "PBIE register accessor: an alias for `Reg<PBIE_SPEC>`"]
pub type PBIE = crate::Reg<pbie::PBIE_SPEC>;
#[doc = "Port B Interrupt Enable"]
pub mod pbie;
#[doc = "PBIFG register accessor: an alias for `Reg<PBIFG_SPEC>`"]
pub type PBIFG = crate::Reg<pbifg::PBIFG_SPEC>;
#[doc = "Port B Interrupt Flag"]
pub mod pbifg;
#[doc = "P4IV register accessor: an alias for `Reg<P4IV_SPEC>`"]
pub type P4IV = crate::Reg<p4iv::P4IV_SPEC>;
#[doc = "Port 4 Interrupt Vector Register"]
pub mod p4iv;
#[doc = "PCIN register accessor: an alias for `Reg<PCIN_SPEC>`"]
pub type PCIN = crate::Reg<pcin::PCIN_SPEC>;
#[doc = "Port C Input"]
pub mod pcin;
#[doc = "PCOUT register accessor: an alias for `Reg<PCOUT_SPEC>`"]
pub type PCOUT = crate::Reg<pcout::PCOUT_SPEC>;
#[doc = "Port C Output"]
pub mod pcout;
#[doc = "PCDIR register accessor: an alias for `Reg<PCDIR_SPEC>`"]
pub type PCDIR = crate::Reg<pcdir::PCDIR_SPEC>;
#[doc = "Port C Direction"]
pub mod pcdir;
#[doc = "PCREN register accessor: an alias for `Reg<PCREN_SPEC>`"]
pub type PCREN = crate::Reg<pcren::PCREN_SPEC>;
#[doc = "Port C Resistor Enable"]
pub mod pcren;
#[doc = "PCDS register accessor: an alias for `Reg<PCDS_SPEC>`"]
pub type PCDS = crate::Reg<pcds::PCDS_SPEC>;
#[doc = "Port C Drive Strength"]
pub mod pcds;
#[doc = "PCSEL0 register accessor: an alias for `Reg<PCSEL0_SPEC>`"]
pub type PCSEL0 = crate::Reg<pcsel0::PCSEL0_SPEC>;
#[doc = "Port C Select 0"]
pub mod pcsel0;
#[doc = "PCSEL1 register accessor: an alias for `Reg<PCSEL1_SPEC>`"]
pub type PCSEL1 = crate::Reg<pcsel1::PCSEL1_SPEC>;
#[doc = "Port C Select 1"]
pub mod pcsel1;
#[doc = "P5IV register accessor: an alias for `Reg<P5IV_SPEC>`"]
pub type P5IV = crate::Reg<p5iv::P5IV_SPEC>;
#[doc = "Port 5 Interrupt Vector Register"]
pub mod p5iv;
#[doc = "PCSELC register accessor: an alias for `Reg<PCSELC_SPEC>`"]
pub type PCSELC = crate::Reg<pcselc::PCSELC_SPEC>;
#[doc = "Port C Complement Select"]
pub mod pcselc;
#[doc = "PCIES register accessor: an alias for `Reg<PCIES_SPEC>`"]
pub type PCIES = crate::Reg<pcies::PCIES_SPEC>;
#[doc = "Port C Interrupt Edge Select"]
pub mod pcies;
#[doc = "PCIE register accessor: an alias for `Reg<PCIE_SPEC>`"]
pub type PCIE = crate::Reg<pcie::PCIE_SPEC>;
#[doc = "Port C Interrupt Enable"]
pub mod pcie;
#[doc = "PCIFG register accessor: an alias for `Reg<PCIFG_SPEC>`"]
pub type PCIFG = crate::Reg<pcifg::PCIFG_SPEC>;
#[doc = "Port C Interrupt Flag"]
pub mod pcifg;
#[doc = "P6IV register accessor: an alias for `Reg<P6IV_SPEC>`"]
pub type P6IV = crate::Reg<p6iv::P6IV_SPEC>;
#[doc = "Port 6 Interrupt Vector Register"]
pub mod p6iv;
#[doc = "PDIN register accessor: an alias for `Reg<PDIN_SPEC>`"]
pub type PDIN = crate::Reg<pdin::PDIN_SPEC>;
#[doc = "Port D Input"]
pub mod pdin;
#[doc = "PDOUT register accessor: an alias for `Reg<PDOUT_SPEC>`"]
pub type PDOUT = crate::Reg<pdout::PDOUT_SPEC>;
#[doc = "Port D Output"]
pub mod pdout;
#[doc = "PDDIR register accessor: an alias for `Reg<PDDIR_SPEC>`"]
pub type PDDIR = crate::Reg<pddir::PDDIR_SPEC>;
#[doc = "Port D Direction"]
pub mod pddir;
#[doc = "PDREN register accessor: an alias for `Reg<PDREN_SPEC>`"]
pub type PDREN = crate::Reg<pdren::PDREN_SPEC>;
#[doc = "Port D Resistor Enable"]
pub mod pdren;
#[doc = "PDDS register accessor: an alias for `Reg<PDDS_SPEC>`"]
pub type PDDS = crate::Reg<pdds::PDDS_SPEC>;
#[doc = "Port D Drive Strength"]
pub mod pdds;
#[doc = "PDSEL0 register accessor: an alias for `Reg<PDSEL0_SPEC>`"]
pub type PDSEL0 = crate::Reg<pdsel0::PDSEL0_SPEC>;
#[doc = "Port D Select 0"]
pub mod pdsel0;
#[doc = "PDSEL1 register accessor: an alias for `Reg<PDSEL1_SPEC>`"]
pub type PDSEL1 = crate::Reg<pdsel1::PDSEL1_SPEC>;
#[doc = "Port D Select 1"]
pub mod pdsel1;
#[doc = "P7IV register accessor: an alias for `Reg<P7IV_SPEC>`"]
pub type P7IV = crate::Reg<p7iv::P7IV_SPEC>;
#[doc = "Port 7 Interrupt Vector Register"]
pub mod p7iv;
#[doc = "PDSELC register accessor: an alias for `Reg<PDSELC_SPEC>`"]
pub type PDSELC = crate::Reg<pdselc::PDSELC_SPEC>;
#[doc = "Port D Complement Select"]
pub mod pdselc;
#[doc = "PDIES register accessor: an alias for `Reg<PDIES_SPEC>`"]
pub type PDIES = crate::Reg<pdies::PDIES_SPEC>;
#[doc = "Port D Interrupt Edge Select"]
pub mod pdies;
#[doc = "PDIE register accessor: an alias for `Reg<PDIE_SPEC>`"]
pub type PDIE = crate::Reg<pdie::PDIE_SPEC>;
#[doc = "Port D Interrupt Enable"]
pub mod pdie;
#[doc = "PDIFG register accessor: an alias for `Reg<PDIFG_SPEC>`"]
pub type PDIFG = crate::Reg<pdifg::PDIFG_SPEC>;
#[doc = "Port D Interrupt Flag"]
pub mod pdifg;
#[doc = "P8IV register accessor: an alias for `Reg<P8IV_SPEC>`"]
pub type P8IV = crate::Reg<p8iv::P8IV_SPEC>;
#[doc = "Port 8 Interrupt Vector Register"]
pub mod p8iv;
#[doc = "PEIN register accessor: an alias for `Reg<PEIN_SPEC>`"]
pub type PEIN = crate::Reg<pein::PEIN_SPEC>;
#[doc = "Port E Input"]
pub mod pein;
#[doc = "PEOUT register accessor: an alias for `Reg<PEOUT_SPEC>`"]
pub type PEOUT = crate::Reg<peout::PEOUT_SPEC>;
#[doc = "Port E Output"]
pub mod peout;
#[doc = "PEDIR register accessor: an alias for `Reg<PEDIR_SPEC>`"]
pub type PEDIR = crate::Reg<pedir::PEDIR_SPEC>;
#[doc = "Port E Direction"]
pub mod pedir;
#[doc = "PEREN register accessor: an alias for `Reg<PEREN_SPEC>`"]
pub type PEREN = crate::Reg<peren::PEREN_SPEC>;
#[doc = "Port E Resistor Enable"]
pub mod peren;
#[doc = "PEDS register accessor: an alias for `Reg<PEDS_SPEC>`"]
pub type PEDS = crate::Reg<peds::PEDS_SPEC>;
#[doc = "Port E Drive Strength"]
pub mod peds;
#[doc = "PESEL0 register accessor: an alias for `Reg<PESEL0_SPEC>`"]
pub type PESEL0 = crate::Reg<pesel0::PESEL0_SPEC>;
#[doc = "Port E Select 0"]
pub mod pesel0;
#[doc = "PESEL1 register accessor: an alias for `Reg<PESEL1_SPEC>`"]
pub type PESEL1 = crate::Reg<pesel1::PESEL1_SPEC>;
#[doc = "Port E Select 1"]
pub mod pesel1;
#[doc = "P9IV register accessor: an alias for `Reg<P9IV_SPEC>`"]
pub type P9IV = crate::Reg<p9iv::P9IV_SPEC>;
#[doc = "Port 9 Interrupt Vector Register"]
pub mod p9iv;
#[doc = "PESELC register accessor: an alias for `Reg<PESELC_SPEC>`"]
pub type PESELC = crate::Reg<peselc::PESELC_SPEC>;
#[doc = "Port E Complement Select"]
pub mod peselc;
#[doc = "PEIES register accessor: an alias for `Reg<PEIES_SPEC>`"]
pub type PEIES = crate::Reg<peies::PEIES_SPEC>;
#[doc = "Port E Interrupt Edge Select"]
pub mod peies;
#[doc = "PEIE register accessor: an alias for `Reg<PEIE_SPEC>`"]
pub type PEIE = crate::Reg<peie::PEIE_SPEC>;
#[doc = "Port E Interrupt Enable"]
pub mod peie;
#[doc = "PEIFG register accessor: an alias for `Reg<PEIFG_SPEC>`"]
pub type PEIFG = crate::Reg<peifg::PEIFG_SPEC>;
#[doc = "Port E Interrupt Flag"]
pub mod peifg;
#[doc = "P10IV register accessor: an alias for `Reg<P10IV_SPEC>`"]
pub type P10IV = crate::Reg<p10iv::P10IV_SPEC>;
#[doc = "Port 10 Interrupt Vector Register"]
pub mod p10iv;
#[doc = "PJIN register accessor: an alias for `Reg<PJIN_SPEC>`"]
pub type PJIN = crate::Reg<pjin::PJIN_SPEC>;
#[doc = "Port J Input"]
pub mod pjin;
#[doc = "PJOUT register accessor: an alias for `Reg<PJOUT_SPEC>`"]
pub type PJOUT = crate::Reg<pjout::PJOUT_SPEC>;
#[doc = "Port J Output"]
pub mod pjout;
#[doc = "PJDIR register accessor: an alias for `Reg<PJDIR_SPEC>`"]
pub type PJDIR = crate::Reg<pjdir::PJDIR_SPEC>;
#[doc = "Port J Direction"]
pub mod pjdir;
#[doc = "PJREN register accessor: an alias for `Reg<PJREN_SPEC>`"]
pub type PJREN = crate::Reg<pjren::PJREN_SPEC>;
#[doc = "Port J Resistor Enable"]
pub mod pjren;
#[doc = "PJDS register accessor: an alias for `Reg<PJDS_SPEC>`"]
pub type PJDS = crate::Reg<pjds::PJDS_SPEC>;
#[doc = "Port J Drive Strength"]
pub mod pjds;
#[doc = "PJSEL0 register accessor: an alias for `Reg<PJSEL0_SPEC>`"]
pub type PJSEL0 = crate::Reg<pjsel0::PJSEL0_SPEC>;
#[doc = "Port J Select 0"]
pub mod pjsel0;
#[doc = "PJSEL1 register accessor: an alias for `Reg<PJSEL1_SPEC>`"]
pub type PJSEL1 = crate::Reg<pjsel1::PJSEL1_SPEC>;
#[doc = "Port J Select 1"]
pub mod pjsel1;
#[doc = "PJSELC register accessor: an alias for `Reg<PJSELC_SPEC>`"]
pub type PJSELC = crate::Reg<pjselc::PJSELC_SPEC>;
#[doc = "Port J Complement Select"]
pub mod pjselc;
