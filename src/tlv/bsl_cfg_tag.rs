#[doc = "Register `BSL_CFG_TAG` reader"]
pub struct R(crate::R<BSL_CFG_TAG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSL_CFG_TAG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSL_CFG_TAG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSL_CFG_TAG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BSL Configuration Tag\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_cfg_tag](index.html) module"]
pub struct BSL_CFG_TAG_SPEC;
impl crate::RegisterSpec for BSL_CFG_TAG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsl_cfg_tag::R](R) reader structure"]
impl crate::Readable for BSL_CFG_TAG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSL_CFG_TAG to value 0x0f"]
impl crate::Resettable for BSL_CFG_TAG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
