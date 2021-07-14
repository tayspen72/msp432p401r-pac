#[doc = "Register `REF_CAL_TAG` reader"]
pub struct R(crate::R<REF_CAL_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_CAL_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_CAL_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_CAL_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "REF Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cal_tag](index.html) module"]
pub struct REF_CAL_TAG_SPEC;
impl crate::RegisterSpec for REF_CAL_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_cal_tag::R](R) reader structure"]
impl crate::Readable for REF_CAL_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REF_CAL_TAG to value 0x08"]
impl crate::Resettable for REF_CAL_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
