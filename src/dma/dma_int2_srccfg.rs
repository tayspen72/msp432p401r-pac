#[doc = "Register `DMA_INT2_SRCCFG` reader"]
pub struct R(crate::R<DMA_INT2_SRCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT2_SRCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT2_SRCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT2_SRCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT2_SRCCFG` writer"]
pub struct W(crate::W<DMA_INT2_SRCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT2_SRCCFG_SPEC>;
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
impl From<crate::W<DMA_INT2_SRCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT2_SRCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT_SRC` reader - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub struct INT_SRC_R(crate::FieldReader<u8, u8>);
impl INT_SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        INT_SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT_SRC` writer - Controls which channel's completion event is mapped as a source of this Interrupt"]
pub struct INT_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `EN` reader - Enables DMA_INT2 mapping"]
pub struct EN_R(crate::FieldReader<bool, bool>);
impl EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN` writer - Enables DMA_INT2 mapping"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&self) -> INT_SRC_R {
        INT_SRC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Enables DMA_INT2 mapping"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Controls which channel's completion event is mapped as a source of this Interrupt"]
    #[inline(always)]
    pub fn int_src(&mut self) -> INT_SRC_W {
        INT_SRC_W { w: self }
    }
    #[doc = "Bit 5 - Enables DMA_INT2 mapping"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt 2 Source Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int2_srccfg](index.html) module"]
pub struct DMA_INT2_SRCCFG_SPEC;
impl crate::RegisterSpec for DMA_INT2_SRCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int2_srccfg::R](R) reader structure"]
impl crate::Readable for DMA_INT2_SRCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int2_srccfg::W](W) writer structure"]
impl crate::Writable for DMA_INT2_SRCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_INT2_SRCCFG to value 0"]
impl crate::Resettable for DMA_INT2_SRCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
