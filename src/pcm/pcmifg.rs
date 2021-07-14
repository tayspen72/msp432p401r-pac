#[doc = "Register `PCMIFG` reader"]
pub struct R(crate::R<PCMIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPM_INVALID_TR_IFG` reader - LPM invalid transition flag"]
pub struct LPM_INVALID_TR_IFG_R(crate::FieldReader<bool, bool>);
impl LPM_INVALID_TR_IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_INVALID_TR_IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_INVALID_TR_IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_INVALID_CLK_IFG` reader - LPM invalid clock flag"]
pub struct LPM_INVALID_CLK_IFG_R(crate::FieldReader<bool, bool>);
impl LPM_INVALID_CLK_IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_INVALID_CLK_IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_INVALID_CLK_IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AM_INVALID_TR_IFG` reader - Active mode invalid transition flag"]
pub struct AM_INVALID_TR_IFG_R(crate::FieldReader<bool, bool>);
impl AM_INVALID_TR_IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        AM_INVALID_TR_IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AM_INVALID_TR_IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_ERROR_IFG` reader - DC-DC error flag"]
pub struct DCDC_ERROR_IFG_R(crate::FieldReader<bool, bool>);
impl DCDC_ERROR_IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_ERROR_IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDC_ERROR_IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - LPM invalid transition flag"]
    #[inline(always)]
    pub fn lpm_invalid_tr_ifg(&self) -> LPM_INVALID_TR_IFG_R {
        LPM_INVALID_TR_IFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LPM invalid clock flag"]
    #[inline(always)]
    pub fn lpm_invalid_clk_ifg(&self) -> LPM_INVALID_CLK_IFG_R {
        LPM_INVALID_CLK_IFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Active mode invalid transition flag"]
    #[inline(always)]
    pub fn am_invalid_tr_ifg(&self) -> AM_INVALID_TR_IFG_R {
        AM_INVALID_TR_IFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DC-DC error flag"]
    #[inline(always)]
    pub fn dcdc_error_ifg(&self) -> DCDC_ERROR_IFG_R {
        DCDC_ERROR_IFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmifg](index.html) module"]
pub struct PCMIFG_SPEC;
impl crate::RegisterSpec for PCMIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmifg::R](R) reader structure"]
impl crate::Readable for PCMIFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PCMIFG to value 0"]
impl crate::Resettable for PCMIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
