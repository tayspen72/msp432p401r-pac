#[doc = "Register `CExCTL3` reader"]
pub struct R(crate::R<CEXCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEXCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEXCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEXCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CExCTL3` writer"]
pub struct W(crate::W<CEXCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CEXCTL3_SPEC>;
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
impl From<crate::W<CEXCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CEXCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD0_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD0_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD0_1 = 1,
}
impl From<CEPD0_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD0` reader - Port disable"]
pub struct CEPD0_R(crate::FieldReader<bool, CEPD0_A>);
impl CEPD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD0_A {
        match self.bits {
            false => CEPD0_A::CEPD0_0,
            true => CEPD0_A::CEPD0_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD0_0`"]
    #[inline(always)]
    pub fn is_cepd0_0(&self) -> bool {
        **self == CEPD0_A::CEPD0_0
    }
    #[doc = "Checks if the value of the field is `CEPD0_1`"]
    #[inline(always)]
    pub fn is_cepd0_1(&self) -> bool {
        **self == CEPD0_A::CEPD0_1
    }
}
impl core::ops::Deref for CEPD0_R {
    type Target = crate::FieldReader<bool, CEPD0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD0` writer - Port disable"]
pub struct CEPD0_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd0_0(self) -> &'a mut W {
        self.variant(CEPD0_A::CEPD0_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd0_1(self) -> &'a mut W {
        self.variant(CEPD0_A::CEPD0_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD1_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD1_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD1_1 = 1,
}
impl From<CEPD1_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD1` reader - Port disable"]
pub struct CEPD1_R(crate::FieldReader<bool, CEPD1_A>);
impl CEPD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD1_A {
        match self.bits {
            false => CEPD1_A::CEPD1_0,
            true => CEPD1_A::CEPD1_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD1_0`"]
    #[inline(always)]
    pub fn is_cepd1_0(&self) -> bool {
        **self == CEPD1_A::CEPD1_0
    }
    #[doc = "Checks if the value of the field is `CEPD1_1`"]
    #[inline(always)]
    pub fn is_cepd1_1(&self) -> bool {
        **self == CEPD1_A::CEPD1_1
    }
}
impl core::ops::Deref for CEPD1_R {
    type Target = crate::FieldReader<bool, CEPD1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD1` writer - Port disable"]
pub struct CEPD1_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd1_0(self) -> &'a mut W {
        self.variant(CEPD1_A::CEPD1_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd1_1(self) -> &'a mut W {
        self.variant(CEPD1_A::CEPD1_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD2_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD2_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD2_1 = 1,
}
impl From<CEPD2_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD2` reader - Port disable"]
pub struct CEPD2_R(crate::FieldReader<bool, CEPD2_A>);
impl CEPD2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD2_A {
        match self.bits {
            false => CEPD2_A::CEPD2_0,
            true => CEPD2_A::CEPD2_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD2_0`"]
    #[inline(always)]
    pub fn is_cepd2_0(&self) -> bool {
        **self == CEPD2_A::CEPD2_0
    }
    #[doc = "Checks if the value of the field is `CEPD2_1`"]
    #[inline(always)]
    pub fn is_cepd2_1(&self) -> bool {
        **self == CEPD2_A::CEPD2_1
    }
}
impl core::ops::Deref for CEPD2_R {
    type Target = crate::FieldReader<bool, CEPD2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD2` writer - Port disable"]
pub struct CEPD2_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd2_0(self) -> &'a mut W {
        self.variant(CEPD2_A::CEPD2_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd2_1(self) -> &'a mut W {
        self.variant(CEPD2_A::CEPD2_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD3_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD3_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD3_1 = 1,
}
impl From<CEPD3_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD3` reader - Port disable"]
pub struct CEPD3_R(crate::FieldReader<bool, CEPD3_A>);
impl CEPD3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD3_A {
        match self.bits {
            false => CEPD3_A::CEPD3_0,
            true => CEPD3_A::CEPD3_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD3_0`"]
    #[inline(always)]
    pub fn is_cepd3_0(&self) -> bool {
        **self == CEPD3_A::CEPD3_0
    }
    #[doc = "Checks if the value of the field is `CEPD3_1`"]
    #[inline(always)]
    pub fn is_cepd3_1(&self) -> bool {
        **self == CEPD3_A::CEPD3_1
    }
}
impl core::ops::Deref for CEPD3_R {
    type Target = crate::FieldReader<bool, CEPD3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD3` writer - Port disable"]
pub struct CEPD3_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd3_0(self) -> &'a mut W {
        self.variant(CEPD3_A::CEPD3_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd3_1(self) -> &'a mut W {
        self.variant(CEPD3_A::CEPD3_1)
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
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD4_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD4_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD4_1 = 1,
}
impl From<CEPD4_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD4` reader - Port disable"]
pub struct CEPD4_R(crate::FieldReader<bool, CEPD4_A>);
impl CEPD4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD4_A {
        match self.bits {
            false => CEPD4_A::CEPD4_0,
            true => CEPD4_A::CEPD4_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD4_0`"]
    #[inline(always)]
    pub fn is_cepd4_0(&self) -> bool {
        **self == CEPD4_A::CEPD4_0
    }
    #[doc = "Checks if the value of the field is `CEPD4_1`"]
    #[inline(always)]
    pub fn is_cepd4_1(&self) -> bool {
        **self == CEPD4_A::CEPD4_1
    }
}
impl core::ops::Deref for CEPD4_R {
    type Target = crate::FieldReader<bool, CEPD4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD4` writer - Port disable"]
pub struct CEPD4_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd4_0(self) -> &'a mut W {
        self.variant(CEPD4_A::CEPD4_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd4_1(self) -> &'a mut W {
        self.variant(CEPD4_A::CEPD4_1)
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
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD5_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD5_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD5_1 = 1,
}
impl From<CEPD5_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD5` reader - Port disable"]
pub struct CEPD5_R(crate::FieldReader<bool, CEPD5_A>);
impl CEPD5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD5_A {
        match self.bits {
            false => CEPD5_A::CEPD5_0,
            true => CEPD5_A::CEPD5_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD5_0`"]
    #[inline(always)]
    pub fn is_cepd5_0(&self) -> bool {
        **self == CEPD5_A::CEPD5_0
    }
    #[doc = "Checks if the value of the field is `CEPD5_1`"]
    #[inline(always)]
    pub fn is_cepd5_1(&self) -> bool {
        **self == CEPD5_A::CEPD5_1
    }
}
impl core::ops::Deref for CEPD5_R {
    type Target = crate::FieldReader<bool, CEPD5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD5` writer - Port disable"]
pub struct CEPD5_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd5_0(self) -> &'a mut W {
        self.variant(CEPD5_A::CEPD5_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd5_1(self) -> &'a mut W {
        self.variant(CEPD5_A::CEPD5_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u16 & 0x01) << 5);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD6_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD6_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD6_1 = 1,
}
impl From<CEPD6_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD6` reader - Port disable"]
pub struct CEPD6_R(crate::FieldReader<bool, CEPD6_A>);
impl CEPD6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD6_A {
        match self.bits {
            false => CEPD6_A::CEPD6_0,
            true => CEPD6_A::CEPD6_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD6_0`"]
    #[inline(always)]
    pub fn is_cepd6_0(&self) -> bool {
        **self == CEPD6_A::CEPD6_0
    }
    #[doc = "Checks if the value of the field is `CEPD6_1`"]
    #[inline(always)]
    pub fn is_cepd6_1(&self) -> bool {
        **self == CEPD6_A::CEPD6_1
    }
}
impl core::ops::Deref for CEPD6_R {
    type Target = crate::FieldReader<bool, CEPD6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD6` writer - Port disable"]
pub struct CEPD6_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd6_0(self) -> &'a mut W {
        self.variant(CEPD6_A::CEPD6_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd6_1(self) -> &'a mut W {
        self.variant(CEPD6_A::CEPD6_1)
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
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD7_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD7_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD7_1 = 1,
}
impl From<CEPD7_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD7` reader - Port disable"]
pub struct CEPD7_R(crate::FieldReader<bool, CEPD7_A>);
impl CEPD7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD7_A {
        match self.bits {
            false => CEPD7_A::CEPD7_0,
            true => CEPD7_A::CEPD7_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD7_0`"]
    #[inline(always)]
    pub fn is_cepd7_0(&self) -> bool {
        **self == CEPD7_A::CEPD7_0
    }
    #[doc = "Checks if the value of the field is `CEPD7_1`"]
    #[inline(always)]
    pub fn is_cepd7_1(&self) -> bool {
        **self == CEPD7_A::CEPD7_1
    }
}
impl core::ops::Deref for CEPD7_R {
    type Target = crate::FieldReader<bool, CEPD7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD7` writer - Port disable"]
pub struct CEPD7_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd7_0(self) -> &'a mut W {
        self.variant(CEPD7_A::CEPD7_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd7_1(self) -> &'a mut W {
        self.variant(CEPD7_A::CEPD7_1)
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
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD8_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD8_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD8_1 = 1,
}
impl From<CEPD8_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD8` reader - Port disable"]
pub struct CEPD8_R(crate::FieldReader<bool, CEPD8_A>);
impl CEPD8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD8_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD8_A {
        match self.bits {
            false => CEPD8_A::CEPD8_0,
            true => CEPD8_A::CEPD8_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD8_0`"]
    #[inline(always)]
    pub fn is_cepd8_0(&self) -> bool {
        **self == CEPD8_A::CEPD8_0
    }
    #[doc = "Checks if the value of the field is `CEPD8_1`"]
    #[inline(always)]
    pub fn is_cepd8_1(&self) -> bool {
        **self == CEPD8_A::CEPD8_1
    }
}
impl core::ops::Deref for CEPD8_R {
    type Target = crate::FieldReader<bool, CEPD8_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD8` writer - Port disable"]
pub struct CEPD8_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD8_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd8_0(self) -> &'a mut W {
        self.variant(CEPD8_A::CEPD8_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd8_1(self) -> &'a mut W {
        self.variant(CEPD8_A::CEPD8_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD9_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD9_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD9_1 = 1,
}
impl From<CEPD9_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD9` reader - Port disable"]
pub struct CEPD9_R(crate::FieldReader<bool, CEPD9_A>);
impl CEPD9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD9_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD9_A {
        match self.bits {
            false => CEPD9_A::CEPD9_0,
            true => CEPD9_A::CEPD9_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD9_0`"]
    #[inline(always)]
    pub fn is_cepd9_0(&self) -> bool {
        **self == CEPD9_A::CEPD9_0
    }
    #[doc = "Checks if the value of the field is `CEPD9_1`"]
    #[inline(always)]
    pub fn is_cepd9_1(&self) -> bool {
        **self == CEPD9_A::CEPD9_1
    }
}
impl core::ops::Deref for CEPD9_R {
    type Target = crate::FieldReader<bool, CEPD9_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD9` writer - Port disable"]
pub struct CEPD9_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD9_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd9_0(self) -> &'a mut W {
        self.variant(CEPD9_A::CEPD9_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd9_1(self) -> &'a mut W {
        self.variant(CEPD9_A::CEPD9_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD10_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD10_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD10_1 = 1,
}
impl From<CEPD10_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD10` reader - Port disable"]
pub struct CEPD10_R(crate::FieldReader<bool, CEPD10_A>);
impl CEPD10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD10_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD10_A {
        match self.bits {
            false => CEPD10_A::CEPD10_0,
            true => CEPD10_A::CEPD10_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD10_0`"]
    #[inline(always)]
    pub fn is_cepd10_0(&self) -> bool {
        **self == CEPD10_A::CEPD10_0
    }
    #[doc = "Checks if the value of the field is `CEPD10_1`"]
    #[inline(always)]
    pub fn is_cepd10_1(&self) -> bool {
        **self == CEPD10_A::CEPD10_1
    }
}
impl core::ops::Deref for CEPD10_R {
    type Target = crate::FieldReader<bool, CEPD10_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD10` writer - Port disable"]
pub struct CEPD10_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD10_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd10_0(self) -> &'a mut W {
        self.variant(CEPD10_A::CEPD10_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd10_1(self) -> &'a mut W {
        self.variant(CEPD10_A::CEPD10_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u16 & 0x01) << 10);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD11_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD11_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD11_1 = 1,
}
impl From<CEPD11_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD11` reader - Port disable"]
pub struct CEPD11_R(crate::FieldReader<bool, CEPD11_A>);
impl CEPD11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD11_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD11_A {
        match self.bits {
            false => CEPD11_A::CEPD11_0,
            true => CEPD11_A::CEPD11_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD11_0`"]
    #[inline(always)]
    pub fn is_cepd11_0(&self) -> bool {
        **self == CEPD11_A::CEPD11_0
    }
    #[doc = "Checks if the value of the field is `CEPD11_1`"]
    #[inline(always)]
    pub fn is_cepd11_1(&self) -> bool {
        **self == CEPD11_A::CEPD11_1
    }
}
impl core::ops::Deref for CEPD11_R {
    type Target = crate::FieldReader<bool, CEPD11_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD11` writer - Port disable"]
pub struct CEPD11_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD11_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd11_0(self) -> &'a mut W {
        self.variant(CEPD11_A::CEPD11_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd11_1(self) -> &'a mut W {
        self.variant(CEPD11_A::CEPD11_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u16 & 0x01) << 11);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD12_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD12_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD12_1 = 1,
}
impl From<CEPD12_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD12` reader - Port disable"]
pub struct CEPD12_R(crate::FieldReader<bool, CEPD12_A>);
impl CEPD12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD12_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD12_A {
        match self.bits {
            false => CEPD12_A::CEPD12_0,
            true => CEPD12_A::CEPD12_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD12_0`"]
    #[inline(always)]
    pub fn is_cepd12_0(&self) -> bool {
        **self == CEPD12_A::CEPD12_0
    }
    #[doc = "Checks if the value of the field is `CEPD12_1`"]
    #[inline(always)]
    pub fn is_cepd12_1(&self) -> bool {
        **self == CEPD12_A::CEPD12_1
    }
}
impl core::ops::Deref for CEPD12_R {
    type Target = crate::FieldReader<bool, CEPD12_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD12` writer - Port disable"]
pub struct CEPD12_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD12_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd12_0(self) -> &'a mut W {
        self.variant(CEPD12_A::CEPD12_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd12_1(self) -> &'a mut W {
        self.variant(CEPD12_A::CEPD12_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u16 & 0x01) << 12);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD13_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD13_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD13_1 = 1,
}
impl From<CEPD13_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD13` reader - Port disable"]
pub struct CEPD13_R(crate::FieldReader<bool, CEPD13_A>);
impl CEPD13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD13_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD13_A {
        match self.bits {
            false => CEPD13_A::CEPD13_0,
            true => CEPD13_A::CEPD13_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD13_0`"]
    #[inline(always)]
    pub fn is_cepd13_0(&self) -> bool {
        **self == CEPD13_A::CEPD13_0
    }
    #[doc = "Checks if the value of the field is `CEPD13_1`"]
    #[inline(always)]
    pub fn is_cepd13_1(&self) -> bool {
        **self == CEPD13_A::CEPD13_1
    }
}
impl core::ops::Deref for CEPD13_R {
    type Target = crate::FieldReader<bool, CEPD13_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD13` writer - Port disable"]
pub struct CEPD13_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD13_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd13_0(self) -> &'a mut W {
        self.variant(CEPD13_A::CEPD13_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd13_1(self) -> &'a mut W {
        self.variant(CEPD13_A::CEPD13_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u16 & 0x01) << 13);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD14_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD14_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD14_1 = 1,
}
impl From<CEPD14_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD14_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD14` reader - Port disable"]
pub struct CEPD14_R(crate::FieldReader<bool, CEPD14_A>);
impl CEPD14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD14_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD14_A {
        match self.bits {
            false => CEPD14_A::CEPD14_0,
            true => CEPD14_A::CEPD14_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD14_0`"]
    #[inline(always)]
    pub fn is_cepd14_0(&self) -> bool {
        **self == CEPD14_A::CEPD14_0
    }
    #[doc = "Checks if the value of the field is `CEPD14_1`"]
    #[inline(always)]
    pub fn is_cepd14_1(&self) -> bool {
        **self == CEPD14_A::CEPD14_1
    }
}
impl core::ops::Deref for CEPD14_R {
    type Target = crate::FieldReader<bool, CEPD14_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD14` writer - Port disable"]
pub struct CEPD14_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD14_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd14_0(self) -> &'a mut W {
        self.variant(CEPD14_A::CEPD14_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd14_1(self) -> &'a mut W {
        self.variant(CEPD14_A::CEPD14_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u16 & 0x01) << 14);
        self.w
    }
}
#[doc = "Port disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEPD15_A {
    #[doc = "0: The input buffer is enabled"]
    CEPD15_0 = 0,
    #[doc = "1: The input buffer is disabled"]
    CEPD15_1 = 1,
}
impl From<CEPD15_A> for bool {
    #[inline(always)]
    fn from(variant: CEPD15_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEPD15` reader - Port disable"]
pub struct CEPD15_R(crate::FieldReader<bool, CEPD15_A>);
impl CEPD15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEPD15_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEPD15_A {
        match self.bits {
            false => CEPD15_A::CEPD15_0,
            true => CEPD15_A::CEPD15_1,
        }
    }
    #[doc = "Checks if the value of the field is `CEPD15_0`"]
    #[inline(always)]
    pub fn is_cepd15_0(&self) -> bool {
        **self == CEPD15_A::CEPD15_0
    }
    #[doc = "Checks if the value of the field is `CEPD15_1`"]
    #[inline(always)]
    pub fn is_cepd15_1(&self) -> bool {
        **self == CEPD15_A::CEPD15_1
    }
}
impl core::ops::Deref for CEPD15_R {
    type Target = crate::FieldReader<bool, CEPD15_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEPD15` writer - Port disable"]
pub struct CEPD15_W<'a> {
    w: &'a mut W,
}
impl<'a> CEPD15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEPD15_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The input buffer is enabled"]
    #[inline(always)]
    pub fn cepd15_0(self) -> &'a mut W {
        self.variant(CEPD15_A::CEPD15_0)
    }
    #[doc = "The input buffer is disabled"]
    #[inline(always)]
    pub fn cepd15_1(self) -> &'a mut W {
        self.variant(CEPD15_A::CEPD15_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u16 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&self) -> CEPD0_R {
        CEPD0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&self) -> CEPD1_R {
        CEPD1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&self) -> CEPD2_R {
        CEPD2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&self) -> CEPD3_R {
        CEPD3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&self) -> CEPD4_R {
        CEPD4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&self) -> CEPD5_R {
        CEPD5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&self) -> CEPD6_R {
        CEPD6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&self) -> CEPD7_R {
        CEPD7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&self) -> CEPD8_R {
        CEPD8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&self) -> CEPD9_R {
        CEPD9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&self) -> CEPD10_R {
        CEPD10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&self) -> CEPD11_R {
        CEPD11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&self) -> CEPD12_R {
        CEPD12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&self) -> CEPD13_R {
        CEPD13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&self) -> CEPD14_R {
        CEPD14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&self) -> CEPD15_R {
        CEPD15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port disable"]
    #[inline(always)]
    pub fn cepd0(&mut self) -> CEPD0_W {
        CEPD0_W { w: self }
    }
    #[doc = "Bit 1 - Port disable"]
    #[inline(always)]
    pub fn cepd1(&mut self) -> CEPD1_W {
        CEPD1_W { w: self }
    }
    #[doc = "Bit 2 - Port disable"]
    #[inline(always)]
    pub fn cepd2(&mut self) -> CEPD2_W {
        CEPD2_W { w: self }
    }
    #[doc = "Bit 3 - Port disable"]
    #[inline(always)]
    pub fn cepd3(&mut self) -> CEPD3_W {
        CEPD3_W { w: self }
    }
    #[doc = "Bit 4 - Port disable"]
    #[inline(always)]
    pub fn cepd4(&mut self) -> CEPD4_W {
        CEPD4_W { w: self }
    }
    #[doc = "Bit 5 - Port disable"]
    #[inline(always)]
    pub fn cepd5(&mut self) -> CEPD5_W {
        CEPD5_W { w: self }
    }
    #[doc = "Bit 6 - Port disable"]
    #[inline(always)]
    pub fn cepd6(&mut self) -> CEPD6_W {
        CEPD6_W { w: self }
    }
    #[doc = "Bit 7 - Port disable"]
    #[inline(always)]
    pub fn cepd7(&mut self) -> CEPD7_W {
        CEPD7_W { w: self }
    }
    #[doc = "Bit 8 - Port disable"]
    #[inline(always)]
    pub fn cepd8(&mut self) -> CEPD8_W {
        CEPD8_W { w: self }
    }
    #[doc = "Bit 9 - Port disable"]
    #[inline(always)]
    pub fn cepd9(&mut self) -> CEPD9_W {
        CEPD9_W { w: self }
    }
    #[doc = "Bit 10 - Port disable"]
    #[inline(always)]
    pub fn cepd10(&mut self) -> CEPD10_W {
        CEPD10_W { w: self }
    }
    #[doc = "Bit 11 - Port disable"]
    #[inline(always)]
    pub fn cepd11(&mut self) -> CEPD11_W {
        CEPD11_W { w: self }
    }
    #[doc = "Bit 12 - Port disable"]
    #[inline(always)]
    pub fn cepd12(&mut self) -> CEPD12_W {
        CEPD12_W { w: self }
    }
    #[doc = "Bit 13 - Port disable"]
    #[inline(always)]
    pub fn cepd13(&mut self) -> CEPD13_W {
        CEPD13_W { w: self }
    }
    #[doc = "Bit 14 - Port disable"]
    #[inline(always)]
    pub fn cepd14(&mut self) -> CEPD14_W {
        CEPD14_W { w: self }
    }
    #[doc = "Bit 15 - Port disable"]
    #[inline(always)]
    pub fn cepd15(&mut self) -> CEPD15_W {
        CEPD15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_ctl3](index.html) module"]
pub struct CEXCTL3_SPEC;
impl crate::RegisterSpec for CEXCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_ctl3::R](R) reader structure"]
impl crate::Readable for CEXCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cex_ctl3::W](W) writer structure"]
impl crate::Writable for CEXCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CExCTL3 to value 0"]
impl crate::Resettable for CEXCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
