#[doc = "Register `PCOUT` reader"]
pub struct R(crate::R<PCOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCOUT` writer"]
pub struct W(crate::W<PCOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCOUT_SPEC>;
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
impl From<crate::W<PCOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P5OUT` reader - Port 5 Output"]
pub struct P5OUT_R(crate::FieldReader<u8, u8>);
impl P5OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P5OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P5OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P5OUT` writer - Port 5 Output"]
pub struct P5OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P5OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P6OUT` reader - Port 6 Output"]
pub struct P6OUT_R(crate::FieldReader<u8, u8>);
impl P6OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P6OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P6OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P6OUT` writer - Port 6 Output"]
pub struct P6OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P6OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&self) -> P5OUT_R {
        P5OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&self) -> P6OUT_R {
        P6OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 5 Output"]
    #[inline(always)]
    pub fn p5out(&mut self) -> P5OUT_W {
        P5OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 6 Output"]
    #[inline(always)]
    pub fn p6out(&mut self) -> P6OUT_W {
        P6OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port C Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcout](index.html) module"]
pub struct PCOUT_SPEC;
impl crate::RegisterSpec for PCOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pcout::R](R) reader structure"]
impl crate::Readable for PCOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcout::W](W) writer structure"]
impl crate::Writable for PCOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCOUT to value 0"]
impl crate::Resettable for PCOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
