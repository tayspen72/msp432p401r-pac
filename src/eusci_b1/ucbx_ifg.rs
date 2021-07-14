#[doc = "Register `UCBxIFG` reader"]
pub struct R(crate::R<UCBXIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxIFG` writer"]
pub struct W(crate::W<UCBXIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXIFG_SPEC>;
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
impl From<crate::W<UCBXIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "eUSCI_B receive interrupt flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG0_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG0_1 = 1,
}
impl From<UCRXIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG0` reader - eUSCI_B receive interrupt flag 0"]
pub struct UCRXIFG0_R(crate::FieldReader<bool, UCRXIFG0_A>);
impl UCRXIFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG0_A {
        match self.bits {
            false => UCRXIFG0_A::UCRXIFG0_0,
            true => UCRXIFG0_A::UCRXIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0_0`"]
    #[inline(always)]
    pub fn is_ucrxifg0_0(&self) -> bool {
        **self == UCRXIFG0_A::UCRXIFG0_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG0_1`"]
    #[inline(always)]
    pub fn is_ucrxifg0_1(&self) -> bool {
        **self == UCRXIFG0_A::UCRXIFG0_1
    }
}
impl core::ops::Deref for UCRXIFG0_R {
    type Target = crate::FieldReader<bool, UCRXIFG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG0` writer - eUSCI_B receive interrupt flag 0"]
pub struct UCRXIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_0(self) -> &'a mut W {
        self.variant(UCRXIFG0_A::UCRXIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg0_1(self) -> &'a mut W {
        self.variant(UCRXIFG0_A::UCRXIFG0_1)
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
#[doc = "eUSCI_B transmit interrupt flag 0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG0_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG0_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG0_1 = 1,
}
impl From<UCTXIFG0_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG0` reader - eUSCI_B transmit interrupt flag 0"]
pub struct UCTXIFG0_R(crate::FieldReader<bool, UCTXIFG0_A>);
impl UCTXIFG0_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG0_A {
        match self.bits {
            false => UCTXIFG0_A::UCTXIFG0_0,
            true => UCTXIFG0_A::UCTXIFG0_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0_0`"]
    #[inline(always)]
    pub fn is_uctxifg0_0(&self) -> bool {
        **self == UCTXIFG0_A::UCTXIFG0_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG0_1`"]
    #[inline(always)]
    pub fn is_uctxifg0_1(&self) -> bool {
        **self == UCTXIFG0_A::UCTXIFG0_1
    }
}
impl core::ops::Deref for UCTXIFG0_R {
    type Target = crate::FieldReader<bool, UCTXIFG0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG0` writer - eUSCI_B transmit interrupt flag 0"]
pub struct UCTXIFG0_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_0(self) -> &'a mut W {
        self.variant(UCTXIFG0_A::UCTXIFG0_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg0_1(self) -> &'a mut W {
        self.variant(UCTXIFG0_A::UCTXIFG0_1)
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
#[doc = "START condition interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCSTTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCSTTIFG_1 = 1,
}
impl From<UCSTTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIFG` reader - START condition interrupt flag"]
pub struct UCSTTIFG_R(crate::FieldReader<bool, UCSTTIFG_A>);
impl UCSTTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIFG_A {
        match self.bits {
            false => UCSTTIFG_A::UCSTTIFG_0,
            true => UCSTTIFG_A::UCSTTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_0`"]
    #[inline(always)]
    pub fn is_ucsttifg_0(&self) -> bool {
        **self == UCSTTIFG_A::UCSTTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_1`"]
    #[inline(always)]
    pub fn is_ucsttifg_1(&self) -> bool {
        **self == UCSTTIFG_A::UCSTTIFG_1
    }
}
impl core::ops::Deref for UCSTTIFG_R {
    type Target = crate::FieldReader<bool, UCSTTIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIFG` writer - START condition interrupt flag"]
pub struct UCSTTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTTIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_0(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_1(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_1)
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
#[doc = "STOP condition interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTPIFG_A {
    #[doc = "0: No interrupt pending"]
    UCSTPIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCSTPIFG_1 = 1,
}
impl From<UCSTPIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTPIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTPIFG` reader - STOP condition interrupt flag"]
pub struct UCSTPIFG_R(crate::FieldReader<bool, UCSTPIFG_A>);
impl UCSTPIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTPIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTPIFG_A {
        match self.bits {
            false => UCSTPIFG_A::UCSTPIFG_0,
            true => UCSTPIFG_A::UCSTPIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG_0`"]
    #[inline(always)]
    pub fn is_ucstpifg_0(&self) -> bool {
        **self == UCSTPIFG_A::UCSTPIFG_0
    }
    #[doc = "Checks if the value of the field is `UCSTPIFG_1`"]
    #[inline(always)]
    pub fn is_ucstpifg_1(&self) -> bool {
        **self == UCSTPIFG_A::UCSTPIFG_1
    }
}
impl core::ops::Deref for UCSTPIFG_R {
    type Target = crate::FieldReader<bool, UCSTPIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTPIFG` writer - STOP condition interrupt flag"]
pub struct UCSTPIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTPIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTPIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_0(self) -> &'a mut W {
        self.variant(UCSTPIFG_A::UCSTPIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucstpifg_1(self) -> &'a mut W {
        self.variant(UCSTPIFG_A::UCSTPIFG_1)
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
#[doc = "Arbitration lost interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCALIFG_A {
    #[doc = "0: No interrupt pending"]
    UCALIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCALIFG_1 = 1,
}
impl From<UCALIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCALIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCALIFG` reader - Arbitration lost interrupt flag"]
pub struct UCALIFG_R(crate::FieldReader<bool, UCALIFG_A>);
impl UCALIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCALIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCALIFG_A {
        match self.bits {
            false => UCALIFG_A::UCALIFG_0,
            true => UCALIFG_A::UCALIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCALIFG_0`"]
    #[inline(always)]
    pub fn is_ucalifg_0(&self) -> bool {
        **self == UCALIFG_A::UCALIFG_0
    }
    #[doc = "Checks if the value of the field is `UCALIFG_1`"]
    #[inline(always)]
    pub fn is_ucalifg_1(&self) -> bool {
        **self == UCALIFG_A::UCALIFG_1
    }
}
impl core::ops::Deref for UCALIFG_R {
    type Target = crate::FieldReader<bool, UCALIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCALIFG` writer - Arbitration lost interrupt flag"]
pub struct UCALIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCALIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCALIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_0(self) -> &'a mut W {
        self.variant(UCALIFG_A::UCALIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucalifg_1(self) -> &'a mut W {
        self.variant(UCALIFG_A::UCALIFG_1)
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
#[doc = "Not-acknowledge received interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCNACKIFG_A {
    #[doc = "0: No interrupt pending"]
    UCNACKIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCNACKIFG_1 = 1,
}
impl From<UCNACKIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCNACKIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCNACKIFG` reader - Not-acknowledge received interrupt flag"]
pub struct UCNACKIFG_R(crate::FieldReader<bool, UCNACKIFG_A>);
impl UCNACKIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCNACKIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCNACKIFG_A {
        match self.bits {
            false => UCNACKIFG_A::UCNACKIFG_0,
            true => UCNACKIFG_A::UCNACKIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG_0`"]
    #[inline(always)]
    pub fn is_ucnackifg_0(&self) -> bool {
        **self == UCNACKIFG_A::UCNACKIFG_0
    }
    #[doc = "Checks if the value of the field is `UCNACKIFG_1`"]
    #[inline(always)]
    pub fn is_ucnackifg_1(&self) -> bool {
        **self == UCNACKIFG_A::UCNACKIFG_1
    }
}
impl core::ops::Deref for UCNACKIFG_R {
    type Target = crate::FieldReader<bool, UCNACKIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCNACKIFG` writer - Not-acknowledge received interrupt flag"]
pub struct UCNACKIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCNACKIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCNACKIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_0(self) -> &'a mut W {
        self.variant(UCNACKIFG_A::UCNACKIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucnackifg_1(self) -> &'a mut W {
        self.variant(UCNACKIFG_A::UCNACKIFG_1)
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
#[doc = "Byte counter interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBCNTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCBCNTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCBCNTIFG_1 = 1,
}
impl From<UCBCNTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCBCNTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBCNTIFG` reader - Byte counter interrupt flag"]
pub struct UCBCNTIFG_R(crate::FieldReader<bool, UCBCNTIFG_A>);
impl UCBCNTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBCNTIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBCNTIFG_A {
        match self.bits {
            false => UCBCNTIFG_A::UCBCNTIFG_0,
            true => UCBCNTIFG_A::UCBCNTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG_0`"]
    #[inline(always)]
    pub fn is_ucbcntifg_0(&self) -> bool {
        **self == UCBCNTIFG_A::UCBCNTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCBCNTIFG_1`"]
    #[inline(always)]
    pub fn is_ucbcntifg_1(&self) -> bool {
        **self == UCBCNTIFG_A::UCBCNTIFG_1
    }
}
impl core::ops::Deref for UCBCNTIFG_R {
    type Target = crate::FieldReader<bool, UCBCNTIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBCNTIFG` writer - Byte counter interrupt flag"]
pub struct UCBCNTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBCNTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBCNTIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_0(self) -> &'a mut W {
        self.variant(UCBCNTIFG_A::UCBCNTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbcntifg_1(self) -> &'a mut W {
        self.variant(UCBCNTIFG_A::UCBCNTIFG_1)
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
#[doc = "Clock low timeout interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCCLTOIFG_A {
    #[doc = "0: No interrupt pending"]
    UCCLTOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCCLTOIFG_1 = 1,
}
impl From<UCCLTOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCCLTOIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCCLTOIFG` reader - Clock low timeout interrupt flag"]
pub struct UCCLTOIFG_R(crate::FieldReader<bool, UCCLTOIFG_A>);
impl UCCLTOIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCCLTOIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCCLTOIFG_A {
        match self.bits {
            false => UCCLTOIFG_A::UCCLTOIFG_0,
            true => UCCLTOIFG_A::UCCLTOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG_0`"]
    #[inline(always)]
    pub fn is_uccltoifg_0(&self) -> bool {
        **self == UCCLTOIFG_A::UCCLTOIFG_0
    }
    #[doc = "Checks if the value of the field is `UCCLTOIFG_1`"]
    #[inline(always)]
    pub fn is_uccltoifg_1(&self) -> bool {
        **self == UCCLTOIFG_A::UCCLTOIFG_1
    }
}
impl core::ops::Deref for UCCLTOIFG_R {
    type Target = crate::FieldReader<bool, UCCLTOIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCCLTOIFG` writer - Clock low timeout interrupt flag"]
pub struct UCCLTOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCCLTOIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCCLTOIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_0(self) -> &'a mut W {
        self.variant(UCCLTOIFG_A::UCCLTOIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uccltoifg_1(self) -> &'a mut W {
        self.variant(UCCLTOIFG_A::UCCLTOIFG_1)
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
#[doc = "eUSCI_B receive interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG1_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG1_1 = 1,
}
impl From<UCRXIFG1_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG1` reader - eUSCI_B receive interrupt flag 1"]
pub struct UCRXIFG1_R(crate::FieldReader<bool, UCRXIFG1_A>);
impl UCRXIFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG1_A {
        match self.bits {
            false => UCRXIFG1_A::UCRXIFG1_0,
            true => UCRXIFG1_A::UCRXIFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1_0`"]
    #[inline(always)]
    pub fn is_ucrxifg1_0(&self) -> bool {
        **self == UCRXIFG1_A::UCRXIFG1_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG1_1`"]
    #[inline(always)]
    pub fn is_ucrxifg1_1(&self) -> bool {
        **self == UCRXIFG1_A::UCRXIFG1_1
    }
}
impl core::ops::Deref for UCRXIFG1_R {
    type Target = crate::FieldReader<bool, UCRXIFG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG1` writer - eUSCI_B receive interrupt flag 1"]
pub struct UCRXIFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_0(self) -> &'a mut W {
        self.variant(UCRXIFG1_A::UCRXIFG1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg1_1(self) -> &'a mut W {
        self.variant(UCRXIFG1_A::UCRXIFG1_1)
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
#[doc = "eUSCI_B transmit interrupt flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG1_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG1_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG1_1 = 1,
}
impl From<UCTXIFG1_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG1` reader - eUSCI_B transmit interrupt flag 1"]
pub struct UCTXIFG1_R(crate::FieldReader<bool, UCTXIFG1_A>);
impl UCTXIFG1_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG1_A {
        match self.bits {
            false => UCTXIFG1_A::UCTXIFG1_0,
            true => UCTXIFG1_A::UCTXIFG1_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1_0`"]
    #[inline(always)]
    pub fn is_uctxifg1_0(&self) -> bool {
        **self == UCTXIFG1_A::UCTXIFG1_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG1_1`"]
    #[inline(always)]
    pub fn is_uctxifg1_1(&self) -> bool {
        **self == UCTXIFG1_A::UCTXIFG1_1
    }
}
impl core::ops::Deref for UCTXIFG1_R {
    type Target = crate::FieldReader<bool, UCTXIFG1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG1` writer - eUSCI_B transmit interrupt flag 1"]
pub struct UCTXIFG1_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_0(self) -> &'a mut W {
        self.variant(UCTXIFG1_A::UCTXIFG1_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg1_1(self) -> &'a mut W {
        self.variant(UCTXIFG1_A::UCTXIFG1_1)
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
#[doc = "eUSCI_B receive interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG2_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG2_1 = 1,
}
impl From<UCRXIFG2_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG2` reader - eUSCI_B receive interrupt flag 2"]
pub struct UCRXIFG2_R(crate::FieldReader<bool, UCRXIFG2_A>);
impl UCRXIFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG2_A {
        match self.bits {
            false => UCRXIFG2_A::UCRXIFG2_0,
            true => UCRXIFG2_A::UCRXIFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2_0`"]
    #[inline(always)]
    pub fn is_ucrxifg2_0(&self) -> bool {
        **self == UCRXIFG2_A::UCRXIFG2_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG2_1`"]
    #[inline(always)]
    pub fn is_ucrxifg2_1(&self) -> bool {
        **self == UCRXIFG2_A::UCRXIFG2_1
    }
}
impl core::ops::Deref for UCRXIFG2_R {
    type Target = crate::FieldReader<bool, UCRXIFG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG2` writer - eUSCI_B receive interrupt flag 2"]
pub struct UCRXIFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_0(self) -> &'a mut W {
        self.variant(UCRXIFG2_A::UCRXIFG2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg2_1(self) -> &'a mut W {
        self.variant(UCRXIFG2_A::UCRXIFG2_1)
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
#[doc = "eUSCI_B transmit interrupt flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG2_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG2_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG2_1 = 1,
}
impl From<UCTXIFG2_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG2` reader - eUSCI_B transmit interrupt flag 2"]
pub struct UCTXIFG2_R(crate::FieldReader<bool, UCTXIFG2_A>);
impl UCTXIFG2_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG2_A {
        match self.bits {
            false => UCTXIFG2_A::UCTXIFG2_0,
            true => UCTXIFG2_A::UCTXIFG2_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2_0`"]
    #[inline(always)]
    pub fn is_uctxifg2_0(&self) -> bool {
        **self == UCTXIFG2_A::UCTXIFG2_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG2_1`"]
    #[inline(always)]
    pub fn is_uctxifg2_1(&self) -> bool {
        **self == UCTXIFG2_A::UCTXIFG2_1
    }
}
impl core::ops::Deref for UCTXIFG2_R {
    type Target = crate::FieldReader<bool, UCTXIFG2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG2` writer - eUSCI_B transmit interrupt flag 2"]
pub struct UCTXIFG2_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_0(self) -> &'a mut W {
        self.variant(UCTXIFG2_A::UCTXIFG2_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg2_1(self) -> &'a mut W {
        self.variant(UCTXIFG2_A::UCTXIFG2_1)
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
#[doc = "eUSCI_B receive interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG3_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG3_1 = 1,
}
impl From<UCRXIFG3_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG3` reader - eUSCI_B receive interrupt flag 3"]
pub struct UCRXIFG3_R(crate::FieldReader<bool, UCRXIFG3_A>);
impl UCRXIFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG3_A {
        match self.bits {
            false => UCRXIFG3_A::UCRXIFG3_0,
            true => UCRXIFG3_A::UCRXIFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3_0`"]
    #[inline(always)]
    pub fn is_ucrxifg3_0(&self) -> bool {
        **self == UCRXIFG3_A::UCRXIFG3_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG3_1`"]
    #[inline(always)]
    pub fn is_ucrxifg3_1(&self) -> bool {
        **self == UCRXIFG3_A::UCRXIFG3_1
    }
}
impl core::ops::Deref for UCRXIFG3_R {
    type Target = crate::FieldReader<bool, UCRXIFG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG3` writer - eUSCI_B receive interrupt flag 3"]
pub struct UCRXIFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_0(self) -> &'a mut W {
        self.variant(UCRXIFG3_A::UCRXIFG3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg3_1(self) -> &'a mut W {
        self.variant(UCRXIFG3_A::UCRXIFG3_1)
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
#[doc = "eUSCI_B transmit interrupt flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG3_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG3_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG3_1 = 1,
}
impl From<UCTXIFG3_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG3` reader - eUSCI_B transmit interrupt flag 3"]
pub struct UCTXIFG3_R(crate::FieldReader<bool, UCTXIFG3_A>);
impl UCTXIFG3_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG3_A {
        match self.bits {
            false => UCTXIFG3_A::UCTXIFG3_0,
            true => UCTXIFG3_A::UCTXIFG3_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3_0`"]
    #[inline(always)]
    pub fn is_uctxifg3_0(&self) -> bool {
        **self == UCTXIFG3_A::UCTXIFG3_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG3_1`"]
    #[inline(always)]
    pub fn is_uctxifg3_1(&self) -> bool {
        **self == UCTXIFG3_A::UCTXIFG3_1
    }
}
impl core::ops::Deref for UCTXIFG3_R {
    type Target = crate::FieldReader<bool, UCTXIFG3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG3` writer - eUSCI_B transmit interrupt flag 3"]
pub struct UCTXIFG3_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_0(self) -> &'a mut W {
        self.variant(UCTXIFG3_A::UCTXIFG3_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg3_1(self) -> &'a mut W {
        self.variant(UCTXIFG3_A::UCTXIFG3_1)
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
#[doc = "Bit position 9 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCBIT9IFG_A {
    #[doc = "0: No interrupt pending"]
    UCBIT9IFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCBIT9IFG_1 = 1,
}
impl From<UCBIT9IFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCBIT9IFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCBIT9IFG` reader - Bit position 9 interrupt flag"]
pub struct UCBIT9IFG_R(crate::FieldReader<bool, UCBIT9IFG_A>);
impl UCBIT9IFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCBIT9IFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCBIT9IFG_A {
        match self.bits {
            false => UCBIT9IFG_A::UCBIT9IFG_0,
            true => UCBIT9IFG_A::UCBIT9IFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG_0`"]
    #[inline(always)]
    pub fn is_ucbit9ifg_0(&self) -> bool {
        **self == UCBIT9IFG_A::UCBIT9IFG_0
    }
    #[doc = "Checks if the value of the field is `UCBIT9IFG_1`"]
    #[inline(always)]
    pub fn is_ucbit9ifg_1(&self) -> bool {
        **self == UCBIT9IFG_A::UCBIT9IFG_1
    }
}
impl core::ops::Deref for UCBIT9IFG_R {
    type Target = crate::FieldReader<bool, UCBIT9IFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCBIT9IFG` writer - Bit position 9 interrupt flag"]
pub struct UCBIT9IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCBIT9IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCBIT9IFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_0(self) -> &'a mut W {
        self.variant(UCBIT9IFG_A::UCBIT9IFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucbit9ifg_1(self) -> &'a mut W {
        self.variant(UCBIT9IFG_A::UCBIT9IFG_1)
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
impl R {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&self) -> UCRXIFG0_R {
        UCRXIFG0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&self) -> UCTXIFG0_R {
        UCTXIFG0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&self) -> UCSTPIFG_R {
        UCSTPIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&self) -> UCALIFG_R {
        UCALIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&self) -> UCNACKIFG_R {
        UCNACKIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&self) -> UCBCNTIFG_R {
        UCBCNTIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&self) -> UCCLTOIFG_R {
        UCCLTOIFG_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&self) -> UCRXIFG1_R {
        UCRXIFG1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&self) -> UCTXIFG1_R {
        UCTXIFG1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&self) -> UCRXIFG2_R {
        UCRXIFG2_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&self) -> UCTXIFG2_R {
        UCTXIFG2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&self) -> UCRXIFG3_R {
        UCRXIFG3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&self) -> UCTXIFG3_R {
        UCTXIFG3_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&self) -> UCBIT9IFG_R {
        UCBIT9IFG_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - eUSCI_B receive interrupt flag 0"]
    #[inline(always)]
    pub fn ucrxifg0(&mut self) -> UCRXIFG0_W {
        UCRXIFG0_W { w: self }
    }
    #[doc = "Bit 1 - eUSCI_B transmit interrupt flag 0"]
    #[inline(always)]
    pub fn uctxifg0(&mut self) -> UCTXIFG0_W {
        UCTXIFG0_W { w: self }
    }
    #[doc = "Bit 2 - START condition interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W {
        UCSTTIFG_W { w: self }
    }
    #[doc = "Bit 3 - STOP condition interrupt flag"]
    #[inline(always)]
    pub fn ucstpifg(&mut self) -> UCSTPIFG_W {
        UCSTPIFG_W { w: self }
    }
    #[doc = "Bit 4 - Arbitration lost interrupt flag"]
    #[inline(always)]
    pub fn ucalifg(&mut self) -> UCALIFG_W {
        UCALIFG_W { w: self }
    }
    #[doc = "Bit 5 - Not-acknowledge received interrupt flag"]
    #[inline(always)]
    pub fn ucnackifg(&mut self) -> UCNACKIFG_W {
        UCNACKIFG_W { w: self }
    }
    #[doc = "Bit 6 - Byte counter interrupt flag"]
    #[inline(always)]
    pub fn ucbcntifg(&mut self) -> UCBCNTIFG_W {
        UCBCNTIFG_W { w: self }
    }
    #[doc = "Bit 7 - Clock low timeout interrupt flag"]
    #[inline(always)]
    pub fn uccltoifg(&mut self) -> UCCLTOIFG_W {
        UCCLTOIFG_W { w: self }
    }
    #[doc = "Bit 8 - eUSCI_B receive interrupt flag 1"]
    #[inline(always)]
    pub fn ucrxifg1(&mut self) -> UCRXIFG1_W {
        UCRXIFG1_W { w: self }
    }
    #[doc = "Bit 9 - eUSCI_B transmit interrupt flag 1"]
    #[inline(always)]
    pub fn uctxifg1(&mut self) -> UCTXIFG1_W {
        UCTXIFG1_W { w: self }
    }
    #[doc = "Bit 10 - eUSCI_B receive interrupt flag 2"]
    #[inline(always)]
    pub fn ucrxifg2(&mut self) -> UCRXIFG2_W {
        UCRXIFG2_W { w: self }
    }
    #[doc = "Bit 11 - eUSCI_B transmit interrupt flag 2"]
    #[inline(always)]
    pub fn uctxifg2(&mut self) -> UCTXIFG2_W {
        UCTXIFG2_W { w: self }
    }
    #[doc = "Bit 12 - eUSCI_B receive interrupt flag 3"]
    #[inline(always)]
    pub fn ucrxifg3(&mut self) -> UCRXIFG3_W {
        UCRXIFG3_W { w: self }
    }
    #[doc = "Bit 13 - eUSCI_B transmit interrupt flag 3"]
    #[inline(always)]
    pub fn uctxifg3(&mut self) -> UCTXIFG3_W {
        UCTXIFG3_W { w: self }
    }
    #[doc = "Bit 14 - Bit position 9 interrupt flag"]
    #[inline(always)]
    pub fn ucbit9ifg(&mut self) -> UCBIT9IFG_W {
        UCBIT9IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_ifg](index.html) module"]
pub struct UCBXIFG_SPEC;
impl crate::RegisterSpec for UCBXIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_ifg::R](R) reader structure"]
impl crate::Readable for UCBXIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_ifg::W](W) writer structure"]
impl crate::Writable for UCBXIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxIFG to value 0x02"]
impl crate::Resettable for UCBXIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
