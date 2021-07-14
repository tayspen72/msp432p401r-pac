#[doc = "Register `SYS_SYSTEM_STAT` reader"]
pub struct R(crate::R<SYS_SYSTEM_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SYSTEM_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SYSTEM_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SYSTEM_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBG_SEC_ACT` reader - Debug Security active"]
pub struct DBG_SEC_ACT_R(crate::FieldReader<bool, bool>);
impl DBG_SEC_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DBG_SEC_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DBG_SEC_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `JTAG_SWD_LOCK_ACT` reader - Indicates if JTAG and SWD Lock is active"]
pub struct JTAG_SWD_LOCK_ACT_R(crate::FieldReader<bool, bool>);
impl JTAG_SWD_LOCK_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        JTAG_SWD_LOCK_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for JTAG_SWD_LOCK_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IP_PROT_ACT` reader - Indicates if IP protection is active"]
pub struct IP_PROT_ACT_R(crate::FieldReader<bool, bool>);
impl IP_PROT_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IP_PROT_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IP_PROT_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 3 - Debug Security active"]
    #[inline(always)]
    pub fn dbg_sec_act(&self) -> DBG_SEC_ACT_R {
        DBG_SEC_ACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates if JTAG and SWD Lock is active"]
    #[inline(always)]
    pub fn jtag_swd_lock_act(&self) -> JTAG_SWD_LOCK_ACT_R {
        JTAG_SWD_LOCK_ACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates if IP protection is active"]
    #[inline(always)]
    pub fn ip_prot_act(&self) -> IP_PROT_ACT_R {
        IP_PROT_ACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_system_stat](index.html) module"]
pub struct SYS_SYSTEM_STAT_SPEC;
impl crate::RegisterSpec for SYS_SYSTEM_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_system_stat::R](R) reader structure"]
impl crate::Readable for SYS_SYSTEM_STAT_SPEC {
    type Reader = R;
}
