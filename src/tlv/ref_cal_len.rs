#[doc = "Register `REF_CAL_LEN` reader"]
pub struct R(crate::R<REF_CAL_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REF_CAL_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REF_CAL_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REF_CAL_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "REF Calibration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cal_len](index.html) module"]
pub struct REF_CAL_LEN_SPEC;
impl crate::RegisterSpec for REF_CAL_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ref_cal_len::R](R) reader structure"]
impl crate::Readable for REF_CAL_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REF_CAL_LEN to value 0"]
impl crate::Resettable for REF_CAL_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
