#[doc = "Register `RANDOM_NUM_TAG` reader"]
pub struct R(crate::R<RANDOM_NUM_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RANDOM_NUM_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RANDOM_NUM_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RANDOM_NUM_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "128-bit Random Number Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [random_num_tag](index.html) module"]
pub struct RANDOM_NUM_TAG_SPEC;
impl crate::RegisterSpec for RANDOM_NUM_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [random_num_tag::R](R) reader structure"]
impl crate::Readable for RANDOM_NUM_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RANDOM_NUM_TAG to value 0x0d"]
impl crate::Resettable for RANDOM_NUM_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
