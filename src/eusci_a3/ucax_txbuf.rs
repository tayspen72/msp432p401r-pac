#[doc = "Register `UCAxTXBUF` reader"]
pub struct R(crate::R<UCAXTXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXTXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXTXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXTXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxTXBUF` writer"]
pub struct W(crate::W<UCAXTXBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXTXBUF_SPEC>;
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
impl From<crate::W<UCAXTXBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXTXBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCTXBUF` reader - Transmit data buffer"]
pub struct UCTXBUF_R(crate::FieldReader<u8, u8>);
impl UCTXBUF_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCTXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTXBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXBUF` writer - Transmit data buffer"]
pub struct UCTXBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&self) -> UCTXBUF_R {
        UCTXBUF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit data buffer"]
    #[inline(always)]
    pub fn uctxbuf(&mut self) -> UCTXBUF_W {
        UCTXBUF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Transmit Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_txbuf](index.html) module"]
pub struct UCAXTXBUF_SPEC;
impl crate::RegisterSpec for UCAXTXBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_txbuf::R](R) reader structure"]
impl crate::Readable for UCAXTXBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_txbuf::W](W) writer structure"]
impl crate::Writable for UCAXTXBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxTXBUF to value 0"]
impl crate::Resettable for UCAXTXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
