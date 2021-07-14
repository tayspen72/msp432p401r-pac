#[doc = "Register `PEIE` reader"]
pub struct R(crate::R<PEIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEIE` writer"]
pub struct W(crate::W<PEIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEIE_SPEC>;
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
impl From<crate::W<PEIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9IE` reader - Port 9 Interrupt Enable"]
pub struct P9IE_R(crate::FieldReader<u8, u8>);
impl P9IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9IE` writer - Port 9 Interrupt Enable"]
pub struct P9IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10IE` reader - Port 10 Interrupt Enable"]
pub struct P10IE_R(crate::FieldReader<u8, u8>);
impl P10IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10IE` writer - Port 10 Interrupt Enable"]
pub struct P10IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&self) -> P9IE_R {
        P9IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&self) -> P10IE_R {
        P10IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Enable"]
    #[inline(always)]
    pub fn p9ie(&mut self) -> P9IE_W {
        P9IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Enable"]
    #[inline(always)]
    pub fn p10ie(&mut self) -> P10IE_W {
        P10IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peie](index.html) module"]
pub struct PEIE_SPEC;
impl crate::RegisterSpec for PEIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peie::R](R) reader structure"]
impl crate::Readable for PEIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peie::W](W) writer structure"]
impl crate::Writable for PEIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEIE to value 0"]
impl crate::Resettable for PEIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
