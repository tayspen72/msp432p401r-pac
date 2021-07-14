#[doc = "Register `DMA_USEBURSTCLR` writer"]
pub struct W(crate::W<DMA_USEBURSTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_USEBURSTCLR_SPEC>;
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
impl From<crate::W<DMA_USEBURSTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_USEBURSTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Set the appropriate bit to enable dma_sreq to generate requests.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLR_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_USEBURST_SET Register to disable dma_sreq from generating requests."]
    CLR_0 = 0,
    #[doc = "1: Enables dma_sreq\\[C\\]
to generate DMA requests.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    CLR_1 = 1,
}
impl From<CLR_AW> for u32 {
    #[inline(always)]
    fn from(variant: CLR_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CLR` writer - Set the appropriate bit to enable dma_sreq to generate requests."]
pub struct CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_USEBURST_SET Register to disable dma_sreq from generating requests."]
    #[inline(always)]
    pub fn clr_0(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_0)
    }
    #[doc = "Enables dma_sreq\\[C\\]
to generate DMA requests. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn clr_1(self) -> &'a mut W {
        self.variant(CLR_AW::CLR_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Set the appropriate bit to enable dma_sreq to generate requests."]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W {
        CLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Useburst Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_useburstclr](index.html) module"]
pub struct DMA_USEBURSTCLR_SPEC;
impl crate::RegisterSpec for DMA_USEBURSTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dma_useburstclr::W](W) writer structure"]
impl crate::Writable for DMA_USEBURSTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_USEBURSTCLR to value 0"]
impl crate::Resettable for DMA_USEBURSTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
