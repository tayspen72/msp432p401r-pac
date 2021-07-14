#[doc = "Register `DMA_WAITSTAT` reader"]
pub struct R(crate::R<DMA_WAITSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_WAITSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_WAITSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_WAITSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Channel wait on request status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WAITREQ_A {
    #[doc = "0: dma_waitonreq\\[C\\]
is LOW."]
    WAITREQ_0 = 0,
    #[doc = "1: dma_waitonreq\\[C\\]
is HIGH."]
    WAITREQ_1 = 1,
}
impl From<WAITREQ_A> for u32 {
    #[inline(always)]
    fn from(variant: WAITREQ_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAITREQ` reader - Channel wait on request status."]
pub struct WAITREQ_R(crate::FieldReader<u32, WAITREQ_A>);
impl WAITREQ_R {
    pub(crate) fn new(bits: u32) -> Self {
        WAITREQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WAITREQ_A> {
        match self.bits {
            0 => Some(WAITREQ_A::WAITREQ_0),
            1 => Some(WAITREQ_A::WAITREQ_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WAITREQ_0`"]
    #[inline(always)]
    pub fn is_waitreq_0(&self) -> bool {
        **self == WAITREQ_A::WAITREQ_0
    }
    #[doc = "Checks if the value of the field is `WAITREQ_1`"]
    #[inline(always)]
    pub fn is_waitreq_1(&self) -> bool {
        **self == WAITREQ_A::WAITREQ_1
    }
}
impl core::ops::Deref for WAITREQ_R {
    type Target = crate::FieldReader<u32, WAITREQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel wait on request status."]
    #[inline(always)]
    pub fn waitreq(&self) -> WAITREQ_R {
        WAITREQ_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel Wait on Request Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_waitstat](index.html) module"]
pub struct DMA_WAITSTAT_SPEC;
impl crate::RegisterSpec for DMA_WAITSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_waitstat::R](R) reader structure"]
impl crate::Readable for DMA_WAITSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_WAITSTAT to value 0"]
impl crate::Resettable for DMA_WAITSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
