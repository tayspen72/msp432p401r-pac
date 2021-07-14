#[doc = "Register `PJIN` reader"]
pub struct R(crate::R<PJIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PJIN` reader - Port J Input"]
pub struct PJIN_R(crate::FieldReader<u16, u16>);
impl PJIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Input"]
    #[inline(always)]
    pub fn pjin(&self) -> PJIN_R {
        PJIN_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Port J Input\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjin](index.html) module"]
pub struct PJIN_SPEC;
impl crate::RegisterSpec for PJIN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjin::R](R) reader structure"]
impl crate::Readable for PJIN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PJIN to value 0"]
impl crate::Resettable for PJIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
