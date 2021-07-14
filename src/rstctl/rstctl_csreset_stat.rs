#[doc = "Register `RSTCTL_CSRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_CSRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_CSRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_CSRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_CSRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DCOR_SHT` reader - Indicates if POR was caused by DCO short circuit fault in the external resistor mode"]
pub struct DCOR_SHT_R(crate::FieldReader<bool, bool>);
impl DCOR_SHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCOR_SHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOR_SHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by DCO short circuit fault in the external resistor mode"]
    #[inline(always)]
    pub fn dcor_sht(&self) -> DCOR_SHT_R {
        DCOR_SHT_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "CS Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_csreset_stat](index.html) module"]
pub struct RSTCTL_CSRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_CSRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_csreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_CSRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_CSRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_CSRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
