#[doc = "Register `T32RIS1` reader"]
pub struct R(crate::R<T32RIS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32RIS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32RIS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32RIS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RAW_IFG` reader - Raw interrupt status"]
pub struct RAW_IFG_R(crate::FieldReader<bool, bool>);
impl RAW_IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RAW_IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RAW_IFG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt status"]
    #[inline(always)]
    pub fn raw_ifg(&self) -> RAW_IFG_R {
        RAW_IFG_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Timer 1 Raw Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32ris1](index.html) module"]
pub struct T32RIS1_SPEC;
impl crate::RegisterSpec for T32RIS1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32ris1::R](R) reader structure"]
impl crate::Readable for T32RIS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets T32RIS1 to value 0"]
impl crate::Resettable for T32RIS1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
