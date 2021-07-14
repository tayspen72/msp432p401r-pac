#[doc = "Register `PEIES` reader"]
pub struct R(crate::R<PEIES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEIES` writer"]
pub struct W(crate::W<PEIES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEIES_SPEC>;
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
impl From<crate::W<PEIES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEIES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9IES` reader - Port 9 Interrupt Edge Select"]
pub struct P9IES_R(crate::FieldReader<u8, u8>);
impl P9IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9IES` writer - Port 9 Interrupt Edge Select"]
pub struct P9IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10IES` reader - Port 10 Interrupt Edge Select"]
pub struct P10IES_R(crate::FieldReader<u8, u8>);
impl P10IES_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10IES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10IES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10IES` writer - Port 10 Interrupt Edge Select"]
pub struct P10IES_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&self) -> P9IES_R {
        P9IES_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&self) -> P10IES_R {
        P10IES_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p9ies(&mut self) -> P9IES_W {
        P9IES_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Edge Select"]
    #[inline(always)]
    pub fn p10ies(&mut self) -> P10IES_W {
        P10IES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Interrupt Edge Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peies](index.html) module"]
pub struct PEIES_SPEC;
impl crate::RegisterSpec for PEIES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peies::R](R) reader structure"]
impl crate::Readable for PEIES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peies::W](W) writer structure"]
impl crate::Writable for PEIES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEIES to value 0"]
impl crate::Resettable for PEIES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
