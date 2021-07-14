#[doc = "Register `FLCTL_BANK0_RDCTL` reader"]
pub struct R(crate::R<FLCTL_BANK0_RDCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK0_RDCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK0_RDCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK0_RDCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK0_RDCTL` writer"]
pub struct W(crate::W<FLCTL_BANK0_RDCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK0_RDCTL_SPEC>;
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
impl From<crate::W<FLCTL_BANK0_RDCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK0_RDCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Flash read mode control setting for Bank 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RD_MODE_A {
    #[doc = "0: Normal read mode"]
    RD_MODE_0 = 0,
    #[doc = "1: Read Margin 0"]
    RD_MODE_1 = 1,
    #[doc = "2: Read Margin 1"]
    RD_MODE_2 = 2,
    #[doc = "3: Program Verify"]
    RD_MODE_3 = 3,
    #[doc = "4: Erase Verify"]
    RD_MODE_4 = 4,
    #[doc = "5: Leakage Verify"]
    RD_MODE_5 = 5,
    #[doc = "9: Read Margin 0B"]
    RD_MODE_9 = 9,
    #[doc = "10: Read Margin 1B"]
    RD_MODE_10 = 10,
}
impl From<RD_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: RD_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RD_MODE` reader - Flash read mode control setting for Bank 0"]
pub struct RD_MODE_R(crate::FieldReader<u8, RD_MODE_A>);
impl RD_MODE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RD_MODE_A> {
        match self.bits {
            0 => Some(RD_MODE_A::RD_MODE_0),
            1 => Some(RD_MODE_A::RD_MODE_1),
            2 => Some(RD_MODE_A::RD_MODE_2),
            3 => Some(RD_MODE_A::RD_MODE_3),
            4 => Some(RD_MODE_A::RD_MODE_4),
            5 => Some(RD_MODE_A::RD_MODE_5),
            9 => Some(RD_MODE_A::RD_MODE_9),
            10 => Some(RD_MODE_A::RD_MODE_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RD_MODE_0`"]
    #[inline(always)]
    pub fn is_rd_mode_0(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_0
    }
    #[doc = "Checks if the value of the field is `RD_MODE_1`"]
    #[inline(always)]
    pub fn is_rd_mode_1(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_1
    }
    #[doc = "Checks if the value of the field is `RD_MODE_2`"]
    #[inline(always)]
    pub fn is_rd_mode_2(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_2
    }
    #[doc = "Checks if the value of the field is `RD_MODE_3`"]
    #[inline(always)]
    pub fn is_rd_mode_3(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_3
    }
    #[doc = "Checks if the value of the field is `RD_MODE_4`"]
    #[inline(always)]
    pub fn is_rd_mode_4(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_4
    }
    #[doc = "Checks if the value of the field is `RD_MODE_5`"]
    #[inline(always)]
    pub fn is_rd_mode_5(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_5
    }
    #[doc = "Checks if the value of the field is `RD_MODE_9`"]
    #[inline(always)]
    pub fn is_rd_mode_9(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_9
    }
    #[doc = "Checks if the value of the field is `RD_MODE_10`"]
    #[inline(always)]
    pub fn is_rd_mode_10(&self) -> bool {
        **self == RD_MODE_A::RD_MODE_10
    }
}
impl core::ops::Deref for RD_MODE_R {
    type Target = crate::FieldReader<u8, RD_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RD_MODE` writer - Flash read mode control setting for Bank 0"]
pub struct RD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RD_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Normal read mode"]
    #[inline(always)]
    pub fn rd_mode_0(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_0)
    }
    #[doc = "Read Margin 0"]
    #[inline(always)]
    pub fn rd_mode_1(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_1)
    }
    #[doc = "Read Margin 1"]
    #[inline(always)]
    pub fn rd_mode_2(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_2)
    }
    #[doc = "Program Verify"]
    #[inline(always)]
    pub fn rd_mode_3(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_3)
    }
    #[doc = "Erase Verify"]
    #[inline(always)]
    pub fn rd_mode_4(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_4)
    }
    #[doc = "Leakage Verify"]
    #[inline(always)]
    pub fn rd_mode_5(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_5)
    }
    #[doc = "Read Margin 0B"]
    #[inline(always)]
    pub fn rd_mode_9(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_9)
    }
    #[doc = "Read Margin 1B"]
    #[inline(always)]
    pub fn rd_mode_10(self) -> &'a mut W {
        self.variant(RD_MODE_A::RD_MODE_10)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `BUFI` reader - Enables read buffering feature for instruction fetches to this Bank"]
pub struct BUFI_R(crate::FieldReader<bool, bool>);
impl BUFI_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFI` writer - Enables read buffering feature for instruction fetches to this Bank"]
pub struct BUFI_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFI_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `BUFD` reader - Enables read buffering feature for data reads to this Bank"]
pub struct BUFD_R(crate::FieldReader<bool, bool>);
impl BUFD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUFD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUFD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUFD` writer - Enables read buffering feature for data reads to this Bank"]
pub struct BUFD_W<'a> {
    w: &'a mut W,
}
impl<'a> BUFD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Number of wait states for read\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WAIT_A {
    #[doc = "0: 0 wait states"]
    WAIT_0 = 0,
    #[doc = "1: 1 wait states"]
    WAIT_1 = 1,
    #[doc = "2: 2 wait states"]
    WAIT_2 = 2,
    #[doc = "3: 3 wait states"]
    WAIT_3 = 3,
    #[doc = "4: 4 wait states"]
    WAIT_4 = 4,
    #[doc = "5: 5 wait states"]
    WAIT_5 = 5,
    #[doc = "6: 6 wait states"]
    WAIT_6 = 6,
    #[doc = "7: 7 wait states"]
    WAIT_7 = 7,
    #[doc = "8: 8 wait states"]
    WAIT_8 = 8,
    #[doc = "9: 9 wait states"]
    WAIT_9 = 9,
    #[doc = "10: 10 wait states"]
    WAIT_10 = 10,
    #[doc = "11: 11 wait states"]
    WAIT_11 = 11,
    #[doc = "12: 12 wait states"]
    WAIT_12 = 12,
    #[doc = "13: 13 wait states"]
    WAIT_13 = 13,
    #[doc = "14: 14 wait states"]
    WAIT_14 = 14,
    #[doc = "15: 15 wait states"]
    WAIT_15 = 15,
}
impl From<WAIT_A> for u8 {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `WAIT` reader - Number of wait states for read"]
pub struct WAIT_R(crate::FieldReader<u8, WAIT_A>);
impl WAIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        WAIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            0 => WAIT_A::WAIT_0,
            1 => WAIT_A::WAIT_1,
            2 => WAIT_A::WAIT_2,
            3 => WAIT_A::WAIT_3,
            4 => WAIT_A::WAIT_4,
            5 => WAIT_A::WAIT_5,
            6 => WAIT_A::WAIT_6,
            7 => WAIT_A::WAIT_7,
            8 => WAIT_A::WAIT_8,
            9 => WAIT_A::WAIT_9,
            10 => WAIT_A::WAIT_10,
            11 => WAIT_A::WAIT_11,
            12 => WAIT_A::WAIT_12,
            13 => WAIT_A::WAIT_13,
            14 => WAIT_A::WAIT_14,
            15 => WAIT_A::WAIT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_0`"]
    #[inline(always)]
    pub fn is_wait_0(&self) -> bool {
        **self == WAIT_A::WAIT_0
    }
    #[doc = "Checks if the value of the field is `WAIT_1`"]
    #[inline(always)]
    pub fn is_wait_1(&self) -> bool {
        **self == WAIT_A::WAIT_1
    }
    #[doc = "Checks if the value of the field is `WAIT_2`"]
    #[inline(always)]
    pub fn is_wait_2(&self) -> bool {
        **self == WAIT_A::WAIT_2
    }
    #[doc = "Checks if the value of the field is `WAIT_3`"]
    #[inline(always)]
    pub fn is_wait_3(&self) -> bool {
        **self == WAIT_A::WAIT_3
    }
    #[doc = "Checks if the value of the field is `WAIT_4`"]
    #[inline(always)]
    pub fn is_wait_4(&self) -> bool {
        **self == WAIT_A::WAIT_4
    }
    #[doc = "Checks if the value of the field is `WAIT_5`"]
    #[inline(always)]
    pub fn is_wait_5(&self) -> bool {
        **self == WAIT_A::WAIT_5
    }
    #[doc = "Checks if the value of the field is `WAIT_6`"]
    #[inline(always)]
    pub fn is_wait_6(&self) -> bool {
        **self == WAIT_A::WAIT_6
    }
    #[doc = "Checks if the value of the field is `WAIT_7`"]
    #[inline(always)]
    pub fn is_wait_7(&self) -> bool {
        **self == WAIT_A::WAIT_7
    }
    #[doc = "Checks if the value of the field is `WAIT_8`"]
    #[inline(always)]
    pub fn is_wait_8(&self) -> bool {
        **self == WAIT_A::WAIT_8
    }
    #[doc = "Checks if the value of the field is `WAIT_9`"]
    #[inline(always)]
    pub fn is_wait_9(&self) -> bool {
        **self == WAIT_A::WAIT_9
    }
    #[doc = "Checks if the value of the field is `WAIT_10`"]
    #[inline(always)]
    pub fn is_wait_10(&self) -> bool {
        **self == WAIT_A::WAIT_10
    }
    #[doc = "Checks if the value of the field is `WAIT_11`"]
    #[inline(always)]
    pub fn is_wait_11(&self) -> bool {
        **self == WAIT_A::WAIT_11
    }
    #[doc = "Checks if the value of the field is `WAIT_12`"]
    #[inline(always)]
    pub fn is_wait_12(&self) -> bool {
        **self == WAIT_A::WAIT_12
    }
    #[doc = "Checks if the value of the field is `WAIT_13`"]
    #[inline(always)]
    pub fn is_wait_13(&self) -> bool {
        **self == WAIT_A::WAIT_13
    }
    #[doc = "Checks if the value of the field is `WAIT_14`"]
    #[inline(always)]
    pub fn is_wait_14(&self) -> bool {
        **self == WAIT_A::WAIT_14
    }
    #[doc = "Checks if the value of the field is `WAIT_15`"]
    #[inline(always)]
    pub fn is_wait_15(&self) -> bool {
        **self == WAIT_A::WAIT_15
    }
}
impl core::ops::Deref for WAIT_R {
    type Target = crate::FieldReader<u8, WAIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WAIT` writer - Number of wait states for read"]
pub struct WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> WAIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAIT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0 wait states"]
    #[inline(always)]
    pub fn wait_0(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_0)
    }
    #[doc = "1 wait states"]
    #[inline(always)]
    pub fn wait_1(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_1)
    }
    #[doc = "2 wait states"]
    #[inline(always)]
    pub fn wait_2(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_2)
    }
    #[doc = "3 wait states"]
    #[inline(always)]
    pub fn wait_3(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_3)
    }
    #[doc = "4 wait states"]
    #[inline(always)]
    pub fn wait_4(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_4)
    }
    #[doc = "5 wait states"]
    #[inline(always)]
    pub fn wait_5(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_5)
    }
    #[doc = "6 wait states"]
    #[inline(always)]
    pub fn wait_6(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_6)
    }
    #[doc = "7 wait states"]
    #[inline(always)]
    pub fn wait_7(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_7)
    }
    #[doc = "8 wait states"]
    #[inline(always)]
    pub fn wait_8(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_8)
    }
    #[doc = "9 wait states"]
    #[inline(always)]
    pub fn wait_9(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_9)
    }
    #[doc = "10 wait states"]
    #[inline(always)]
    pub fn wait_10(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_10)
    }
    #[doc = "11 wait states"]
    #[inline(always)]
    pub fn wait_11(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_11)
    }
    #[doc = "12 wait states"]
    #[inline(always)]
    pub fn wait_12(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_12)
    }
    #[doc = "13 wait states"]
    #[inline(always)]
    pub fn wait_13(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_13)
    }
    #[doc = "14 wait states"]
    #[inline(always)]
    pub fn wait_14(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_14)
    }
    #[doc = "15 wait states"]
    #[inline(always)]
    pub fn wait_15(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Read mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RD_MODE_STATUS_A {
    #[doc = "0: Normal read mode"]
    RD_MODE_STATUS_0 = 0,
    #[doc = "1: Read Margin 0"]
    RD_MODE_STATUS_1 = 1,
    #[doc = "2: Read Margin 1"]
    RD_MODE_STATUS_2 = 2,
    #[doc = "3: Program Verify"]
    RD_MODE_STATUS_3 = 3,
    #[doc = "4: Erase Verify"]
    RD_MODE_STATUS_4 = 4,
    #[doc = "5: Leakage Verify"]
    RD_MODE_STATUS_5 = 5,
    #[doc = "9: Read Margin 0B"]
    RD_MODE_STATUS_9 = 9,
    #[doc = "10: Read Margin 1B"]
    RD_MODE_STATUS_10 = 10,
}
impl From<RD_MODE_STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: RD_MODE_STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RD_MODE_STATUS` reader - Read mode"]
pub struct RD_MODE_STATUS_R(crate::FieldReader<u8, RD_MODE_STATUS_A>);
impl RD_MODE_STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        RD_MODE_STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RD_MODE_STATUS_A> {
        match self.bits {
            0 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_0),
            1 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_1),
            2 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_2),
            3 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_3),
            4 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_4),
            5 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_5),
            9 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_9),
            10 => Some(RD_MODE_STATUS_A::RD_MODE_STATUS_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_0`"]
    #[inline(always)]
    pub fn is_rd_mode_status_0(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_0
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_1`"]
    #[inline(always)]
    pub fn is_rd_mode_status_1(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_1
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_2`"]
    #[inline(always)]
    pub fn is_rd_mode_status_2(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_2
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_3`"]
    #[inline(always)]
    pub fn is_rd_mode_status_3(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_3
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_4`"]
    #[inline(always)]
    pub fn is_rd_mode_status_4(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_4
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_5`"]
    #[inline(always)]
    pub fn is_rd_mode_status_5(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_5
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_9`"]
    #[inline(always)]
    pub fn is_rd_mode_status_9(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_9
    }
    #[doc = "Checks if the value of the field is `RD_MODE_STATUS_10`"]
    #[inline(always)]
    pub fn is_rd_mode_status_10(&self) -> bool {
        **self == RD_MODE_STATUS_A::RD_MODE_STATUS_10
    }
}
impl core::ops::Deref for RD_MODE_STATUS_R {
    type Target = crate::FieldReader<u8, RD_MODE_STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Flash read mode control setting for Bank 0"]
    #[inline(always)]
    pub fn rd_mode(&self) -> RD_MODE_R {
        RD_MODE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Enables read buffering feature for instruction fetches to this Bank"]
    #[inline(always)]
    pub fn bufi(&self) -> BUFI_R {
        BUFI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Enables read buffering feature for data reads to this Bank"]
    #[inline(always)]
    pub fn bufd(&self) -> BUFD_R {
        BUFD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - Number of wait states for read"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Read mode"]
    #[inline(always)]
    pub fn rd_mode_status(&self) -> RD_MODE_STATUS_R {
        RD_MODE_STATUS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Flash read mode control setting for Bank 0"]
    #[inline(always)]
    pub fn rd_mode(&mut self) -> RD_MODE_W {
        RD_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Enables read buffering feature for instruction fetches to this Bank"]
    #[inline(always)]
    pub fn bufi(&mut self) -> BUFI_W {
        BUFI_W { w: self }
    }
    #[doc = "Bit 5 - Enables read buffering feature for data reads to this Bank"]
    #[inline(always)]
    pub fn bufd(&mut self) -> BUFD_W {
        BUFD_W { w: self }
    }
    #[doc = "Bits 12:15 - Number of wait states for read"]
    #[inline(always)]
    pub fn wait(&mut self) -> WAIT_W {
        WAIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bank0 Read Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank0_rdctl](index.html) module"]
pub struct FLCTL_BANK0_RDCTL_SPEC;
impl crate::RegisterSpec for FLCTL_BANK0_RDCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank0_rdctl::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK0_RDCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank0_rdctl::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK0_RDCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK0_RDCTL to value 0"]
impl crate::Resettable for FLCTL_BANK0_RDCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
