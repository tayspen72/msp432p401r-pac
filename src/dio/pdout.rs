#[doc = "Register `PDOUT` reader"]
pub struct R(crate::R<PDOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDOUT` writer"]
pub struct W(crate::W<PDOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDOUT_SPEC>;
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
impl From<crate::W<PDOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7OUT` reader - Port 7 Output"]
pub struct P7OUT_R(crate::FieldReader<u8, u8>);
impl P7OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7OUT` writer - Port 7 Output"]
pub struct P7OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P7OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P8OUT` reader - Port 8 Output"]
pub struct P8OUT_R(crate::FieldReader<u8, u8>);
impl P8OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8OUT` writer - Port 8 Output"]
pub struct P8OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P8OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&self) -> P7OUT_R {
        P7OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&self) -> P8OUT_R {
        P8OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Output"]
    #[inline(always)]
    pub fn p7out(&mut self) -> P7OUT_W {
        P7OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Output"]
    #[inline(always)]
    pub fn p8out(&mut self) -> P8OUT_W {
        P8OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdout](index.html) module"]
pub struct PDOUT_SPEC;
impl crate::RegisterSpec for PDOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdout::R](R) reader structure"]
impl crate::Readable for PDOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdout::W](W) writer structure"]
impl crate::Writable for PDOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDOUT to value 0"]
impl crate::Resettable for PDOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
