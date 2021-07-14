#[doc = "Register `DMA_SWREQ` writer"]
pub struct W(crate::W<DMA_SWREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SWREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_SWREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SWREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CHNL_SW_REQ_AW {
    #[doc = "0: Does not create a DMA request for the channel"]
    CHNL_SW_REQ_0 = 0,
    #[doc = "1: Creates a DMA request for the channel"]
    CHNL_SW_REQ_1 = 1,
}
impl From<CHNL_SW_REQ_AW> for u32 {
    #[inline(always)]
    fn from(variant: CHNL_SW_REQ_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CHNL_SW_REQ` writer - Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel."]
pub struct CHNL_SW_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CHNL_SW_REQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHNL_SW_REQ_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Does not create a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_0(self) -> &'a mut W {
        self.variant(CHNL_SW_REQ_AW::CHNL_SW_REQ_0)
    }
    #[doc = "Creates a DMA request for the channel"]
    #[inline(always)]
    pub fn chnl_sw_req_1(self) -> &'a mut W {
        self.variant(CHNL_SW_REQ_AW::CHNL_SW_REQ_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to generate a software DMA request on the corresponding DMA channel. Writing to a bit where a DMA channel is not implemented does not create a DMA request for that channel."]
    #[inline(always)]
    pub fn chnl_sw_req(&mut self) -> CHNL_SW_REQ_W {
        CHNL_SW_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Software Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_swreq](index.html) module"]
pub struct DMA_SWREQ_SPEC;
impl crate::RegisterSpec for DMA_SWREQ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_swreq::W](W) writer structure"]
impl crate::Writable for DMA_SWREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SWREQ to value 0"]
impl crate::Resettable for DMA_SWREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
