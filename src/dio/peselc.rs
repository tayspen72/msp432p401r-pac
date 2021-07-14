#[doc = "Register `PESELC` reader"]
pub struct R(crate::R<PESELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PESELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PESELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PESELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PESELC` writer"]
pub struct W(crate::W<PESELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PESELC_SPEC>;
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
impl From<crate::W<PESELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PESELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9SELC` reader - Port 9 Complement Select"]
pub struct P9SELC_R(crate::FieldReader<u8, u8>);
impl P9SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9SELC` writer - Port 9 Complement Select"]
pub struct P9SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P9SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10SELC` reader - Port 10 Complement Select"]
pub struct P10SELC_R(crate::FieldReader<u8, u8>);
impl P10SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10SELC` writer - Port 10 Complement Select"]
pub struct P10SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P10SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&self) -> P9SELC_R {
        P9SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&self) -> P10SELC_R {
        P10SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Complement Select"]
    #[inline(always)]
    pub fn p9selc(&mut self) -> P9SELC_W {
        P9SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Complement Select"]
    #[inline(always)]
    pub fn p10selc(&mut self) -> P10SELC_W {
        P10SELC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peselc](index.html) module"]
pub struct PESELC_SPEC;
impl crate::RegisterSpec for PESELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peselc::R](R) reader structure"]
impl crate::Readable for PESELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peselc::W](W) writer structure"]
impl crate::Writable for PESELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PESELC to value 0"]
impl crate::Resettable for PESELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
