#[doc = "Register `UCAxMCTLW` reader"]
pub struct R(crate::R<UCAXMCTLW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXMCTLW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXMCTLW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXMCTLW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxMCTLW` writer"]
pub struct W(crate::W<UCAXMCTLW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXMCTLW_SPEC>;
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
impl From<crate::W<UCAXMCTLW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXMCTLW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Oversampling mode enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOS16_A {
    #[doc = "0: Disabled"]
    UCOS16_0 = 0,
    #[doc = "1: Enabled"]
    UCOS16_1 = 1,
}
impl From<UCOS16_A> for bool {
    #[inline(always)]
    fn from(variant: UCOS16_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOS16` reader - Oversampling mode enabled"]
pub struct UCOS16_R(crate::FieldReader<bool, UCOS16_A>);
impl UCOS16_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCOS16_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOS16_A {
        match self.bits {
            false => UCOS16_A::UCOS16_0,
            true => UCOS16_A::UCOS16_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOS16_0`"]
    #[inline(always)]
    pub fn is_ucos16_0(&self) -> bool {
        **self == UCOS16_A::UCOS16_0
    }
    #[doc = "Checks if the value of the field is `UCOS16_1`"]
    #[inline(always)]
    pub fn is_ucos16_1(&self) -> bool {
        **self == UCOS16_A::UCOS16_1
    }
}
impl core::ops::Deref for UCOS16_R {
    type Target = crate::FieldReader<bool, UCOS16_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOS16` writer - Oversampling mode enabled"]
pub struct UCOS16_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOS16_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOS16_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ucos16_0(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ucos16_1(self) -> &'a mut W {
        self.variant(UCOS16_A::UCOS16_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Field `UCBRF` reader - First modulation stage select"]
pub struct UCBRF_R(crate::FieldReader<u8, u8>);
impl UCBRF_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCBRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBRF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBRF` writer - First modulation stage select"]
pub struct UCBRF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u16 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `UCBRS` reader - Second modulation stage select"]
pub struct UCBRS_R(crate::FieldReader<u8, u8>);
impl UCBRS_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCBRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBRS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBRS` writer - Second modulation stage select"]
pub struct UCBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBRS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&self) -> UCOS16_R {
        UCOS16_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&self) -> UCBRF_R {
        UCBRF_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&self) -> UCBRS_R {
        UCBRS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Oversampling mode enabled"]
    #[inline(always)]
    pub fn ucos16(&mut self) -> UCOS16_W {
        UCOS16_W { w: self }
    }
    #[doc = "Bits 4:7 - First modulation stage select"]
    #[inline(always)]
    pub fn ucbrf(&mut self) -> UCBRF_W {
        UCBRF_W { w: self }
    }
    #[doc = "Bits 8:15 - Second modulation stage select"]
    #[inline(always)]
    pub fn ucbrs(&mut self) -> UCBRS_W {
        UCBRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Modulation Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_mctlw](index.html) module"]
pub struct UCAXMCTLW_SPEC;
impl crate::RegisterSpec for UCAXMCTLW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_mctlw::R](R) reader structure"]
impl crate::Readable for UCAXMCTLW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_mctlw::W](W) writer structure"]
impl crate::Writable for UCAXMCTLW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxMCTLW to value 0"]
impl crate::Resettable for UCAXMCTLW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
