#[doc = "Register `SYS_PERIHALT_CTL` reader"]
pub struct R(crate::R<SYS_PERIHALT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_PERIHALT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_PERIHALT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_PERIHALT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_PERIHALT_CTL` writer"]
pub struct W(crate::W<SYS_PERIHALT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_PERIHALT_CTL_SPEC>;
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
impl From<crate::W<SYS_PERIHALT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_PERIHALT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_T16_0_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_T16_0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_T16_0_1 = 1,
}
impl From<HALT_T16_0_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_T16_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_0` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_0_R(crate::FieldReader<bool, HALT_T16_0_A>);
impl HALT_T16_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_T16_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_T16_0_A {
        match self.bits {
            false => HALT_T16_0_A::HALT_T16_0_0,
            true => HALT_T16_0_A::HALT_T16_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_T16_0_0`"]
    #[inline(always)]
    pub fn is_halt_t16_0_0(&self) -> bool {
        **self == HALT_T16_0_A::HALT_T16_0_0
    }
    #[doc = "Checks if the value of the field is `HALT_T16_0_1`"]
    #[inline(always)]
    pub fn is_halt_t16_0_1(&self) -> bool {
        **self == HALT_T16_0_A::HALT_T16_0_1
    }
}
impl core::ops::Deref for HALT_T16_0_R {
    type Target = crate::FieldReader<bool, HALT_T16_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_T16_0` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_0_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_T16_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_T16_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0_0(self) -> &'a mut W {
        self.variant(HALT_T16_0_A::HALT_T16_0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0_1(self) -> &'a mut W {
        self.variant(HALT_T16_0_A::HALT_T16_0_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_T16_1_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_T16_1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_T16_1_1 = 1,
}
impl From<HALT_T16_1_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_T16_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_1` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_1_R(crate::FieldReader<bool, HALT_T16_1_A>);
impl HALT_T16_1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_T16_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_T16_1_A {
        match self.bits {
            false => HALT_T16_1_A::HALT_T16_1_0,
            true => HALT_T16_1_A::HALT_T16_1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_T16_1_0`"]
    #[inline(always)]
    pub fn is_halt_t16_1_0(&self) -> bool {
        **self == HALT_T16_1_A::HALT_T16_1_0
    }
    #[doc = "Checks if the value of the field is `HALT_T16_1_1`"]
    #[inline(always)]
    pub fn is_halt_t16_1_1(&self) -> bool {
        **self == HALT_T16_1_A::HALT_T16_1_1
    }
}
impl core::ops::Deref for HALT_T16_1_R {
    type Target = crate::FieldReader<bool, HALT_T16_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_T16_1` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_1_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_T16_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_T16_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1_0(self) -> &'a mut W {
        self.variant(HALT_T16_1_A::HALT_T16_1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1_1(self) -> &'a mut W {
        self.variant(HALT_T16_1_A::HALT_T16_1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_T16_2_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_T16_2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_T16_2_1 = 1,
}
impl From<HALT_T16_2_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_T16_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_2` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_2_R(crate::FieldReader<bool, HALT_T16_2_A>);
impl HALT_T16_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_T16_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_T16_2_A {
        match self.bits {
            false => HALT_T16_2_A::HALT_T16_2_0,
            true => HALT_T16_2_A::HALT_T16_2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_T16_2_0`"]
    #[inline(always)]
    pub fn is_halt_t16_2_0(&self) -> bool {
        **self == HALT_T16_2_A::HALT_T16_2_0
    }
    #[doc = "Checks if the value of the field is `HALT_T16_2_1`"]
    #[inline(always)]
    pub fn is_halt_t16_2_1(&self) -> bool {
        **self == HALT_T16_2_A::HALT_T16_2_1
    }
}
impl core::ops::Deref for HALT_T16_2_R {
    type Target = crate::FieldReader<bool, HALT_T16_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_T16_2` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_2_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_T16_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_T16_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2_0(self) -> &'a mut W {
        self.variant(HALT_T16_2_A::HALT_T16_2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2_1(self) -> &'a mut W {
        self.variant(HALT_T16_2_A::HALT_T16_2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_T16_3_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_T16_3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_T16_3_1 = 1,
}
impl From<HALT_T16_3_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_T16_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T16_3` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_3_R(crate::FieldReader<bool, HALT_T16_3_A>);
impl HALT_T16_3_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_T16_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_T16_3_A {
        match self.bits {
            false => HALT_T16_3_A::HALT_T16_3_0,
            true => HALT_T16_3_A::HALT_T16_3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_T16_3_0`"]
    #[inline(always)]
    pub fn is_halt_t16_3_0(&self) -> bool {
        **self == HALT_T16_3_A::HALT_T16_3_0
    }
    #[doc = "Checks if the value of the field is `HALT_T16_3_1`"]
    #[inline(always)]
    pub fn is_halt_t16_3_1(&self) -> bool {
        **self == HALT_T16_3_A::HALT_T16_3_1
    }
}
impl core::ops::Deref for HALT_T16_3_R {
    type Target = crate::FieldReader<bool, HALT_T16_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_T16_3` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_T16_3_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_T16_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_T16_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3_0(self) -> &'a mut W {
        self.variant(HALT_T16_3_A::HALT_T16_3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3_1(self) -> &'a mut W {
        self.variant(HALT_T16_3_A::HALT_T16_3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_T32_0_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_T32_0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_T32_0_1 = 1,
}
impl From<HALT_T32_0_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_T32_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_T32_0` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_T32_0_R(crate::FieldReader<bool, HALT_T32_0_A>);
impl HALT_T32_0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_T32_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_T32_0_A {
        match self.bits {
            false => HALT_T32_0_A::HALT_T32_0_0,
            true => HALT_T32_0_A::HALT_T32_0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_T32_0_0`"]
    #[inline(always)]
    pub fn is_halt_t32_0_0(&self) -> bool {
        **self == HALT_T32_0_A::HALT_T32_0_0
    }
    #[doc = "Checks if the value of the field is `HALT_T32_0_1`"]
    #[inline(always)]
    pub fn is_halt_t32_0_1(&self) -> bool {
        **self == HALT_T32_0_A::HALT_T32_0_1
    }
}
impl core::ops::Deref for HALT_T32_0_R {
    type Target = crate::FieldReader<bool, HALT_T32_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_T32_0` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_T32_0_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_T32_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_T32_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0_0(self) -> &'a mut W {
        self.variant(HALT_T32_0_A::HALT_T32_0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0_1(self) -> &'a mut W {
        self.variant(HALT_T32_0_A::HALT_T32_0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUA0_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUA0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUA0_1 = 1,
}
impl From<HALT_EUA0_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUA0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA0` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA0_R(crate::FieldReader<bool, HALT_EUA0_A>);
impl HALT_EUA0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUA0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUA0_A {
        match self.bits {
            false => HALT_EUA0_A::HALT_EUA0_0,
            true => HALT_EUA0_A::HALT_EUA0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUA0_0`"]
    #[inline(always)]
    pub fn is_halt_e_ua0_0(&self) -> bool {
        **self == HALT_EUA0_A::HALT_EUA0_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUA0_1`"]
    #[inline(always)]
    pub fn is_halt_e_ua0_1(&self) -> bool {
        **self == HALT_EUA0_A::HALT_EUA0_1
    }
}
impl core::ops::Deref for HALT_EUA0_R {
    type Target = crate::FieldReader<bool, HALT_EUA0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUA0` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA0_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUA0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUA0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0_0(self) -> &'a mut W {
        self.variant(HALT_EUA0_A::HALT_EUA0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0_1(self) -> &'a mut W {
        self.variant(HALT_EUA0_A::HALT_EUA0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUA1_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUA1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUA1_1 = 1,
}
impl From<HALT_EUA1_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUA1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA1` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA1_R(crate::FieldReader<bool, HALT_EUA1_A>);
impl HALT_EUA1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUA1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUA1_A {
        match self.bits {
            false => HALT_EUA1_A::HALT_EUA1_0,
            true => HALT_EUA1_A::HALT_EUA1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUA1_0`"]
    #[inline(always)]
    pub fn is_halt_e_ua1_0(&self) -> bool {
        **self == HALT_EUA1_A::HALT_EUA1_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUA1_1`"]
    #[inline(always)]
    pub fn is_halt_e_ua1_1(&self) -> bool {
        **self == HALT_EUA1_A::HALT_EUA1_1
    }
}
impl core::ops::Deref for HALT_EUA1_R {
    type Target = crate::FieldReader<bool, HALT_EUA1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUA1` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA1_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUA1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUA1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1_0(self) -> &'a mut W {
        self.variant(HALT_EUA1_A::HALT_EUA1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1_1(self) -> &'a mut W {
        self.variant(HALT_EUA1_A::HALT_EUA1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUA2_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUA2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUA2_1 = 1,
}
impl From<HALT_EUA2_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUA2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA2` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA2_R(crate::FieldReader<bool, HALT_EUA2_A>);
impl HALT_EUA2_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUA2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUA2_A {
        match self.bits {
            false => HALT_EUA2_A::HALT_EUA2_0,
            true => HALT_EUA2_A::HALT_EUA2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUA2_0`"]
    #[inline(always)]
    pub fn is_halt_e_ua2_0(&self) -> bool {
        **self == HALT_EUA2_A::HALT_EUA2_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUA2_1`"]
    #[inline(always)]
    pub fn is_halt_e_ua2_1(&self) -> bool {
        **self == HALT_EUA2_A::HALT_EUA2_1
    }
}
impl core::ops::Deref for HALT_EUA2_R {
    type Target = crate::FieldReader<bool, HALT_EUA2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUA2` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA2_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUA2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUA2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2_0(self) -> &'a mut W {
        self.variant(HALT_EUA2_A::HALT_EUA2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2_1(self) -> &'a mut W {
        self.variant(HALT_EUA2_A::HALT_EUA2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUA3_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUA3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUA3_1 = 1,
}
impl From<HALT_EUA3_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUA3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUA3` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA3_R(crate::FieldReader<bool, HALT_EUA3_A>);
impl HALT_EUA3_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUA3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUA3_A {
        match self.bits {
            false => HALT_EUA3_A::HALT_EUA3_0,
            true => HALT_EUA3_A::HALT_EUA3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUA3_0`"]
    #[inline(always)]
    pub fn is_halt_e_ua3_0(&self) -> bool {
        **self == HALT_EUA3_A::HALT_EUA3_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUA3_1`"]
    #[inline(always)]
    pub fn is_halt_e_ua3_1(&self) -> bool {
        **self == HALT_EUA3_A::HALT_EUA3_1
    }
}
impl core::ops::Deref for HALT_EUA3_R {
    type Target = crate::FieldReader<bool, HALT_EUA3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUA3` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUA3_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUA3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUA3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3_0(self) -> &'a mut W {
        self.variant(HALT_EUA3_A::HALT_EUA3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3_1(self) -> &'a mut W {
        self.variant(HALT_EUA3_A::HALT_EUA3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUB0_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUB0_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUB0_1 = 1,
}
impl From<HALT_EUB0_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUB0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB0` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB0_R(crate::FieldReader<bool, HALT_EUB0_A>);
impl HALT_EUB0_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUB0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUB0_A {
        match self.bits {
            false => HALT_EUB0_A::HALT_EUB0_0,
            true => HALT_EUB0_A::HALT_EUB0_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUB0_0`"]
    #[inline(always)]
    pub fn is_halt_e_ub0_0(&self) -> bool {
        **self == HALT_EUB0_A::HALT_EUB0_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUB0_1`"]
    #[inline(always)]
    pub fn is_halt_e_ub0_1(&self) -> bool {
        **self == HALT_EUB0_A::HALT_EUB0_1
    }
}
impl core::ops::Deref for HALT_EUB0_R {
    type Target = crate::FieldReader<bool, HALT_EUB0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUB0` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB0_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUB0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUB0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0_0(self) -> &'a mut W {
        self.variant(HALT_EUB0_A::HALT_EUB0_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0_1(self) -> &'a mut W {
        self.variant(HALT_EUB0_A::HALT_EUB0_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUB1_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUB1_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUB1_1 = 1,
}
impl From<HALT_EUB1_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUB1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB1` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB1_R(crate::FieldReader<bool, HALT_EUB1_A>);
impl HALT_EUB1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUB1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUB1_A {
        match self.bits {
            false => HALT_EUB1_A::HALT_EUB1_0,
            true => HALT_EUB1_A::HALT_EUB1_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUB1_0`"]
    #[inline(always)]
    pub fn is_halt_e_ub1_0(&self) -> bool {
        **self == HALT_EUB1_A::HALT_EUB1_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUB1_1`"]
    #[inline(always)]
    pub fn is_halt_e_ub1_1(&self) -> bool {
        **self == HALT_EUB1_A::HALT_EUB1_1
    }
}
impl core::ops::Deref for HALT_EUB1_R {
    type Target = crate::FieldReader<bool, HALT_EUB1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUB1` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB1_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUB1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUB1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1_0(self) -> &'a mut W {
        self.variant(HALT_EUB1_A::HALT_EUB1_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1_1(self) -> &'a mut W {
        self.variant(HALT_EUB1_A::HALT_EUB1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUB2_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUB2_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUB2_1 = 1,
}
impl From<HALT_EUB2_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUB2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB2` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB2_R(crate::FieldReader<bool, HALT_EUB2_A>);
impl HALT_EUB2_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUB2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUB2_A {
        match self.bits {
            false => HALT_EUB2_A::HALT_EUB2_0,
            true => HALT_EUB2_A::HALT_EUB2_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUB2_0`"]
    #[inline(always)]
    pub fn is_halt_e_ub2_0(&self) -> bool {
        **self == HALT_EUB2_A::HALT_EUB2_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUB2_1`"]
    #[inline(always)]
    pub fn is_halt_e_ub2_1(&self) -> bool {
        **self == HALT_EUB2_A::HALT_EUB2_1
    }
}
impl core::ops::Deref for HALT_EUB2_R {
    type Target = crate::FieldReader<bool, HALT_EUB2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUB2` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB2_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUB2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUB2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2_0(self) -> &'a mut W {
        self.variant(HALT_EUB2_A::HALT_EUB2_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2_1(self) -> &'a mut W {
        self.variant(HALT_EUB2_A::HALT_EUB2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_EUB3_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_EUB3_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_EUB3_1 = 1,
}
impl From<HALT_EUB3_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_EUB3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_eUB3` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB3_R(crate::FieldReader<bool, HALT_EUB3_A>);
impl HALT_EUB3_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_EUB3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_EUB3_A {
        match self.bits {
            false => HALT_EUB3_A::HALT_EUB3_0,
            true => HALT_EUB3_A::HALT_EUB3_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_EUB3_0`"]
    #[inline(always)]
    pub fn is_halt_e_ub3_0(&self) -> bool {
        **self == HALT_EUB3_A::HALT_EUB3_0
    }
    #[doc = "Checks if the value of the field is `HALT_EUB3_1`"]
    #[inline(always)]
    pub fn is_halt_e_ub3_1(&self) -> bool {
        **self == HALT_EUB3_A::HALT_EUB3_1
    }
}
impl core::ops::Deref for HALT_EUB3_R {
    type Target = crate::FieldReader<bool, HALT_EUB3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_eUB3` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_EUB3_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_EUB3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_EUB3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3_0(self) -> &'a mut W {
        self.variant(HALT_EUB3_A::HALT_EUB3_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3_1(self) -> &'a mut W {
        self.variant(HALT_EUB3_A::HALT_EUB3_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_ADC_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_ADC_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_ADC_1 = 1,
}
impl From<HALT_ADC_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_ADC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_ADC` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_ADC_R(crate::FieldReader<bool, HALT_ADC_A>);
impl HALT_ADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_ADC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_ADC_A {
        match self.bits {
            false => HALT_ADC_A::HALT_ADC_0,
            true => HALT_ADC_A::HALT_ADC_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_ADC_0`"]
    #[inline(always)]
    pub fn is_halt_adc_0(&self) -> bool {
        **self == HALT_ADC_A::HALT_ADC_0
    }
    #[doc = "Checks if the value of the field is `HALT_ADC_1`"]
    #[inline(always)]
    pub fn is_halt_adc_1(&self) -> bool {
        **self == HALT_ADC_A::HALT_ADC_1
    }
}
impl core::ops::Deref for HALT_ADC_R {
    type Target = crate::FieldReader<bool, HALT_ADC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_ADC` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_ADC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_ADC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc_0(self) -> &'a mut W {
        self.variant(HALT_ADC_A::HALT_ADC_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc_1(self) -> &'a mut W {
        self.variant(HALT_ADC_A::HALT_ADC_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_WDT_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_WDT_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_WDT_1 = 1,
}
impl From<HALT_WDT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_WDT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_WDT` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_WDT_R(crate::FieldReader<bool, HALT_WDT_A>);
impl HALT_WDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_WDT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_WDT_A {
        match self.bits {
            false => HALT_WDT_A::HALT_WDT_0,
            true => HALT_WDT_A::HALT_WDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_WDT_0`"]
    #[inline(always)]
    pub fn is_halt_wdt_0(&self) -> bool {
        **self == HALT_WDT_A::HALT_WDT_0
    }
    #[doc = "Checks if the value of the field is `HALT_WDT_1`"]
    #[inline(always)]
    pub fn is_halt_wdt_1(&self) -> bool {
        **self == HALT_WDT_A::HALT_WDT_1
    }
}
impl core::ops::Deref for HALT_WDT_R {
    type Target = crate::FieldReader<bool, HALT_WDT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_WDT` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_WDT_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_WDT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_WDT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt_0(self) -> &'a mut W {
        self.variant(HALT_WDT_A::HALT_WDT_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt_1(self) -> &'a mut W {
        self.variant(HALT_WDT_A::HALT_WDT_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Freezes IP operation when CPU is halted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HALT_DMA_A {
    #[doc = "0: IP operation unaffected when CPU is halted"]
    HALT_DMA_0 = 0,
    #[doc = "1: freezes IP operation when CPU is halted"]
    HALT_DMA_1 = 1,
}
impl From<HALT_DMA_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_DMA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HALT_DMA` reader - Freezes IP operation when CPU is halted"]
pub struct HALT_DMA_R(crate::FieldReader<bool, HALT_DMA_A>);
impl HALT_DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALT_DMA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_DMA_A {
        match self.bits {
            false => HALT_DMA_A::HALT_DMA_0,
            true => HALT_DMA_A::HALT_DMA_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALT_DMA_0`"]
    #[inline(always)]
    pub fn is_halt_dma_0(&self) -> bool {
        **self == HALT_DMA_A::HALT_DMA_0
    }
    #[doc = "Checks if the value of the field is `HALT_DMA_1`"]
    #[inline(always)]
    pub fn is_halt_dma_1(&self) -> bool {
        **self == HALT_DMA_A::HALT_DMA_1
    }
}
impl core::ops::Deref for HALT_DMA_R {
    type Target = crate::FieldReader<bool, HALT_DMA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALT_DMA` writer - Freezes IP operation when CPU is halted"]
pub struct HALT_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> HALT_DMA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HALT_DMA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IP operation unaffected when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma_0(self) -> &'a mut W {
        self.variant(HALT_DMA_A::HALT_DMA_0)
    }
    #[doc = "freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma_1(self) -> &'a mut W {
        self.variant(HALT_DMA_A::HALT_DMA_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0(&self) -> HALT_T16_0_R {
        HALT_T16_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1(&self) -> HALT_T16_1_R {
        HALT_T16_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2(&self) -> HALT_T16_2_R {
        HALT_T16_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3(&self) -> HALT_T16_3_R {
        HALT_T16_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0(&self) -> HALT_T32_0_R {
        HALT_T32_0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0(&self) -> HALT_EUA0_R {
        HALT_EUA0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1(&self) -> HALT_EUA1_R {
        HALT_EUA1_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2(&self) -> HALT_EUA2_R {
        HALT_EUA2_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3(&self) -> HALT_EUA3_R {
        HALT_EUA3_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0(&self) -> HALT_EUB0_R {
        HALT_EUB0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1(&self) -> HALT_EUB1_R {
        HALT_EUB1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2(&self) -> HALT_EUB2_R {
        HALT_EUB2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3(&self) -> HALT_EUB3_R {
        HALT_EUB3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc(&self) -> HALT_ADC_R {
        HALT_ADC_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt(&self) -> HALT_WDT_R {
        HALT_WDT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma(&self) -> HALT_DMA_R {
        HALT_DMA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_0(&mut self) -> HALT_T16_0_W {
        HALT_T16_0_W { w: self }
    }
    #[doc = "Bit 1 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_1(&mut self) -> HALT_T16_1_W {
        HALT_T16_1_W { w: self }
    }
    #[doc = "Bit 2 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_2(&mut self) -> HALT_T16_2_W {
        HALT_T16_2_W { w: self }
    }
    #[doc = "Bit 3 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t16_3(&mut self) -> HALT_T16_3_W {
        HALT_T16_3_W { w: self }
    }
    #[doc = "Bit 4 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_t32_0(&mut self) -> HALT_T32_0_W {
        HALT_T32_0_W { w: self }
    }
    #[doc = "Bit 5 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua0(&mut self) -> HALT_EUA0_W {
        HALT_EUA0_W { w: self }
    }
    #[doc = "Bit 6 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua1(&mut self) -> HALT_EUA1_W {
        HALT_EUA1_W { w: self }
    }
    #[doc = "Bit 7 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua2(&mut self) -> HALT_EUA2_W {
        HALT_EUA2_W { w: self }
    }
    #[doc = "Bit 8 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ua3(&mut self) -> HALT_EUA3_W {
        HALT_EUA3_W { w: self }
    }
    #[doc = "Bit 9 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub0(&mut self) -> HALT_EUB0_W {
        HALT_EUB0_W { w: self }
    }
    #[doc = "Bit 10 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub1(&mut self) -> HALT_EUB1_W {
        HALT_EUB1_W { w: self }
    }
    #[doc = "Bit 11 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub2(&mut self) -> HALT_EUB2_W {
        HALT_EUB2_W { w: self }
    }
    #[doc = "Bit 12 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_e_ub3(&mut self) -> HALT_EUB3_W {
        HALT_EUB3_W { w: self }
    }
    #[doc = "Bit 13 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_adc(&mut self) -> HALT_ADC_W {
        HALT_ADC_W { w: self }
    }
    #[doc = "Bit 14 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_wdt(&mut self) -> HALT_WDT_W {
        HALT_WDT_W { w: self }
    }
    #[doc = "Bit 15 - Freezes IP operation when CPU is halted"]
    #[inline(always)]
    pub fn halt_dma(&mut self) -> HALT_DMA_W {
        HALT_DMA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Halt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_perihalt_ctl](index.html) module"]
pub struct SYS_PERIHALT_CTL_SPEC;
impl crate::RegisterSpec for SYS_PERIHALT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_perihalt_ctl::R](R) reader structure"]
impl crate::Readable for SYS_PERIHALT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_perihalt_ctl::W](W) writer structure"]
impl crate::Writable for SYS_PERIHALT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_PERIHALT_CTL to value 0x4000"]
impl crate::Resettable for SYS_PERIHALT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x4000
    }
}
