#[doc = "Register `UCAxBRW` reader"]
pub struct R(crate::R<UCAXBRW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXBRW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXBRW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXBRW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxBRW` writer"]
pub struct W(crate::W<UCAXBRW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXBRW_SPEC>;
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
impl From<crate::W<UCAXBRW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXBRW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCBR` reader - Clock prescaler setting of the Baud rate generator"]
pub struct UCBR_R(crate::FieldReader<u16, u16>);
impl UCBR_R {
    pub(crate) fn new(bits: u16) -> Self {
        UCBR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCBR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBR` writer - Clock prescaler setting of the Baud rate generator"]
pub struct UCBR_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock prescaler setting of the Baud rate generator"]
    #[inline(always)]
    pub fn ucbr(&self) -> UCBR_R {
        UCBR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock prescaler setting of the Baud rate generator"]
    #[inline(always)]
    pub fn ucbr(&mut self) -> UCBR_W {
        UCBR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Baud Rate Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_brw](index.html) module"]
pub struct UCAXBRW_SPEC;
impl crate::RegisterSpec for UCAXBRW_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_brw::R](R) reader structure"]
impl crate::Readable for UCAXBRW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_brw::W](W) writer structure"]
impl crate::Writable for UCAXBRW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxBRW to value 0"]
impl crate::Resettable for UCAXBRW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
