#[doc = "Register `UCBxRXBUF` reader"]
pub struct R(crate::R<UCBXRXBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXRXBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXRXBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXRXBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UCRXBUF` reader - Receive data buffer"]
pub struct UCRXBUF_R(crate::FieldReader<u8, u8>);
impl UCRXBUF_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCRXBUF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCRXBUF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Receive data buffer"]
    #[inline(always)]
    pub fn ucrxbuf(&self) -> UCRXBUF_R {
        UCRXBUF_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "eUSCI_Bx Receive Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_rxbuf](index.html) module"]
pub struct UCBXRXBUF_SPEC;
impl crate::RegisterSpec for UCBXRXBUF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_rxbuf::R](R) reader structure"]
impl crate::Readable for UCBXRXBUF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCBxRXBUF to value 0"]
impl crate::Resettable for UCBXRXBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
