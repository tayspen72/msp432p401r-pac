#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
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
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Watchdog timer interval select\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTIS_A {
    #[doc = "0: Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    WDTIS_0 = 0,
    #[doc = "1: Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    WDTIS_1 = 1,
    #[doc = "2: Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    WDTIS_2 = 2,
    #[doc = "3: Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    WDTIS_3 = 3,
    #[doc = "4: Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    WDTIS_4 = 4,
    #[doc = "5: Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    WDTIS_5 = 5,
    #[doc = "6: Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    WDTIS_6 = 6,
    #[doc = "7: Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    WDTIS_7 = 7,
}
impl From<WDTIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTIS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTIS` reader - Watchdog timer interval select"]
pub struct WDTIS_R(crate::FieldReader<u8, WDTIS_A>);
impl WDTIS_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTIS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIS_A {
        match self.bits {
            0 => WDTIS_A::WDTIS_0,
            1 => WDTIS_A::WDTIS_1,
            2 => WDTIS_A::WDTIS_2,
            3 => WDTIS_A::WDTIS_3,
            4 => WDTIS_A::WDTIS_4,
            5 => WDTIS_A::WDTIS_5,
            6 => WDTIS_A::WDTIS_6,
            7 => WDTIS_A::WDTIS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTIS_0`"]
    #[inline(always)]
    pub fn is_wdtis_0(&self) -> bool {
        **self == WDTIS_A::WDTIS_0
    }
    #[doc = "Checks if the value of the field is `WDTIS_1`"]
    #[inline(always)]
    pub fn is_wdtis_1(&self) -> bool {
        **self == WDTIS_A::WDTIS_1
    }
    #[doc = "Checks if the value of the field is `WDTIS_2`"]
    #[inline(always)]
    pub fn is_wdtis_2(&self) -> bool {
        **self == WDTIS_A::WDTIS_2
    }
    #[doc = "Checks if the value of the field is `WDTIS_3`"]
    #[inline(always)]
    pub fn is_wdtis_3(&self) -> bool {
        **self == WDTIS_A::WDTIS_3
    }
    #[doc = "Checks if the value of the field is `WDTIS_4`"]
    #[inline(always)]
    pub fn is_wdtis_4(&self) -> bool {
        **self == WDTIS_A::WDTIS_4
    }
    #[doc = "Checks if the value of the field is `WDTIS_5`"]
    #[inline(always)]
    pub fn is_wdtis_5(&self) -> bool {
        **self == WDTIS_A::WDTIS_5
    }
    #[doc = "Checks if the value of the field is `WDTIS_6`"]
    #[inline(always)]
    pub fn is_wdtis_6(&self) -> bool {
        **self == WDTIS_A::WDTIS_6
    }
    #[doc = "Checks if the value of the field is `WDTIS_7`"]
    #[inline(always)]
    pub fn is_wdtis_7(&self) -> bool {
        **self == WDTIS_A::WDTIS_7
    }
}
impl core::ops::Deref for WDTIS_R {
    type Target = crate::FieldReader<u8, WDTIS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTIS` writer - Watchdog timer interval select"]
pub struct WDTIS_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTIS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTIS_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Watchdog clock source / (2^(31)) (18:12:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_0(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_0)
    }
    #[doc = "Watchdog clock source /(2^(27)) (01:08:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_1(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_1)
    }
    #[doc = "Watchdog clock source /(2^(23)) (00:04:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_2(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_2)
    }
    #[doc = "Watchdog clock source /(2^(19)) (00:00:16 at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_3(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_3)
    }
    #[doc = "Watchdog clock source /(2^(15)) (1 s at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_4(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_4)
    }
    #[doc = "Watchdog clock source / (2^(13)) (250 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_5(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_5)
    }
    #[doc = "Watchdog clock source / (2^(9)) (15.625 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_6(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_6)
    }
    #[doc = "Watchdog clock source / (2^(6)) (1.95 ms at 32.768 kHz)"]
    #[inline(always)]
    pub fn wdtis_7(self) -> &'a mut W {
        self.variant(WDTIS_A::WDTIS_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u16 & 0x07);
        self.w
    }
}
#[doc = "Watchdog timer counter clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTCNTCL_AW {
    #[doc = "0: No action"]
    WDTCNTCL_0 = 0,
    #[doc = "1: WDTCNT = 0000h"]
    WDTCNTCL_1 = 1,
}
impl From<WDTCNTCL_AW> for bool {
    #[inline(always)]
    fn from(variant: WDTCNTCL_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTCNTCL` writer - Watchdog timer counter clear"]
pub struct WDTCNTCL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTCNTCL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTCNTCL_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No action"]
    #[inline(always)]
    pub fn wdtcntcl_0(self) -> &'a mut W {
        self.variant(WDTCNTCL_AW::WDTCNTCL_0)
    }
    #[doc = "WDTCNT = 0000h"]
    #[inline(always)]
    pub fn wdtcntcl_1(self) -> &'a mut W {
        self.variant(WDTCNTCL_AW::WDTCNTCL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
#[doc = "Watchdog timer mode select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTTMSEL_A {
    #[doc = "0: Watchdog mode"]
    WDTTMSEL_0 = 0,
    #[doc = "1: Interval timer mode"]
    WDTTMSEL_1 = 1,
}
impl From<WDTTMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: WDTTMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTTMSEL` reader - Watchdog timer mode select"]
pub struct WDTTMSEL_R(crate::FieldReader<bool, WDTTMSEL_A>);
impl WDTTMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTTMSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTTMSEL_A {
        match self.bits {
            false => WDTTMSEL_A::WDTTMSEL_0,
            true => WDTTMSEL_A::WDTTMSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTTMSEL_0`"]
    #[inline(always)]
    pub fn is_wdttmsel_0(&self) -> bool {
        **self == WDTTMSEL_A::WDTTMSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTTMSEL_1`"]
    #[inline(always)]
    pub fn is_wdttmsel_1(&self) -> bool {
        **self == WDTTMSEL_A::WDTTMSEL_1
    }
}
impl core::ops::Deref for WDTTMSEL_R {
    type Target = crate::FieldReader<bool, WDTTMSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTTMSEL` writer - Watchdog timer mode select"]
pub struct WDTTMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTTMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTTMSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog mode"]
    #[inline(always)]
    pub fn wdttmsel_0(self) -> &'a mut W {
        self.variant(WDTTMSEL_A::WDTTMSEL_0)
    }
    #[doc = "Interval timer mode"]
    #[inline(always)]
    pub fn wdttmsel_1(self) -> &'a mut W {
        self.variant(WDTTMSEL_A::WDTTMSEL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u16 & 0x01) << 4);
        self.w
    }
}
#[doc = "Watchdog timer clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: SMCLK"]
    WDTSSEL_0 = 0,
    #[doc = "1: ACLK"]
    WDTSSEL_1 = 1,
    #[doc = "2: VLOCLK"]
    WDTSSEL_2 = 2,
    #[doc = "3: BCLK"]
    WDTSSEL_3 = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTSSEL` reader - Watchdog timer clock source select"]
pub struct WDTSSEL_R(crate::FieldReader<u8, WDTSSEL_A>);
impl WDTSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::WDTSSEL_0,
            1 => WDTSSEL_A::WDTSSEL_1,
            2 => WDTSSEL_A::WDTSSEL_2,
            3 => WDTSSEL_A::WDTSSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_0`"]
    #[inline(always)]
    pub fn is_wdtssel_0(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_0
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_1`"]
    #[inline(always)]
    pub fn is_wdtssel_1(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_1
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_2`"]
    #[inline(always)]
    pub fn is_wdtssel_2(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_2
    }
    #[doc = "Checks if the value of the field is `WDTSSEL_3`"]
    #[inline(always)]
    pub fn is_wdtssel_3(&self) -> bool {
        **self == WDTSSEL_A::WDTSSEL_3
    }
}
impl core::ops::Deref for WDTSSEL_R {
    type Target = crate::FieldReader<u8, WDTSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTSSEL` writer - Watchdog timer clock source select"]
pub struct WDTSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SMCLK"]
    #[inline(always)]
    pub fn wdtssel_0(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_0)
    }
    #[doc = "ACLK"]
    #[inline(always)]
    pub fn wdtssel_1(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_1)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn wdtssel_2(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_2)
    }
    #[doc = "BCLK"]
    #[inline(always)]
    pub fn wdtssel_3(self) -> &'a mut W {
        self.variant(WDTSSEL_A::WDTSSEL_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u16 & 0x03) << 5);
        self.w
    }
}
#[doc = "Watchdog timer hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WDTHOLD_A {
    #[doc = "0: Watchdog timer is not stopped"]
    WDTHOLD_0 = 0,
    #[doc = "1: Watchdog timer is stopped"]
    WDTHOLD_1 = 1,
}
impl From<WDTHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTHOLD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDTHOLD` reader - Watchdog timer hold"]
pub struct WDTHOLD_R(crate::FieldReader<bool, WDTHOLD_A>);
impl WDTHOLD_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDTHOLD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTHOLD_A {
        match self.bits {
            false => WDTHOLD_A::WDTHOLD_0,
            true => WDTHOLD_A::WDTHOLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDTHOLD_0`"]
    #[inline(always)]
    pub fn is_wdthold_0(&self) -> bool {
        **self == WDTHOLD_A::WDTHOLD_0
    }
    #[doc = "Checks if the value of the field is `WDTHOLD_1`"]
    #[inline(always)]
    pub fn is_wdthold_1(&self) -> bool {
        **self == WDTHOLD_A::WDTHOLD_1
    }
}
impl core::ops::Deref for WDTHOLD_R {
    type Target = crate::FieldReader<bool, WDTHOLD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTHOLD` writer - Watchdog timer hold"]
pub struct WDTHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WDTHOLD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn wdthold_0(self) -> &'a mut W {
        self.variant(WDTHOLD_A::WDTHOLD_0)
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn wdthold_1(self) -> &'a mut W {
        self.variant(WDTHOLD_A::WDTHOLD_1)
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
#[doc = "Field `WDTPW` reader - Watchdog timer password"]
pub struct WDTPW_R(crate::FieldReader<u8, u8>);
impl WDTPW_R {
    pub(crate) fn new(bits: u8) -> Self {
        WDTPW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDTPW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDTPW` writer - Watchdog timer password"]
pub struct WDTPW_W<'a> {
    w: &'a mut W,
}
impl<'a> WDTPW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Watchdog timer interval select"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W {
        WDTIS_W { w: self }
    }
    #[doc = "Bit 3 - Watchdog timer counter clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W {
        WDTCNTCL_W { w: self }
    }
    #[doc = "Bit 4 - Watchdog timer mode select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W {
        WDTTMSEL_W { w: self }
    }
    #[doc = "Bits 5:6 - Watchdog timer clock source select"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W {
        WDTSSEL_W { w: self }
    }
    #[doc = "Bit 7 - Watchdog timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W {
        WDTHOLD_W { w: self }
    }
    #[doc = "Bits 8:15 - Watchdog timer password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W {
        WDTPW_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCTL to value 0x6904"]
impl crate::Resettable for WDTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x6904
    }
}
