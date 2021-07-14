#[doc = "Register `BSL_CFG_LEN` reader"]
pub struct R(crate::R<BSL_CFG_LEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSL_CFG_LEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSL_CFG_LEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSL_CFG_LEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BSL Configuration Length\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_cfg_len](index.html) module"]
pub struct BSL_CFG_LEN_SPEC;
impl crate::RegisterSpec for BSL_CFG_LEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsl_cfg_len::R](R) reader structure"]
impl crate::Readable for BSL_CFG_LEN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSL_CFG_LEN to value 0"]
impl crate::Resettable for BSL_CFG_LEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
