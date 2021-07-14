#[doc = "Register `PCSEL0` reader"]
pub struct R(crate::R<PCSEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCSEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCSEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCSEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCSEL0` writer"]
pub struct W(crate::W<PCSEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCSEL0_SPEC>;
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
impl From<crate::W<PCSEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCSEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5SEL0` reader - Port 5 Select 0"]
pub struct P5SEL0_R(crate::FieldReader<u8, u8>);
impl P5SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5SEL0` writer - Port 5 Select 0"]
pub struct P5SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P5SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6SEL0` reader - Port 6 Select 0"]
pub struct P6SEL0_R(crate::FieldReader<u8, u8>);
impl P6SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6SEL0` writer - Port 6 Select 0"]
pub struct P6SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P6SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&self) -> P5SEL0_R {
        P5SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&self) -> P6SEL0_R {
        P6SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Select 0"]
    #[inline(always)]
    pub fn p5sel0(&mut self) -> P5SEL0_W {
        P5SEL0_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Select 0"]
    #[inline(always)]
    pub fn p6sel0(&mut self) -> P6SEL0_W {
        P6SEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcsel0](index.html) module"]
pub struct PCSEL0_SPEC;
impl crate::RegisterSpec for PCSEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcsel0::R](R) reader structure"]
impl crate::Readable for PCSEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcsel0::W](W) writer structure"]
impl crate::Writable for PCSEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCSEL0 to value 0"]
impl crate::Resettable for PCSEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
