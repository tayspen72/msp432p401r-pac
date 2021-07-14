#[doc = "Register `PCIFG` reader"]
pub struct R(crate::R<PCIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCIFG` writer"]
pub struct W(crate::W<PCIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCIFG_SPEC>;
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
impl From<crate::W<PCIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5IFG` reader - Port 5 Interrupt Flag"]
pub struct P5IFG_R(crate::FieldReader<u8, u8>);
impl P5IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5IFG` writer - Port 5 Interrupt Flag"]
pub struct P5IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P5IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6IFG` reader - Port 6 Interrupt Flag"]
pub struct P6IFG_R(crate::FieldReader<u8, u8>);
impl P6IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6IFG` writer - Port 6 Interrupt Flag"]
pub struct P6IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P6IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&self) -> P5IFG_R {
        P5IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&self) -> P6IFG_R {
        P6IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Interrupt Flag"]
    #[inline(always)]
    pub fn p5ifg(&mut self) -> P5IFG_W {
        P5IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Interrupt Flag"]
    #[inline(always)]
    pub fn p6ifg(&mut self) -> P6IFG_W {
        P6IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcifg](index.html) module"]
pub struct PCIFG_SPEC;
impl crate::RegisterSpec for PCIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcifg::R](R) reader structure"]
impl crate::Readable for PCIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcifg::W](W) writer structure"]
impl crate::Writable for PCIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCIFG to value 0"]
impl crate::Resettable for PCIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
