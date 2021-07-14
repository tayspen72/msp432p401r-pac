#[doc = "Register `DMA_ALTSET` reader"]
pub struct R(crate::R<DMA_ALTSET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ALTSET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ALTSET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ALTSET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ALTSET` writer"]
pub struct W(crate::W<DMA_ALTSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ALTSET_SPEC>;
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
impl From<crate::W<DMA_ALTSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ALTSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_A {
    #[doc = "0: DMA channel C is using the primary data structure."]
    SET_0_READ = 0,
    #[doc = "1: DMA channel C is using the alternate data structure."]
    SET_1_READ = 1,
}
impl From<SET_A> for u32 {
    #[inline(always)]
    fn from(variant: SET_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` reader - Channel Primary-Alternate Set Register"]
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
#[doc = "Channel Primary-Alternate Set Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SET_AW {
    #[doc = "0: No effect.\r\r\nUse the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    SEL_0_WRITE = 0,
    #[doc = "1: Selects the alternate data structure for channel C.\r\r\nWriting to a bit where a DMA channel is not implemented has no effect."]
    SEL_1_WRITE = 1,
}
impl From<SET_AW> for u32 {
    #[inline(always)]
    fn from(variant: SET_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `SET` writer - Channel Primary-Alternate Set Register"]
pub struct SET_W<'a> {
    w: &'a mut W,
}
impl<'a> SET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SET_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No effect. Use the DMA_ALTCLR Register to set bit \\[C\\]
to 0."]
    #[inline(always)]
    pub fn sel_0_write(self) -> &'a mut W {
        self.variant(SET_AW::SEL_0_WRITE)
    }
    #[doc = "Selects the alternate data structure for channel C. Writing to a bit where a DMA channel is not implemented has no effect."]
    #[inline(always)]
    pub fn sel_1_write(self) -> &'a mut W {
        self.variant(SET_AW::SEL_1_WRITE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
    #[inline(always)]
    pub fn set(&self) -> SET_R {
        SET_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Channel Primary-Alternate Set Register"]
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
#[doc = "Channel Primary-Alternate Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_altset](index.html) module"]
pub struct DMA_ALTSET_SPEC;
impl crate::RegisterSpec for DMA_ALTSET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_altset::R](R) reader structure"]
impl crate::Readable for DMA_ALTSET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_altset::W](W) writer structure"]
impl crate::Writable for DMA_ALTSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ALTSET to value 0"]
impl crate::Resettable for DMA_ALTSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
