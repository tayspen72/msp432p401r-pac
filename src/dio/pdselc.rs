#[doc = "Register `PDSELC` reader"]
pub struct R(crate::R<PDSELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSELC` writer"]
pub struct W(crate::W<PDSELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSELC_SPEC>;
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
impl From<crate::W<PDSELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7SELC` reader - Port 7 Complement Select"]
pub struct P7SELC_R(crate::FieldReader<u8, u8>);
impl P7SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SELC` writer - Port 7 Complement Select"]
pub struct P7SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P8SELC` reader - Port 8 Complement Select"]
pub struct P8SELC_R(crate::FieldReader<u8, u8>);
impl P8SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SELC` writer - Port 8 Complement Select"]
pub struct P8SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&self) -> P7SELC_R {
        P7SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&self) -> P8SELC_R {
        P8SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Complement Select"]
    #[inline(always)]
    pub fn p7selc(&mut self) -> P7SELC_W {
        P7SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Complement Select"]
    #[inline(always)]
    pub fn p8selc(&mut self) -> P8SELC_W {
        P8SELC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdselc](index.html) module"]
pub struct PDSELC_SPEC;
impl crate::RegisterSpec for PDSELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdselc::R](R) reader structure"]
impl crate::Readable for PDSELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdselc::W](W) writer structure"]
impl crate::Writable for PDSELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSELC to value 0"]
impl crate::Resettable for PDSELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
