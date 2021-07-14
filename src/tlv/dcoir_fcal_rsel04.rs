#[doc = "Register `DCOIR_FCAL_RSEL04` reader"]
pub struct R(crate::R<DCOIR_FCAL_RSEL04_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOIR_FCAL_RSEL04_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOIR_FCAL_RSEL04_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOIR_FCAL_RSEL04_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DCO IR mode: Frequency calibration for DCORSEL 0 to 4\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_fcal_rsel04](index.html) module"]
pub struct DCOIR_FCAL_RSEL04_SPEC;
impl crate::RegisterSpec for DCOIR_FCAL_RSEL04_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcoir_fcal_rsel04::R](R) reader structure"]
impl crate::Readable for DCOIR_FCAL_RSEL04_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCOIR_FCAL_RSEL04 to value 0"]
impl crate::Resettable for DCOIR_FCAL_RSEL04_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
