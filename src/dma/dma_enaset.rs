#[doc = "Register `DMA_ENASET` reader"]
pub struct R(crate::R<DMA_ENASET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ENASET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ENASET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ENASET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ENASET` writer"]
pub struct W(crate::W<DMA_ENASET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ENASET_SPEC>;
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
impl From<crate::W<DMA_ENASET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ENASET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Returns the enable status of the channels, or enables the corresponding channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_A {
    #[doc = "0: Channel C is disabled."]
    SET_0_READ = 0,
    #[doc = "1: Channel C is enabled."]
    SET_1_READ = 1,
}
impl From<SET_A> for u32 {
    #[inline(always)]
    fn from(variant: SET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` reader - Returns the enable status of the channels, or enables the corresponding channels."]
pub struct SET_R(crate::FieldReader<u32, SET_A>);
impl SET_R {
    pub(crate) fn new(bits: u32) -> Self {
        SET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SET_A> {
        match self.bits {
            0 => Some(SET_A::SET_0_READ),
            1 => Some(SET_A::SET_1_READ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SET_0_READ`"]
    #[inline(always)]
    pub fn is_set_0_read(&self) -> bool {
        **self == SET_A::SET_0_READ
    }
    #[doc = "Checks if the value of the field is `SET_1_READ`"]
    #[inline(always)]
    pub fn is_set_1_read(&self) -> bool {
        **self == SET_A::SET_1_READ
    }
}
impl core::ops::Deref for SET_R {
    type Target = crate::FieldReader<u32, SET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Returns the enable status of the channels, or enables the corresponding channels.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_ENACLR Register to disable a channel."]
    SET_0_WRITE = 0,
    #[doc = "1: Enables channel C.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    SET_1_WRITE = 1,
}
impl From<SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` writer - Returns the enable status of the channels, or enables the corresponding channels."]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_ENACLR Register to disable a channel."]
    #[inline(always)]
    pub fn set_0_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_0_WRITE)
    }
    #[doc = "Enables channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn set_1_write(self) -> &'a mut W {
        self.variant(SET_AW::SET_1_WRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Returns the enable status of the channels, or enables the corresponding channels."]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Returns the enable status of the channels, or enables the corresponding channels."]
    #[inline(always)]
    pub fn set(&mut self) -> SET_W {
        SET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Enable Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_enaset](index.html) module"]
pub struct DMA_ENASET_SPEC;
impl crate::RegisterSpec for DMA_ENASET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_enaset::R](R) reader structure"]
impl crate::Readable for DMA_ENASET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_enaset::W](W) writer structure"]
impl crate::Writable for DMA_ENASET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ENASET to value 0"]
impl crate::Resettable for DMA_ENASET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
