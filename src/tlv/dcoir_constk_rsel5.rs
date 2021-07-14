#[doc = "Register `DCOIR_CONSTK_RSEL5` reader"]
pub struct R(crate::R<DCOIR_CONSTK_RSEL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCOIR_CONSTK_RSEL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCOIR_CONSTK_RSEL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCOIR_CONSTK_RSEL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "DCO IR mode: DCO Constant (K) for DCORSEL 5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcoir_constk_rsel5](index.html) module"]
pub struct DCOIR_CONSTK_RSEL5_SPEC;
impl crate::RegisterSpec for DCOIR_CONSTK_RSEL5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcoir_constk_rsel5::R](R) reader structure"]
impl crate::Readable for DCOIR_CONSTK_RSEL5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCOIR_CONSTK_RSEL5 to value 0"]
impl crate::Resettable for DCOIR_CONSTK_RSEL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
