#[doc = "Register `AESACTL1` reader"]
pub struct R(crate::R<AESACTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESACTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESACTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESACTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESACTL1` writer"]
pub struct W(crate::W<AESACTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESACTL1_SPEC>;
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
impl From<crate::W<AESACTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESACTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESBLKCNTx` reader - Cipher Block Counter"]
pub struct AESBLKCNTX_R(crate::FieldReader<u8, u8>);
impl AESBLKCNTX_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESBLKCNTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESBLKCNTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBLKCNTx` writer - Cipher Block Counter"]
pub struct AESBLKCNTX_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBLKCNTX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&self) -> AESBLKCNTX_R {
        AESBLKCNTX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Cipher Block Counter"]
    #[inline(always)]
    pub fn aesblkcntx(&mut self) -> AESBLKCNTX_W {
        AESBLKCNTX_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesactl1](index.html) module"]
pub struct AESACTL1_SPEC;
impl crate::RegisterSpec for AESACTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesactl1::R](R) reader structure"]
impl crate::Readable for AESACTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesactl1::W](W) writer structure"]
impl crate::Writable for AESACTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESACTL1 to value 0"]
impl crate::Resettable for AESACTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
