#[doc = "Register `PBIN` reader"]
pub struct R(crate::R<PBIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P3IN` reader - Port 3 Input"]
pub struct P3IN_R(crate::FieldReader<u8, u8>);
impl P3IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IN` reader - Port 4 Input"]
pub struct P4IN_R(crate::FieldReader<u8, u8>);
impl P4IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Input"]
    #[inline(always)]
    pub fn p3in(&self) -> P3IN_R {
        P3IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Input"]
    #[inline(always)]
    pub fn p4in(&self) -> P4IN_R {
        P4IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port B Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbin](index.html) module"]
pub struct PBIN_SPEC;
impl crate::RegisterSpec for PBIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbin::R](R) reader structure"]
impl crate::Readable for PBIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PBIN to value 0"]
impl crate::Resettable for PBIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
