#[doc = "Register `PAOUT` reader"]
pub struct R(crate::R<PAOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PAOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PAOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PAOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PAOUT` writer"]
pub struct W(crate::W<PAOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PAOUT_SPEC>;
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
impl From<crate::W<PAOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PAOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2OUT` reader - Port 2 Output"]
pub struct P2OUT_R(crate::FieldReader<u8, u8>);
impl P2OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2OUT` writer - Port 2 Output"]
pub struct P2OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P2OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `P1OUT` reader - Port 1 Output"]
pub struct P1OUT_R(crate::FieldReader<u8, u8>);
impl P1OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P1OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1OUT` writer - Port 1 Output"]
pub struct P1OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P1OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&self) -> P2OUT_R {
        P2OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&self) -> P1OUT_R {
        P1OUT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 8:15 - Port 2 Output"]
    #[inline(always)]
    pub fn p2out(&mut self) -> P2OUT_W {
        P2OUT_W { w: self }
    }
    #[doc = "Bits 0:7 - Port 1 Output"]
    #[inline(always)]
    pub fn p1out(&mut self) -> P1OUT_W {
        P1OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [paout](index.html) module"]
pub struct PAOUT_SPEC;
impl crate::RegisterSpec for PAOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [paout::R](R) reader structure"]
impl crate::Readable for PAOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [paout::W](W) writer structure"]
impl crate::Writable for PAOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PAOUT to value 0"]
impl crate::Resettable for PAOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
