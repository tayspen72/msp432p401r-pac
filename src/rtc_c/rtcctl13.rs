#[doc = "Register `RTCCTL13` reader"]
pub struct R(crate::R<RTCCTL13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL13` writer"]
pub struct W(crate::W<RTCCTL13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL13_SPEC>;
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
impl From<crate::W<RTCCTL13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Real-time clock time event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCTEV_A {
    #[doc = "0: Minute changed"]
    RTCTEV_0 = 0,
    #[doc = "1: Hour changed"]
    RTCTEV_1 = 1,
    #[doc = "2: Every day at midnight (00:00)"]
    RTCTEV_2 = 2,
    #[doc = "3: Every day at noon (12:00)"]
    RTCTEV_3 = 3,
}
impl From<RTCTEV_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCTEV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCTEV` reader - Real-time clock time event"]
pub struct RTCTEV_R(crate::FieldReader<u8, RTCTEV_A>);
impl RTCTEV_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCTEV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCTEV_A {
        match self.bits {
            0 => RTCTEV_A::RTCTEV_0,
            1 => RTCTEV_A::RTCTEV_1,
            2 => RTCTEV_A::RTCTEV_2,
            3 => RTCTEV_A::RTCTEV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCTEV_0`"]
    #[inline(always)]
    pub fn is_rtctev_0(&self) -> bool {
        **self == RTCTEV_A::RTCTEV_0
    }
    #[doc = "Checks if the value of the field is `RTCTEV_1`"]
    #[inline(always)]
    pub fn is_rtctev_1(&self) -> bool {
        **self == RTCTEV_A::RTCTEV_1
    }
    #[doc = "Checks if the value of the field is `RTCTEV_2`"]
    #[inline(always)]
    pub fn is_rtctev_2(&self) -> bool {
        **self == RTCTEV_A::RTCTEV_2
    }
    #[doc = "Checks if the value of the field is `RTCTEV_3`"]
    #[inline(always)]
    pub fn is_rtctev_3(&self) -> bool {
        **self == RTCTEV_A::RTCTEV_3
    }
}
impl core::ops::Deref for RTCTEV_R {
    type Target = crate::FieldReader<u8, RTCTEV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCTEV` writer - Real-time clock time event"]
pub struct RTCTEV_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCTEV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCTEV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Minute changed"]
    #[inline(always)]
    pub fn rtctev_0(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_0)
    }
    #[doc = "Hour changed"]
    #[inline(always)]
    pub fn rtctev_1(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_1)
    }
    #[doc = "Every day at midnight (00:00)"]
    #[inline(always)]
    pub fn rtctev_2(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_2)
    }
    #[doc = "Every day at noon (12:00)"]
    #[inline(always)]
    pub fn rtctev_3(self) -> &'a mut W {
        self.variant(RTCTEV_A::RTCTEV_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
#[doc = "Real-time clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCSSEL_A {
    #[doc = "0: BCLK"]
    RTCSSEL_0 = 0,
}
impl From<RTCSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCSSEL` reader - Real-time clock source select"]
pub struct RTCSSEL_R(crate::FieldReader<u8, RTCSSEL_A>);
impl RTCSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCSSEL_A> {
        match self.bits {
            0 => Some(RTCSSEL_A::RTCSSEL_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTCSSEL_0`"]
    #[inline(always)]
    pub fn is_rtcssel_0(&self) -> bool {
        **self == RTCSSEL_A::RTCSSEL_0
    }
}
impl core::ops::Deref for RTCSSEL_R {
    type Target = crate::FieldReader<u8, RTCSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCSSEL` writer - Real-time clock source select"]
pub struct RTCSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn rtcssel_0(self) -> &'a mut W {
        self.variant(RTCSSEL_A::RTCSSEL_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u16 & 0x03) << 2);
        self.w
    }
}
#[doc = "Real-time clock ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCRDY_A {
    #[doc = "0: RTC time values in transition"]
    RTCRDY_0 = 0,
    #[doc = "1: RTC time values safe for reading. This bit indicates when the real-time clock time values are safe for reading."]
    RTCRDY_1 = 1,
}
impl From<RTCRDY_A> for bool {
    #[inline(always)]
    fn from(variant: RTCRDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCRDY` reader - Real-time clock ready"]
pub struct RTCRDY_R(crate::FieldReader<bool, RTCRDY_A>);
impl RTCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCRDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCRDY_A {
        match self.bits {
            false => RTCRDY_A::RTCRDY_0,
            true => RTCRDY_A::RTCRDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCRDY_0`"]
    #[inline(always)]
    pub fn is_rtcrdy_0(&self) -> bool {
        **self == RTCRDY_A::RTCRDY_0
    }
    #[doc = "Checks if the value of the field is `RTCRDY_1`"]
    #[inline(always)]
    pub fn is_rtcrdy_1(&self) -> bool {
        **self == RTCRDY_A::RTCRDY_1
    }
}
impl core::ops::Deref for RTCRDY_R {
    type Target = crate::FieldReader<bool, RTCRDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCMODE_A {
    #[doc = "1: Calendar mode. Always reads a value of 1."]
    RTCMODE_1 = 1,
}
impl From<RTCMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RTCMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCMODE` reader - "]
pub struct RTCMODE_R(crate::FieldReader<bool, RTCMODE_A>);
impl RTCMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCMODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCMODE_A> {
        match self.bits {
            true => Some(RTCMODE_A::RTCMODE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTCMODE_1`"]
    #[inline(always)]
    pub fn is_rtcmode_1(&self) -> bool {
        **self == RTCMODE_A::RTCMODE_1
    }
}
impl core::ops::Deref for RTCMODE_R {
    type Target = crate::FieldReader<bool, RTCMODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Real-time clock hold\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCHOLD_A {
    #[doc = "0: Real-time clock is operational"]
    RTCHOLD_0 = 0,
    #[doc = "1: When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    RTCHOLD_1 = 1,
}
impl From<RTCHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: RTCHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCHOLD` reader - Real-time clock hold"]
pub struct RTCHOLD_R(crate::FieldReader<bool, RTCHOLD_A>);
impl RTCHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCHOLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCHOLD_A {
        match self.bits {
            false => RTCHOLD_A::RTCHOLD_0,
            true => RTCHOLD_A::RTCHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCHOLD_0`"]
    #[inline(always)]
    pub fn is_rtchold_0(&self) -> bool {
        **self == RTCHOLD_A::RTCHOLD_0
    }
    #[doc = "Checks if the value of the field is `RTCHOLD_1`"]
    #[inline(always)]
    pub fn is_rtchold_1(&self) -> bool {
        **self == RTCHOLD_A::RTCHOLD_1
    }
}
impl core::ops::Deref for RTCHOLD_R {
    type Target = crate::FieldReader<bool, RTCHOLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCHOLD` writer - Real-time clock hold"]
pub struct RTCHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCHOLD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Real-time clock is operational"]
    #[inline(always)]
    pub fn rtchold_0(self) -> &'a mut W {
        self.variant(RTCHOLD_A::RTCHOLD_0)
    }
    #[doc = "When set, the calendar is stopped as well as the prescale counters, RT0PS and RT1PS are don't care"]
    #[inline(always)]
    pub fn rtchold_1(self) -> &'a mut W {
        self.variant(RTCHOLD_A::RTCHOLD_1)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u16 & 0x01) << 6);
        self.w
    }
}
#[doc = "Real-time clock BCD select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTCBCD_A {
    #[doc = "0: Binary (hexadecimal) code selected"]
    RTCBCD_0 = 0,
    #[doc = "1: Binary coded decimal (BCD) code selected"]
    RTCBCD_1 = 1,
}
impl From<RTCBCD_A> for bool {
    #[inline(always)]
    fn from(variant: RTCBCD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTCBCD` reader - Real-time clock BCD select"]
pub struct RTCBCD_R(crate::FieldReader<bool, RTCBCD_A>);
impl RTCBCD_R {
    pub(crate) fn new(bits: bool) -> Self {
        RTCBCD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCBCD_A {
        match self.bits {
            false => RTCBCD_A::RTCBCD_0,
            true => RTCBCD_A::RTCBCD_1,
        }
    }
    #[doc = "Checks if the value of the field is `RTCBCD_0`"]
    #[inline(always)]
    pub fn is_rtcbcd_0(&self) -> bool {
        **self == RTCBCD_A::RTCBCD_0
    }
    #[doc = "Checks if the value of the field is `RTCBCD_1`"]
    #[inline(always)]
    pub fn is_rtcbcd_1(&self) -> bool {
        **self == RTCBCD_A::RTCBCD_1
    }
}
impl core::ops::Deref for RTCBCD_R {
    type Target = crate::FieldReader<bool, RTCBCD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCBCD` writer - Real-time clock BCD select"]
pub struct RTCBCD_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCBCD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCBCD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Binary (hexadecimal) code selected"]
    #[inline(always)]
    pub fn rtcbcd_0(self) -> &'a mut W {
        self.variant(RTCBCD_A::RTCBCD_0)
    }
    #[doc = "Binary coded decimal (BCD) code selected"]
    #[inline(always)]
    pub fn rtcbcd_1(self) -> &'a mut W {
        self.variant(RTCBCD_A::RTCBCD_1)
    }
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
#[doc = "Real-time clock calibration frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RTCCALF_A {
    #[doc = "0: No frequency output to RTCCLK pin"]
    RTCCALF_0 = 0,
    #[doc = "1: 512 Hz"]
    RTCCALF_1 = 1,
    #[doc = "2: 256 Hz"]
    RTCCALF_2 = 2,
    #[doc = "3: 1 Hz"]
    RTCCALF_3 = 3,
}
impl From<RTCCALF_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCCALF_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCCALF` reader - Real-time clock calibration frequency"]
pub struct RTCCALF_R(crate::FieldReader<u8, RTCCALF_A>);
impl RTCCALF_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCCALF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCALF_A {
        match self.bits {
            0 => RTCCALF_A::RTCCALF_0,
            1 => RTCCALF_A::RTCCALF_1,
            2 => RTCCALF_A::RTCCALF_2,
            3 => RTCCALF_A::RTCCALF_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCCALF_0`"]
    #[inline(always)]
    pub fn is_rtccalf_0(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_0
    }
    #[doc = "Checks if the value of the field is `RTCCALF_1`"]
    #[inline(always)]
    pub fn is_rtccalf_1(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_1
    }
    #[doc = "Checks if the value of the field is `RTCCALF_2`"]
    #[inline(always)]
    pub fn is_rtccalf_2(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_2
    }
    #[doc = "Checks if the value of the field is `RTCCALF_3`"]
    #[inline(always)]
    pub fn is_rtccalf_3(&self) -> bool {
        **self == RTCCALF_A::RTCCALF_3
    }
}
impl core::ops::Deref for RTCCALF_R {
    type Target = crate::FieldReader<u8, RTCCALF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCCALF` writer - Real-time clock calibration frequency"]
pub struct RTCCALF_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCALF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTCCALF_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No frequency output to RTCCLK pin"]
    #[inline(always)]
    pub fn rtccalf_0(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_0)
    }
    #[doc = "512 Hz"]
    #[inline(always)]
    pub fn rtccalf_1(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_1)
    }
    #[doc = "256 Hz"]
    #[inline(always)]
    pub fn rtccalf_2(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_2)
    }
    #[doc = "1 Hz"]
    #[inline(always)]
    pub fn rtccalf_3(self) -> &'a mut W {
        self.variant(RTCCALF_A::RTCCALF_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u16 & 0x03) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&self) -> RTCTEV_R {
        RTCTEV_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&self) -> RTCSSEL_R {
        RTCSSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Real-time clock ready"]
    #[inline(always)]
    pub fn rtcrdy(&self) -> RTCRDY_R {
        RTCRDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn rtcmode(&self) -> RTCMODE_R {
        RTCMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&self) -> RTCHOLD_R {
        RTCHOLD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&self) -> RTCBCD_R {
        RTCBCD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&self) -> RTCCALF_R {
        RTCCALF_R::new(((self.bits >> 8) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Real-time clock time event"]
    #[inline(always)]
    pub fn rtctev(&mut self) -> RTCTEV_W {
        RTCTEV_W { w: self }
    }
    #[doc = "Bits 2:3 - Real-time clock source select"]
    #[inline(always)]
    pub fn rtcssel(&mut self) -> RTCSSEL_W {
        RTCSSEL_W { w: self }
    }
    #[doc = "Bit 6 - Real-time clock hold"]
    #[inline(always)]
    pub fn rtchold(&mut self) -> RTCHOLD_W {
        RTCHOLD_W { w: self }
    }
    #[doc = "Bit 7 - Real-time clock BCD select"]
    #[inline(always)]
    pub fn rtcbcd(&mut self) -> RTCBCD_W {
        RTCBCD_W { w: self }
    }
    #[doc = "Bits 8:9 - Real-time clock calibration frequency"]
    #[inline(always)]
    pub fn rtccalf(&mut self) -> RTCCALF_W {
        RTCCALF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTCCTL13 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl13](index.html) module"]
pub struct RTCCTL13_SPEC;
impl crate::RegisterSpec for RTCCTL13_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl13::R](R) reader structure"]
impl crate::Readable for RTCCTL13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl13::W](W) writer structure"]
impl crate::Writable for RTCCTL13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL13 to value 0x70"]
impl crate::Resettable for RTCCTL13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x70
    }
}
