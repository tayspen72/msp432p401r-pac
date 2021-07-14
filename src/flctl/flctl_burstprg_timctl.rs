#[doc = "Register `FLCTL_BURSTPRG_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_BURSTPRG_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BURSTPRG_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BURSTPRG_TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BURSTPRG_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACTIVE` reader - Length of the Active phase for this operation"]
pub struct ACTIVE_R(crate::FieldReader<u32, u32>);
impl ACTIVE_R {
    pub(crate) fn new(bits: u32) -> Self {
        ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACTIVE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 8:27 - Length of the Active phase for this operation"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 8) & 0x000f_ffff) as u32)
    }
}
#[doc = "Burst Program Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_burstprg_timctl](index.html) module"]
pub struct FLCTL_BURSTPRG_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_BURSTPRG_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_burstprg_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_BURSTPRG_TIMCTL_SPEC {
    type Reader = R;
}
