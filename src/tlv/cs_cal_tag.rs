#[doc = "Register `CS_CAL_TAG` reader"]
pub struct R(crate::R<CS_CAL_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_CAL_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_CAL_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_CAL_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Clock System Calibration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs_cal_tag](index.html) module"]
pub struct CS_CAL_TAG_SPEC;
impl crate::RegisterSpec for CS_CAL_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs_cal_tag::R](R) reader structure"]
impl crate::Readable for CS_CAL_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CS_CAL_TAG to value 0x03"]
impl crate::Resettable for CS_CAL_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
