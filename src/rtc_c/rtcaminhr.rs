#[doc = "Register `RTCAMINHR` reader"]
pub struct R(crate::R<RTCAMINHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCAMINHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCAMINHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCAMINHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCAMINHR` writer"]
pub struct W(crate::W<RTCAMINHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCAMINHR_SPEC>;
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
impl From<crate::W<RTCAMINHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCAMINHR_SPEC>) -> Self {
        W(writer)
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
        self.w.bits = (self.w.bits & !0x3f) | (value as u16 & 0x3f);
        self.w
    }
}
#[doc = "Field `MINAE` reader - Alarm enable"]
pub struct MINAE_R(crate::FieldReader<bool, bool>);
impl MINAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MINAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MINAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MINAE` writer - Alarm enable"]
pub struct MINAE_W<'a> {
    w: &'a mut W,
}
impl<'a> MINAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u16 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `Hours` reader - Hours (0 to 23)"]
pub struct HOURS_R(crate::FieldReader<u8, u8>);
impl HOURS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOURS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOURS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Hours` writer - Hours (0 to 23)"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `HOURAE` reader - Alarm enable"]
pub struct HOURAE_R(crate::FieldReader<bool, bool>);
impl HOURAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOURAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOURAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOURAE` writer - Alarm enable"]
pub struct HOURAE_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&self) -> MINUTES_R {
        MINUTES_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&self) -> MINAE_R {
        MINAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&self) -> HOURAE_R {
        HOURAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes (0 to 59)"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MINUTES_W {
        MINUTES_W { w: self }
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn minae(&mut self) -> MINAE_W {
        MINAE_W { w: self }
    }
    #[doc = "Bits 8:12 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn hourae(&mut self) -> HOURAE_W {
        HOURAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCMINHR - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcaminhr](index.html) module"]
pub struct RTCAMINHR_SPEC;
impl crate::RegisterSpec for RTCAMINHR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcaminhr::R](R) reader structure"]
impl crate::Readable for RTCAMINHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcaminhr::W](W) writer structure"]
impl crate::Writable for RTCAMINHR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCAMINHR to value 0"]
impl crate::Resettable for RTCAMINHR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
