#[doc = "Register `PAIE` reader"]
pub struct R(crate::R<PAIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAIE` writer"]
pub struct W(crate::W<PAIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAIE_SPEC>;
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
impl From<crate::W<PAIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IE` reader - Port 1 Interrupt Enable"]
pub struct P1IE_R(crate::FieldReader<u8, u8>);
impl P1IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1IE` writer - Port 1 Interrupt Enable"]
pub struct P1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P1IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P2IE` reader - Port 2 Interrupt Enable"]
pub struct P2IE_R(crate::FieldReader<u8, u8>);
impl P2IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2IE` writer - Port 2 Interrupt Enable"]
pub struct P2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P2IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&self) -> P1IE_R {
        P1IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&self) -> P2IE_R {
        P2IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Interrupt Enable"]
    #[inline(always)]
    pub fn p1ie(&mut self) -> P1IE_W {
        P1IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Interrupt Enable"]
    #[inline(always)]
    pub fn p2ie(&mut self) -> P2IE_W {
        P2IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paie](index.html) module"]
pub struct PAIE_SPEC;
impl crate::RegisterSpec for PAIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paie::R](R) reader structure"]
impl crate::Readable for PAIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paie::W](W) writer structure"]
impl crate::Writable for PAIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAIE to value 0"]
impl crate::Resettable for PAIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
