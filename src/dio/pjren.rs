#[doc = "Register `PJREN` reader"]
pub struct R(crate::R<PJREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJREN` writer"]
pub struct W(crate::W<PJREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJREN_SPEC>;
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
impl From<crate::W<PJREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJREN` reader - Port J Resistor Enable"]
pub struct PJREN_R(crate::FieldReader<u16, u16>);
impl PJREN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJREN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJREN` writer - Port J Resistor Enable"]
pub struct PJREN_W<'a> {
    w: &'a mut W,
}
impl<'a> PJREN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&self) -> PJREN_R {
        PJREN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Resistor Enable"]
    #[inline(always)]
    pub fn pjren(&mut self) -> PJREN_W {
        PJREN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjren](index.html) module"]
pub struct PJREN_SPEC;
impl crate::RegisterSpec for PJREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjren::R](R) reader structure"]
impl crate::Readable for PJREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjren::W](W) writer structure"]
impl crate::Writable for PJREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJREN to value 0"]
impl crate::Resettable for PJREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
