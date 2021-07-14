#[doc = "Register `RSTCTL_PCMRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_PCMRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_PCMRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_PCMRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_PCMRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPM35` reader - Indicates if POR was caused by PCM due to an exit from LPM3.5"]
pub struct LPM35_R(crate::FieldReader<bool, bool>);
impl LPM35_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM45` reader - Indicates if POR was caused by PCM due to an exit from LPM4.5"]
pub struct LPM45_R(crate::FieldReader<bool, bool>);
impl LPM45_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Indicates if POR was caused by PCM due to an exit from LPM3.5"]
    #[inline(always)]
    pub fn lpm35(&self) -> LPM35_R {
        LPM35_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if POR was caused by PCM due to an exit from LPM4.5"]
    #[inline(always)]
    pub fn lpm45(&self) -> LPM45_R {
        LPM45_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "PCM Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_pcmreset_stat](index.html) module"]
pub struct RSTCTL_PCMRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_PCMRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_pcmreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_PCMRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_PCMRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_PCMRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
