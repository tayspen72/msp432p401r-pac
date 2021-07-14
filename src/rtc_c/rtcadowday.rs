#[doc = "Register `RTCADOWDAY` reader"]
pub struct R(crate::R<RTCADOWDAY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCADOWDAY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCADOWDAY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCADOWDAY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCADOWDAY` writer"]
pub struct W(crate::W<RTCADOWDAY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCADOWDAY_SPEC>;
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
impl From<crate::W<RTCADOWDAY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCADOWDAY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DayofWeek` reader - Day of week (0 to 6)"]
pub struct DAYOFWEEK_R(crate::FieldReader<u8, u8>);
impl DAYOFWEEK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAYOFWEEK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAYOFWEEK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DayofWeek` writer - Day of week (0 to 6)"]
pub struct DAYOFWEEK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOFWEEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Field `DOWAE` reader - Alarm enable"]
pub struct DOWAE_R(crate::FieldReader<bool, bool>);
impl DOWAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DOWAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOWAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWAE` writer - Alarm enable"]
pub struct DOWAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWAE_W<'a> {
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
#[doc = "Field `DayofMonth` reader - Day of month (1 to 28, 29, 30, 31)"]
pub struct DAYOFMONTH_R(crate::FieldReader<u8, u8>);
impl DAYOFMONTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAYOFMONTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAYOFMONTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DayofMonth` writer - Day of month (1 to 28, 29, 30, 31)"]
pub struct DAYOFMONTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOFMONTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u16 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DAYAE` reader - Alarm enable"]
pub struct DAYAE_R(crate::FieldReader<bool, bool>);
impl DAYAE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DAYAE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAYAE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DAYAE` writer - Alarm enable"]
pub struct DAYAE_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYAE_W<'a> {
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
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&self) -> DAYOFWEEK_R {
        DAYOFWEEK_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&self) -> DOWAE_R {
        DOWAE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&self) -> DAYOFMONTH_R {
        DAYOFMONTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&self) -> DAYAE_R {
        DAYAE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&mut self) -> DAYOFWEEK_W {
        DAYOFWEEK_W { w: self }
    }
    #[doc = "Bit 7 - Alarm enable"]
    #[inline(always)]
    pub fn dowae(&mut self) -> DOWAE_W {
        DOWAE_W { w: self }
    }
    #[doc = "Bits 8:12 - Day of month (1 to 28, 29, 30, 31)"]
    #[inline(always)]
    pub fn dayof_month(&mut self) -> DAYOFMONTH_W {
        DAYOFMONTH_W { w: self }
    }
    #[doc = "Bit 15 - Alarm enable"]
    #[inline(always)]
    pub fn dayae(&mut self) -> DAYAE_W {
        DAYAE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCADOWDAY - Hexadecimal Format\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcadowday](index.html) module"]
pub struct RTCADOWDAY_SPEC;
impl crate::RegisterSpec for RTCADOWDAY_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcadowday::R](R) reader structure"]
impl crate::Readable for RTCADOWDAY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcadowday::W](W) writer structure"]
impl crate::Writable for RTCADOWDAY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCADOWDAY to value 0"]
impl crate::Resettable for RTCADOWDAY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
