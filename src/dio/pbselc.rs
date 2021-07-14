#[doc = "Register `PBSELC` reader"]
pub struct R(crate::R<PBSELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBSELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBSELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBSELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBSELC` writer"]
pub struct W(crate::W<PBSELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBSELC_SPEC>;
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
impl From<crate::W<PBSELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBSELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3SELC` reader - Port 3 Complement Select"]
pub struct P3SELC_R(crate::FieldReader<u8, u8>);
impl P3SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3SELC` writer - Port 3 Complement Select"]
pub struct P3SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P3SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4SELC` reader - Port 4 Complement Select"]
pub struct P4SELC_R(crate::FieldReader<u8, u8>);
impl P4SELC_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4SELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4SELC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4SELC` writer - Port 4 Complement Select"]
pub struct P4SELC_W<'a> {
    w: &'a mut W,
}
impl<'a> P4SELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&self) -> P3SELC_R {
        P3SELC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&self) -> P4SELC_R {
        P4SELC_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Complement Select"]
    #[inline(always)]
    pub fn p3selc(&mut self) -> P3SELC_W {
        P3SELC_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Complement Select"]
    #[inline(always)]
    pub fn p4selc(&mut self) -> P4SELC_W {
        P4SELC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbselc](index.html) module"]
pub struct PBSELC_SPEC;
impl crate::RegisterSpec for PBSELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbselc::R](R) reader structure"]
impl crate::Readable for PBSELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbselc::W](W) writer structure"]
impl crate::Writable for PBSELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBSELC to value 0"]
impl crate::Resettable for PBSELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
