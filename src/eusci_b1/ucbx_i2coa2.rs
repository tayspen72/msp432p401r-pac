#[doc = "Register `UCBxI2COA2` reader"]
pub struct R(crate::R<UCBXI2COA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXI2COA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXI2COA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXI2COA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxI2COA2` writer"]
pub struct W(crate::W<UCBXI2COA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXI2COA2_SPEC>;
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
impl From<crate::W<UCBXI2COA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXI2COA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2COA2` reader - I2C own address"]
pub struct I2COA2_R(crate::FieldReader<u16, u16>);
impl I2COA2_R {
    pub(crate) fn new(bits: u16) -> Self {
        I2COA2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2COA2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2COA2` writer - I2C own address"]
pub struct I2COA2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2COA2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
#[doc = "Own Address enable register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCOAEN_A {
    #[doc = "0: The slave address defined in I2COA2 is disabled"]
    UCOAEN_0 = 0,
    #[doc = "1: The slave address defined in I2COA2 is enabled"]
    UCOAEN_1 = 1,
}
impl From<UCOAEN_A> for bool {
    #[inline(always)]
    fn from(variant: UCOAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCOAEN` reader - Own Address enable register"]
pub struct UCOAEN_R(crate::FieldReader<bool, UCOAEN_A>);
impl UCOAEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCOAEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCOAEN_A {
        match self.bits {
            false => UCOAEN_A::UCOAEN_0,
            true => UCOAEN_A::UCOAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCOAEN_0`"]
    #[inline(always)]
    pub fn is_ucoaen_0(&self) -> bool {
        **self == UCOAEN_A::UCOAEN_0
    }
    #[doc = "Checks if the value of the field is `UCOAEN_1`"]
    #[inline(always)]
    pub fn is_ucoaen_1(&self) -> bool {
        **self == UCOAEN_A::UCOAEN_1
    }
}
impl core::ops::Deref for UCOAEN_R {
    type Target = crate::FieldReader<bool, UCOAEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCOAEN` writer - Own Address enable register"]
pub struct UCOAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCOAEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCOAEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The slave address defined in I2COA2 is disabled"]
    #[inline(always)]
    pub fn ucoaen_0(self) -> &'a mut W {
        self.variant(UCOAEN_A::UCOAEN_0)
    }
    #[doc = "The slave address defined in I2COA2 is enabled"]
    #[inline(always)]
    pub fn ucoaen_1(self) -> &'a mut W {
        self.variant(UCOAEN_A::UCOAEN_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa2(&self) -> I2COA2_R {
        I2COA2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&self) -> UCOAEN_R {
        UCOAEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C own address"]
    #[inline(always)]
    pub fn i2coa2(&mut self) -> I2COA2_W {
        I2COA2_W { w: self }
    }
    #[doc = "Bit 10 - Own Address enable register"]
    #[inline(always)]
    pub fn ucoaen(&mut self) -> UCOAEN_W {
        UCOAEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Own Address 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2coa2](index.html) module"]
pub struct UCBXI2COA2_SPEC;
impl crate::RegisterSpec for UCBXI2COA2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_i2coa2::R](R) reader structure"]
impl crate::Readable for UCBXI2COA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_i2coa2::W](W) writer structure"]
impl crate::Writable for UCBXI2COA2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxI2COA2 to value 0"]
impl crate::Resettable for UCBXI2COA2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
