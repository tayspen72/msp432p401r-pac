#[doc = "Register `FLCTL_READMARGIN_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_READMARGIN_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_READMARGIN_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_READMARGIN_TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_READMARGIN_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SETUP` reader - Length of the Setup phase for this operation"]
pub struct SETUP_R(crate::FieldReader<u8, u8>);
impl SETUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Read Margin Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_readmargin_timctl](index.html) module"]
pub struct FLCTL_READMARGIN_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_READMARGIN_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_readmargin_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_READMARGIN_TIMCTL_SPEC {
    type Reader = R;
}
