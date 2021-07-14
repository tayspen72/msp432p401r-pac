#[doc = "Register `PJDS` reader"]
pub struct R(crate::R<PJDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PJDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PJDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PJDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PJDS` writer"]
pub struct W(crate::W<PJDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PJDS_SPEC>;
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
impl From<crate::W<PJDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PJDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PJDS` reader - Port J Drive Strength"]
pub struct PJDS_R(crate::FieldReader<u16, u16>);
impl PJDS_R {
    pub(crate) fn new(bits: u16) -> Self {
        PJDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PJDS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PJDS` writer - Port J Drive Strength"]
pub struct PJDS_W<'a> {
    w: &'a mut W,
}
impl<'a> PJDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&self) -> PJDS_R {
        PJDS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Port J Drive Strength"]
    #[inline(always)]
    pub fn pjds(&mut self) -> PJDS_W {
        PJDS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port J Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pjds](index.html) module"]
pub struct PJDS_SPEC;
impl crate::RegisterSpec for PJDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pjds::R](R) reader structure"]
impl crate::Readable for PJDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pjds::W](W) writer structure"]
impl crate::Writable for PJDS_SPEC {
    type Writer = W;
}
