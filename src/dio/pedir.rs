#[doc = "Register `PEDIR` reader"]
pub struct R(crate::R<PEDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEDIR` writer"]
pub struct W(crate::W<PEDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEDIR_SPEC>;
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
impl From<crate::W<PEDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9DIR` reader - Port 9 Direction"]
pub struct P9DIR_R(crate::FieldReader<u8, u8>);
impl P9DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9DIR` writer - Port 9 Direction"]
pub struct P9DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P9DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10DIR` reader - Port 10 Direction"]
pub struct P10DIR_R(crate::FieldReader<u8, u8>);
impl P10DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10DIR` writer - Port 10 Direction"]
pub struct P10DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P10DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&self) -> P9DIR_R {
        P9DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&self) -> P10DIR_R {
        P10DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Direction"]
    #[inline(always)]
    pub fn p9dir(&mut self) -> P9DIR_W {
        P9DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Direction"]
    #[inline(always)]
    pub fn p10dir(&mut self) -> P10DIR_W {
        P10DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pedir](index.html) module"]
pub struct PEDIR_SPEC;
impl crate::RegisterSpec for PEDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pedir::R](R) reader structure"]
impl crate::Readable for PEDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pedir::W](W) writer structure"]
impl crate::Writable for PEDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEDIR to value 0"]
impl crate::Resettable for PEDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
