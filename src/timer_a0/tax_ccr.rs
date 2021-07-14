#[doc = "Register `TAxCCR[%s]` reader"]
pub struct R(crate::R<TAXCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAXCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAXCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAXCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAxCCR[%s]` writer"]
pub struct W(crate::W<TAXCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAXCCR_SPEC>;
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
impl From<crate::W<TAXCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAXCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAxR` reader - TimerA register"]
pub struct TAXR_R(crate::FieldReader<u16, u16>);
impl TAXR_R {
    pub(crate) fn new(bits: u16) -> Self {
        TAXR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAXR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAxR` writer - TimerA register"]
pub struct TAXR_W<'a> {
    w: &'a mut W,
}
impl<'a> TAXR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&self) -> TAXR_R {
        TAXR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TimerA register"]
    #[inline(always)]
    pub fn tax_r(&mut self) -> TAXR_W {
        TAXR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer_A Capture/Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tax_ccr](index.html) module"]
pub struct TAXCCR_SPEC;
impl crate::RegisterSpec for TAXCCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tax_ccr::R](R) reader structure"]
impl crate::Readable for TAXCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tax_ccr::W](W) writer structure"]
impl crate::Writable for TAXCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAxCCR[%s]
to value 0"]
impl crate::Resettable for TAXCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
