#[doc = "Register `PJSEL1` reader"]
pub struct R(crate::R<PJSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJSEL1` writer"]
pub struct W(crate::W<PJSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJSEL1_SPEC>;
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
impl From<crate::W<PJSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJSEL1` reader - Port J Select 1"]
pub struct PJSEL1_R(crate::FieldReader<u16, u16>);
impl PJSEL1_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJSEL1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJSEL1` writer - Port J Select 1"]
pub struct PJSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> PJSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&self) -> PJSEL1_R {
        PJSEL1_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Select 1"]
    #[inline(always)]
    pub fn pjsel1(&mut self) -> PJSEL1_W {
        PJSEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjsel1](index.html) module"]
pub struct PJSEL1_SPEC;
impl crate::RegisterSpec for PJSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjsel1::R](R) reader structure"]
impl crate::Readable for PJSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjsel1::W](W) writer structure"]
impl crate::Writable for PJSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJSEL1 to value 0"]
impl crate::Resettable for PJSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
