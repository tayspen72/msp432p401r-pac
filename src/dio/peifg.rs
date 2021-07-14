#[doc = "Register `PEIFG` reader"]
pub struct R(crate::R<PEIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEIFG` writer"]
pub struct W(crate::W<PEIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEIFG_SPEC>;
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
impl From<crate::W<PEIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9IFG` reader - Port 9 Interrupt Flag"]
pub struct P9IFG_R(crate::FieldReader<u8, u8>);
impl P9IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9IFG` writer - Port 9 Interrupt Flag"]
pub struct P9IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P9IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10IFG` reader - Port 10 Interrupt Flag"]
pub struct P10IFG_R(crate::FieldReader<u8, u8>);
impl P10IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10IFG` writer - Port 10 Interrupt Flag"]
pub struct P10IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P10IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&self) -> P9IFG_R {
        P9IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&self) -> P10IFG_R {
        P10IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Interrupt Flag"]
    #[inline(always)]
    pub fn p9ifg(&mut self) -> P9IFG_W {
        P9IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Interrupt Flag"]
    #[inline(always)]
    pub fn p10ifg(&mut self) -> P10IFG_W {
        P10IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peifg](index.html) module"]
pub struct PEIFG_SPEC;
impl crate::RegisterSpec for PEIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peifg::R](R) reader structure"]
impl crate::Readable for PEIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peifg::W](W) writer structure"]
impl crate::Writable for PEIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEIFG to value 0"]
impl crate::Resettable for PEIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
