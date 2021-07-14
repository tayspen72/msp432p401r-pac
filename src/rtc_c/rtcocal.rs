#[doc = "Register `RTCOCAL` reader"]
pub struct R(crate::R<RTCOCAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCOCAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCOCAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCOCAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCOCAL` writer"]
pub struct W(crate::W<RTCOCAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCOCAL_SPEC>;
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
impl From<crate::W<RTCOCAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCOCAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCOCAL` reader - Real-time clock offset error calibration"]
pub struct RTCOCAL_R(crate::FieldReader<u8, u8>);
impl RTCOCAL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCOCAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCOCAL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCAL` writer - Real-time clock offset error calibration"]
pub struct RTCOCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Real-time clock offset error calibration sign\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCOCALS_A {
    #[doc = "0: Down calibration. Frequency adjusted down."]
    RTCOCALS_0 = 0,
    #[doc = "1: Up calibration. Frequency adjusted up."]
    RTCOCALS_1 = 1,
}
impl From<RTCOCALS_A> for bool {
    #[inline(always)]
    fn from(variant: RTCOCALS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCOCALS` reader - Real-time clock offset error calibration sign"]
pub struct RTCOCALS_R(crate::FieldReader<bool, RTCOCALS_A>);
impl RTCOCALS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCOCALS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCOCALS_A {
        match self.bits {
            false => RTCOCALS_A::RTCOCALS_0,
            true => RTCOCALS_A::RTCOCALS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_0`"]
    #[inline(always)]
    pub fn is_rtcocals_0(&self) -> bool {
        **self == RTCOCALS_A::RTCOCALS_0
    }
    #[doc = "Checks if the value of the field is `RTCOCALS_1`"]
    #[inline(always)]
    pub fn is_rtcocals_1(&self) -> bool {
        **self == RTCOCALS_A::RTCOCALS_1
    }
}
impl core::ops::Deref for RTCOCALS_R {
    type Target = crate::FieldReader<bool, RTCOCALS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCOCALS` writer - Real-time clock offset error calibration sign"]
pub struct RTCOCALS_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCOCALS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCOCALS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Down calibration. Frequency adjusted down."]
    #[inline(always)]
    pub fn rtcocals_0(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_0)
    }
    #[doc = "Up calibration. Frequency adjusted up."]
    #[inline(always)]
    pub fn rtcocals_1(self) -> &'a mut W {
        self.variant(RTCOCALS_A::RTCOCALS_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&self) -> RTCOCAL_R {
        RTCOCAL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&self) -> RTCOCALS_R {
        RTCOCALS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Real-time clock offset error calibration"]
    #[inline(always)]
    pub fn rtcocal(&mut self) -> RTCOCAL_W {
        RTCOCAL_W { w: self }
    }
    #[doc = "Bit 15 - Real-time clock offset error calibration sign"]
    #[inline(always)]
    pub fn rtcocals(&mut self) -> RTCOCALS_W {
        RTCOCALS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCOCAL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcocal](index.html) module"]
pub struct RTCOCAL_SPEC;
impl crate::RegisterSpec for RTCOCAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcocal::R](R) reader structure"]
impl crate::Readable for RTCOCAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcocal::W](W) writer structure"]
impl crate::Writable for RTCOCAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCOCAL to value 0"]
impl crate::Resettable for RTCOCAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
