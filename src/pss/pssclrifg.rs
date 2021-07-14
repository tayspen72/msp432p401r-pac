#[doc = "Register `PSSCLRIFG` reader"]
pub struct R(crate::R<PSSCLRIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSCLRIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSCLRIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSCLRIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSCLRIFG` writer"]
pub struct W(crate::W<PSSCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSCLRIFG_SPEC>;
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
impl From<crate::W<PSSCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SVSMH clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRSVSMHIFG_AW {
    #[doc = "0: No effect"]
    CLRSVSMHIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLRSVSMHIFG_1 = 1,
}
impl From<CLRSVSMHIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRSVSMHIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRSVSMHIFG` writer - SVSMH clear interrupt flag"]
pub struct CLRSVSMHIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRSVSMHIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRSVSMHIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clrsvsmhifg_0(self) -> &'a mut W {
        self.variant(CLRSVSMHIFG_AW::CLRSVSMHIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg_1(self) -> &'a mut W {
        self.variant(CLRSVSMHIFG_AW::CLRSVSMHIFG_1)
    }
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
impl W {
    #[doc = "Bit 1 - SVSMH clear interrupt flag"]
    #[inline(always)]
    pub fn clrsvsmhifg(&mut self) -> CLRSVSMHIFG_W {
        CLRSVSMHIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssclrifg](index.html) module"]
pub struct PSSCLRIFG_SPEC;
impl crate::RegisterSpec for PSSCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssclrifg::R](R) reader structure"]
impl crate::Readable for PSSCLRIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssclrifg::W](W) writer structure"]
impl crate::Writable for PSSCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSSCLRIFG to value 0"]
impl crate::Resettable for PSSCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
