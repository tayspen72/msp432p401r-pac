#[doc = "Register `PJSELC` reader"]
pub struct R(crate::R<PJSELC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSELC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJSELC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJSELC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSELC` writer"]
pub struct W(crate::W<PJSELC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSELC_SPEC>;
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
impl From<crate::W<PJSELC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJSELC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSELC` reader - Port J Complement Select"]
pub struct PJSELC_R(crate::FieldReader<u16, u16>);
impl PJSELC_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJSELC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSELC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSELC` writer - Port J Complement Select"]
pub struct PJSELC_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSELC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&self) -> PJSELC_R {
        PJSELC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Complement Select"]
    #[inline(always)]
    pub fn pjselc(&mut self) -> PJSELC_W {
        PJSELC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Complement Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjselc](index.html) module"]
pub struct PJSELC_SPEC;
impl crate::RegisterSpec for PJSELC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjselc::R](R) reader structure"]
impl crate::Readable for PJSELC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjselc::W](W) writer structure"]
impl crate::Writable for PJSELC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSELC to value 0"]
impl crate::Resettable for PJSELC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
