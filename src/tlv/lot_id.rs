#[doc = "Register `LOT_ID` reader"]
pub struct R(crate::R<LOT_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOT_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOT_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOT_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Lot ID\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lot_id](index.html) module"]
pub struct LOT_ID_SPEC;
impl crate::RegisterSpec for LOT_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lot_id::R](R) reader structure"]
impl crate::Readable for LOT_ID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOT_ID to value 0"]
impl crate::Resettable for LOT_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
