#[doc = "Register `PAREN` reader"]
pub struct R(crate::R<PAREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAREN` writer"]
pub struct W(crate::W<PAREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAREN_SPEC>;
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
impl From<crate::W<PAREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1REN` reader - Port 1 Resistor Enable"]
pub struct P1REN_R(crate::FieldReader<u8, u8>);
impl P1REN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P1REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1REN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1REN` writer - Port 1 Resistor Enable"]
pub struct P1REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P1REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P2REN` reader - Port 2 Resistor Enable"]
pub struct P2REN_R(crate::FieldReader<u8, u8>);
impl P2REN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2REN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2REN` writer - Port 2 Resistor Enable"]
pub struct P2REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P2REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&self) -> P1REN_R {
        P1REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&self) -> P2REN_R {
        P2REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Resistor Enable"]
    #[inline(always)]
    pub fn p1ren(&mut self) -> P1REN_W {
        P1REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Resistor Enable"]
    #[inline(always)]
    pub fn p2ren(&mut self) -> P2REN_W {
        P2REN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paren](index.html) module"]
pub struct PAREN_SPEC;
impl crate::RegisterSpec for PAREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paren::R](R) reader structure"]
impl crate::Readable for PAREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paren::W](W) writer structure"]
impl crate::Writable for PAREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAREN to value 0"]
impl crate::Resettable for PAREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
