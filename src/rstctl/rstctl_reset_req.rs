#[doc = "Register `RSTCTL_RESET_REQ` reader"]
pub struct R(crate::R<RSTCTL_RESET_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_RESET_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_RESET_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_RESET_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL_RESET_REQ` writer"]
pub struct W(crate::W<RSTCTL_RESET_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_RESET_REQ_SPEC>;
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
impl From<crate::W<RSTCTL_RESET_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_RESET_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_REQ` writer - Soft Reset request"]
pub struct SOFT_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_REQ_W<'a> {
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
#[doc = "Field `HARD_REQ` writer - Hard Reset request"]
pub struct HARD_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> HARD_REQ_W<'a> {
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
#[doc = "Field `RSTKEY` writer - Write key to unlock reset request bits"]
pub struct RSTKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> RSTKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Soft Reset request"]
    #[inline(always)]
    pub fn soft_req(&mut self) -> SOFT_REQ_W {
        SOFT_REQ_W { w: self }
    }
    #[doc = "Bit 1 - Hard Reset request"]
    #[inline(always)]
    pub fn hard_req(&mut self) -> HARD_REQ_W {
        HARD_REQ_W { w: self }
    }
    #[doc = "Bits 8:15 - Write key to unlock reset request bits"]
    #[inline(always)]
    pub fn rstkey(&mut self) -> RSTKEY_W {
        RSTKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_reset_req](index.html) module"]
pub struct RSTCTL_RESET_REQ_SPEC;
impl crate::RegisterSpec for RSTCTL_RESET_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_reset_req::R](R) reader structure"]
impl crate::Readable for RSTCTL_RESET_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl_reset_req::W](W) writer structure"]
impl crate::Writable for RSTCTL_RESET_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCTL_RESET_REQ to value 0"]
impl crate::Resettable for RSTCTL_RESET_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
