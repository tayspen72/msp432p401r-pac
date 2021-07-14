#[doc = "Register `DMA_ERRCLR` reader"]
pub struct R(crate::R<DMA_ERRCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ERRCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ERRCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ERRCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ERRCLR` writer"]
pub struct W(crate::W<DMA_ERRCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ERRCLR_SPEC>;
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
impl From<crate::W<DMA_ERRCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ERRCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRCLR_A {
    #[doc = "0: dma_err is LOW"]
    ERRCLR_0_READ = 0,
    #[doc = "1: dma_err is HIGH."]
    ERRCLR_1_READ = 1,
}
impl From<ERRCLR_A> for bool {
    #[inline(always)]
    fn from(variant: ERRCLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCLR` reader - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
pub struct ERRCLR_R(crate::FieldReader<bool, ERRCLR_A>);
impl ERRCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRCLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRCLR_A {
        match self.bits {
            false => ERRCLR_A::ERRCLR_0_READ,
            true => ERRCLR_A::ERRCLR_1_READ,
        }
    }
    #[doc = "Checks if the value of the field is `ERRCLR_0_READ`"]
    #[inline(always)]
    pub fn is_errclr_0_read(&self) -> bool {
        **self == ERRCLR_A::ERRCLR_0_READ
    }
    #[doc = "Checks if the value of the field is `ERRCLR_1_READ`"]
    #[inline(always)]
    pub fn is_errclr_1_read(&self) -> bool {
        **self == ERRCLR_A::ERRCLR_1_READ
    }
}
impl core::ops::Deref for ERRCLR_R {
    type Target = crate::FieldReader<bool, ERRCLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ERRCLR_AW {
    #[doc = "0: No effect, status of dma_err is unchanged."]
    ERRCLR_0_WRITE = 0,
    #[doc = "1: Sets dma_err LOW."]
    ERRCLR_1_WRITE = 1,
}
impl From<ERRCLR_AW> for bool {
    #[inline(always)]
    fn from(variant: ERRCLR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRCLR` writer - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
pub struct ERRCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRCLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ERRCLR_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect, status of dma_err is unchanged."]
    #[inline(always)]
    pub fn errclr_0_write(self) -> &'a mut W {
        self.variant(ERRCLR_AW::ERRCLR_0_WRITE)
    }
    #[doc = "Sets dma_err LOW."]
    #[inline(always)]
    pub fn errclr_1_write(self) -> &'a mut W {
        self.variant(ERRCLR_AW::ERRCLR_1_WRITE)
    }
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&self) -> ERRCLR_R {
        ERRCLR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Returns the status of dma_err, or sets the signal LOW. For test purposes, use the ERRSET register to set dma_err HIGH. Note: If you deassert dma_err at the same time as an ERROR occurs on the AHB-Lite bus, then the ERROR condition takes precedence and dma_err remains asserted."]
    #[inline(always)]
    pub fn errclr(&mut self) -> ERRCLR_W {
        ERRCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bus Error Clear Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_errclr](index.html) module"]
pub struct DMA_ERRCLR_SPEC;
impl crate::RegisterSpec for DMA_ERRCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_errclr::R](R) reader structure"]
impl crate::Readable for DMA_ERRCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_errclr::W](W) writer structure"]
impl crate::Writable for DMA_ERRCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ERRCLR to value 0"]
impl crate::Resettable for DMA_ERRCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
