#[doc = "Register `PEOUT` reader"]
pub struct R(crate::R<PEOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PEOUT` writer"]
pub struct W(crate::W<PEOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PEOUT_SPEC>;
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
impl From<crate::W<PEOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PEOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9OUT` reader - Port 9 Output"]
pub struct P9OUT_R(crate::FieldReader<u8, u8>);
impl P9OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9OUT` writer - Port 9 Output"]
pub struct P9OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P9OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10OUT` reader - Port 10 Output"]
pub struct P10OUT_R(crate::FieldReader<u8, u8>);
impl P10OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10OUT` writer - Port 10 Output"]
pub struct P10OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P10OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&self) -> P9OUT_R {
        P9OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&self) -> P10OUT_R {
        P10OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Output"]
    #[inline(always)]
    pub fn p9out(&mut self) -> P9OUT_W {
        P9OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Output"]
    #[inline(always)]
    pub fn p10out(&mut self) -> P10OUT_W {
        P10OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peout](index.html) module"]
pub struct PEOUT_SPEC;
impl crate::RegisterSpec for PEOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [peout::R](R) reader structure"]
impl crate::Readable for PEOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peout::W](W) writer structure"]
impl crate::Writable for PEOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PEOUT to value 0"]
impl crate::Resettable for PEOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
