#[doc = "Register `PCIE` reader"]
pub struct R(crate::R<PCIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIE` writer"]
pub struct W(crate::W<PCIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIE_SPEC>;
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
impl From<crate::W<PCIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IE` reader - Port 5 Interrupt Enable"]
pub struct P5IE_R(crate::FieldReader<u8, u8>);
impl P5IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IE` writer - Port 5 Interrupt Enable"]
pub struct P5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6IE` reader - Port 6 Interrupt Enable"]
pub struct P6IE_R(crate::FieldReader<u8, u8>);
impl P6IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6IE` writer - Port 6 Interrupt Enable"]
pub struct P6IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&self) -> P5IE_R {
        P5IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&self) -> P6IE_R {
        P6IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Enable"]
    #[inline(always)]
    pub fn p5ie(&mut self) -> P5IE_W {
        P5IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Enable"]
    #[inline(always)]
    pub fn p6ie(&mut self) -> P6IE_W {
        P6IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcie](index.html) module"]
pub struct PCIE_SPEC;
impl crate::RegisterSpec for PCIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcie::R](R) reader structure"]
impl crate::Readable for PCIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcie::W](W) writer structure"]
impl crate::Writable for PCIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIE to value 0"]
impl crate::Resettable for PCIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
