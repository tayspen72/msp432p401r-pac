#[doc = "Register `PDIN` reader"]
pub struct R(crate::R<PDIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P7IN` reader - Port 7 Input"]
pub struct P7IN_R(crate::FieldReader<u8, u8>);
impl P7IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8IN` reader - Port 8 Input"]
pub struct P8IN_R(crate::FieldReader<u8, u8>);
impl P8IN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8IN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Input"]
    #[inline(always)]
    pub fn p7in(&self) -> P7IN_R {
        P7IN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Input"]
    #[inline(always)]
    pub fn p8in(&self) -> P8IN_R {
        P8IN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Port D Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdin](index.html) module"]
pub struct PDIN_SPEC;
impl crate::RegisterSpec for PDIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdin::R](R) reader structure"]
impl crate::Readable for PDIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDIN to value 0"]
impl crate::Resettable for PDIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
