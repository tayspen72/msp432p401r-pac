#[doc = "Register `PJDIR` reader"]
pub struct R(crate::R<PJDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDIR` writer"]
pub struct W(crate::W<PJDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDIR_SPEC>;
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
impl From<crate::W<PJDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDIR` reader - Port J Direction"]
pub struct PJDIR_R(crate::FieldReader<u16, u16>);
impl PJDIR_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDIR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDIR` writer - Port J Direction"]
pub struct PJDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&self) -> PJDIR_R {
        PJDIR_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Direction"]
    #[inline(always)]
    pub fn pjdir(&mut self) -> PJDIR_W {
        PJDIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjdir](index.html) module"]
pub struct PJDIR_SPEC;
impl crate::RegisterSpec for PJDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjdir::R](R) reader structure"]
impl crate::Readable for PJDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjdir::W](W) writer structure"]
impl crate::Writable for PJDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PJDIR to value 0"]
impl crate::Resettable for PJDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
