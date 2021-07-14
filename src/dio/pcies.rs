#[doc = "Register `PCIES` reader"]
pub struct R(crate::R<PCIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIES` writer"]
pub struct W(crate::W<PCIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIES_SPEC>;
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
impl From<crate::W<PCIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IES` reader - Port 5 Interrupt Edge Select"]
pub struct P5IES_R(crate::FieldReader<u8, u8>);
impl P5IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IES` writer - Port 5 Interrupt Edge Select"]
pub struct P5IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6IES` reader - Port 6 Interrupt Edge Select"]
pub struct P6IES_R(crate::FieldReader<u8, u8>);
impl P6IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6IES` writer - Port 6 Interrupt Edge Select"]
pub struct P6IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&self) -> P5IES_R {
        P5IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&self) -> P6IES_R {
        P6IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p5ies(&mut self) -> P5IES_W {
        P5IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p6ies(&mut self) -> P6IES_W {
        P6IES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcies](index.html) module"]
pub struct PCIES_SPEC;
impl crate::RegisterSpec for PCIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcies::R](R) reader structure"]
impl crate::Readable for PCIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcies::W](W) writer structure"]
impl crate::Writable for PCIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIES to value 0"]
impl crate::Resettable for PCIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
