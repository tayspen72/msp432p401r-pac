#[doc = "Register `UCBxI2CSA` reader"]
pub struct R(crate::R<UCBXI2CSA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXI2CSA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXI2CSA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXI2CSA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxI2CSA` writer"]
pub struct W(crate::W<UCBXI2CSA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXI2CSA_SPEC>;
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
impl From<crate::W<UCBXI2CSA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXI2CSA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I2CSA` reader - I2C slave address"]
pub struct I2CSA_R(crate::FieldReader<u16, u16>);
impl I2CSA_R {
    pub(crate) fn new(bits: u16) -> Self {
        I2CSA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CSA_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CSA` writer - I2C slave address"]
pub struct I2CSA_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CSA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&self) -> I2CSA_R {
        I2CSA_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - I2C slave address"]
    #[inline(always)]
    pub fn i2csa(&mut self) -> I2CSA_W {
        I2CSA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Slave Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_i2csa](index.html) module"]
pub struct UCBXI2CSA_SPEC;
impl crate::RegisterSpec for UCBXI2CSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_i2csa::R](R) reader structure"]
impl crate::Readable for UCBXI2CSA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_i2csa::W](W) writer structure"]
impl crate::Writable for UCBXI2CSA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxI2CSA to value 0"]
impl crate::Resettable for UCBXI2CSA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
