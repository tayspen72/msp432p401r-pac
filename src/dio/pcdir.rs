#[doc = "Register `PCDIR` reader"]
pub struct R(crate::R<PCDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCDIR` writer"]
pub struct W(crate::W<PCDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCDIR_SPEC>;
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
impl From<crate::W<PCDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5DIR` reader - Port 5 Direction"]
pub struct P5DIR_R(crate::FieldReader<u8, u8>);
impl P5DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5DIR` writer - Port 5 Direction"]
pub struct P5DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P5DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6DIR` reader - Port 6 Direction"]
pub struct P6DIR_R(crate::FieldReader<u8, u8>);
impl P6DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6DIR` writer - Port 6 Direction"]
pub struct P6DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P6DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&self) -> P5DIR_R {
        P5DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&self) -> P6DIR_R {
        P6DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Direction"]
    #[inline(always)]
    pub fn p5dir(&mut self) -> P5DIR_W {
        P5DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Direction"]
    #[inline(always)]
    pub fn p6dir(&mut self) -> P6DIR_W {
        P6DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcdir](index.html) module"]
pub struct PCDIR_SPEC;
impl crate::RegisterSpec for PCDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcdir::R](R) reader structure"]
impl crate::Readable for PCDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcdir::W](W) writer structure"]
impl crate::Writable for PCDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCDIR to value 0"]
impl crate::Resettable for PCDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
