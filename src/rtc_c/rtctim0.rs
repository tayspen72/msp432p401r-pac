#[doc = "Register `RTCTIM0` reader"]
pub struct R(crate::R<RTCTIM0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIM0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTIM0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTIM0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTIM0` writer"]
pub struct W(crate::W<RTCTIM0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTIM0_SPEC>;
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
impl From<crate::W<RTCTIM0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCTIM0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Seconds` reader - Seconds (0 to 59)"]
pub struct SECONDS_R(crate::FieldReader<u8, u8>);
impl SECONDS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SECONDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECONDS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Seconds` writer - Seconds (0 to 59)"]
pub struct SECONDS_W<'a> {
    w: &'a mut W,
}
impl<'a> SECONDS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
#[doc = "Field `Minutes` reader - Minutes (0 to 59)"]
pub struct MINUTES_R(crate::FieldReader<u8, u8>);
impl MINUTES_R {
    pub(crate) fn new(bits: u8) -> Self {
        MINUTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINUTES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Minutes` writer - Minutes (0 to 59)"]
pub struct MINUTES_W<'a> {
    w: &'a mut W,
}
impl<'a> MINUTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u16 & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&self) -> SECONDS_R {
        SECONDS_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds (0 to 59)"]
    #[inline(always)]
    pub fn seconds(&mut self) -> SECONDS_W {
        SECONDS_W { w: self }
    }
    #[doc = "Bits 8:13 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCTIM0 Register Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim0](index.html) module"]
pub struct RTCTIM0_SPEC;
impl crate::RegisterSpec for RTCTIM0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctim0::R](R) reader structure"]
impl crate::Readable for RTCTIM0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctim0::W](W) writer structure"]
impl crate::Writable for RTCTIM0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTIM0 to value 0"]
impl crate::Resettable for RTCTIM0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
