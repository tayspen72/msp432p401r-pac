#[doc = "Register `RESERVED18` reader"]
pub struct R(crate::R<RESERVED18_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESERVED18_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESERVED18_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESERVED18_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Reserved\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reserved18](index.html) module"]
pub struct RESERVED18_SPEC;
impl crate::RegisterSpec for RESERVED18_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reserved18::R](R) reader structure"]
impl crate::Readable for RESERVED18_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RESERVED18 to value 0"]
impl crate::Resettable for RESERVED18_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
