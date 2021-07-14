#[doc = "Register `UCBxADDRX` reader"]
pub struct R(crate::R<UCBXADDRX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXADDRX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXADDRX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXADDRX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADDRX` reader - Received Address Register"]
pub struct ADDRX_R(crate::FieldReader<u16, u16>);
impl ADDRX_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:9 - Received Address Register"]
    #[inline(always)]
    pub fn addrx(&self) -> ADDRX_R {
        ADDRX_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "eUSCI_Bx I2C Received Address Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_addrx](index.html) module"]
pub struct UCBXADDRX_SPEC;
impl crate::RegisterSpec for UCBXADDRX_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_addrx::R](R) reader structure"]
impl crate::Readable for UCBXADDRX_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCBxADDRX to value 0"]
impl crate::Resettable for UCBXADDRX_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
