#[doc = "Register `PCMCLRIFG` writer"]
pub struct W(crate::W<PCMCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMCLRIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PCMCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLR_LPM_INVALID_TR_IFG` writer - Clear LPM invalid transition flag"]
pub struct CLR_LPM_INVALID_TR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LPM_INVALID_TR_IFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CLR_LPM_INVALID_CLK_IFG` writer - Clear LPM invalid clock flag"]
pub struct CLR_LPM_INVALID_CLK_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LPM_INVALID_CLK_IFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CLR_AM_INVALID_TR_IFG` writer - Clear active mode invalid transition flag"]
pub struct CLR_AM_INVALID_TR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_AM_INVALID_TR_IFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CLR_DCDC_ERROR_IFG` writer - Clear DC-DC error flag"]
pub struct CLR_DCDC_ERROR_IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DCDC_ERROR_IFG_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear LPM invalid transition flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_tr_ifg(&mut self) -> CLR_LPM_INVALID_TR_IFG_W {
        CLR_LPM_INVALID_TR_IFG_W { w: self }
    }
    #[doc = "Bit 1 - Clear LPM invalid clock flag"]
    #[inline(always)]
    pub fn clr_lpm_invalid_clk_ifg(&mut self) -> CLR_LPM_INVALID_CLK_IFG_W {
        CLR_LPM_INVALID_CLK_IFG_W { w: self }
    }
    #[doc = "Bit 2 - Clear active mode invalid transition flag"]
    #[inline(always)]
    pub fn clr_am_invalid_tr_ifg(&mut self) -> CLR_AM_INVALID_TR_IFG_W {
        CLR_AM_INVALID_TR_IFG_W { w: self }
    }
    #[doc = "Bit 6 - Clear DC-DC error flag"]
    #[inline(always)]
    pub fn clr_dcdc_error_ifg(&mut self) -> CLR_DCDC_ERROR_IFG_W {
        CLR_DCDC_ERROR_IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmclrifg](index.html) module"]
pub struct PCMCLRIFG_SPEC;
impl crate::RegisterSpec for PCMCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcmclrifg::W](W) writer structure"]
impl crate::Writable for PCMCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCMCLRIFG to value 0"]
impl crate::Resettable for PCMCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
