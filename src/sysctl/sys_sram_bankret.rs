#[doc = "Register `SYS_SRAM_BANKRET` reader"]
pub struct R(crate::R<SYS_SRAM_BANKRET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_BANKRET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_BANKRET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_BANKRET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_SRAM_BANKRET` writer"]
pub struct W(crate::W<SYS_SRAM_BANKRET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_SRAM_BANKRET_SPEC>;
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
impl From<crate::W<SYS_SRAM_BANKRET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_SRAM_BANKRET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BNK0_RET` reader - Bank0 retention"]
pub struct BNK0_RET_R(crate::FieldReader<bool, bool>);
impl BNK0_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK0_RET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BNK0_RET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bank1 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK1_RET_A {
    #[doc = "0: Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    BNK1_RET_0 = 0,
    #[doc = "1: Bank1 of the SRAM is retained in LPM3 and LPM4"]
    BNK1_RET_1 = 1,
}
impl From<BNK1_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK1_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK1_RET` reader - Bank1 retention"]
pub struct BNK1_RET_R(crate::FieldReader<bool, BNK1_RET_A>);
impl BNK1_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK1_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK1_RET_A {
        match self.bits {
            false => BNK1_RET_A::BNK1_RET_0,
            true => BNK1_RET_A::BNK1_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_0`"]
    #[inline(always)]
    pub fn is_bnk1_ret_0(&self) -> bool {
        **self == BNK1_RET_A::BNK1_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK1_RET_1`"]
    #[inline(always)]
    pub fn is_bnk1_ret_1(&self) -> bool {
        **self == BNK1_RET_A::BNK1_RET_1
    }
}
impl core::ops::Deref for BNK1_RET_R {
    type Target = crate::FieldReader<bool, BNK1_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK1_RET` writer - Bank1 retention"]
pub struct BNK1_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK1_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK1_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank1 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_0(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_0)
    }
    #[doc = "Bank1 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk1_ret_1(self) -> &'a mut W {
        self.variant(BNK1_RET_A::BNK1_RET_1)
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
#[doc = "Bank2 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK2_RET_A {
    #[doc = "0: Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    BNK2_RET_0 = 0,
    #[doc = "1: Bank2 of the SRAM is retained in LPM3 and LPM4"]
    BNK2_RET_1 = 1,
}
impl From<BNK2_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK2_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK2_RET` reader - Bank2 retention"]
pub struct BNK2_RET_R(crate::FieldReader<bool, BNK2_RET_A>);
impl BNK2_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK2_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK2_RET_A {
        match self.bits {
            false => BNK2_RET_A::BNK2_RET_0,
            true => BNK2_RET_A::BNK2_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_0`"]
    #[inline(always)]
    pub fn is_bnk2_ret_0(&self) -> bool {
        **self == BNK2_RET_A::BNK2_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK2_RET_1`"]
    #[inline(always)]
    pub fn is_bnk2_ret_1(&self) -> bool {
        **self == BNK2_RET_A::BNK2_RET_1
    }
}
impl core::ops::Deref for BNK2_RET_R {
    type Target = crate::FieldReader<bool, BNK2_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK2_RET` writer - Bank2 retention"]
pub struct BNK2_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK2_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK2_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank2 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_0(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_0)
    }
    #[doc = "Bank2 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk2_ret_1(self) -> &'a mut W {
        self.variant(BNK2_RET_A::BNK2_RET_1)
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
#[doc = "Bank3 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK3_RET_A {
    #[doc = "0: Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    BNK3_RET_0 = 0,
    #[doc = "1: Bank3 of the SRAM is retained in LPM3 and LPM4"]
    BNK3_RET_1 = 1,
}
impl From<BNK3_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK3_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK3_RET` reader - Bank3 retention"]
pub struct BNK3_RET_R(crate::FieldReader<bool, BNK3_RET_A>);
impl BNK3_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK3_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK3_RET_A {
        match self.bits {
            false => BNK3_RET_A::BNK3_RET_0,
            true => BNK3_RET_A::BNK3_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_0`"]
    #[inline(always)]
    pub fn is_bnk3_ret_0(&self) -> bool {
        **self == BNK3_RET_A::BNK3_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK3_RET_1`"]
    #[inline(always)]
    pub fn is_bnk3_ret_1(&self) -> bool {
        **self == BNK3_RET_A::BNK3_RET_1
    }
}
impl core::ops::Deref for BNK3_RET_R {
    type Target = crate::FieldReader<bool, BNK3_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK3_RET` writer - Bank3 retention"]
pub struct BNK3_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK3_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK3_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank3 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_0(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_0)
    }
    #[doc = "Bank3 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk3_ret_1(self) -> &'a mut W {
        self.variant(BNK3_RET_A::BNK3_RET_1)
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
#[doc = "Bank4 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK4_RET_A {
    #[doc = "0: Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    BNK4_RET_0 = 0,
    #[doc = "1: Bank4 of the SRAM is retained in LPM3 and LPM4"]
    BNK4_RET_1 = 1,
}
impl From<BNK4_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK4_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK4_RET` reader - Bank4 retention"]
pub struct BNK4_RET_R(crate::FieldReader<bool, BNK4_RET_A>);
impl BNK4_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK4_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK4_RET_A {
        match self.bits {
            false => BNK4_RET_A::BNK4_RET_0,
            true => BNK4_RET_A::BNK4_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_0`"]
    #[inline(always)]
    pub fn is_bnk4_ret_0(&self) -> bool {
        **self == BNK4_RET_A::BNK4_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK4_RET_1`"]
    #[inline(always)]
    pub fn is_bnk4_ret_1(&self) -> bool {
        **self == BNK4_RET_A::BNK4_RET_1
    }
}
impl core::ops::Deref for BNK4_RET_R {
    type Target = crate::FieldReader<bool, BNK4_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK4_RET` writer - Bank4 retention"]
pub struct BNK4_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK4_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK4_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank4 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_0(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_0)
    }
    #[doc = "Bank4 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk4_ret_1(self) -> &'a mut W {
        self.variant(BNK4_RET_A::BNK4_RET_1)
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
#[doc = "Bank5 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK5_RET_A {
    #[doc = "0: Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    BNK5_RET_0 = 0,
    #[doc = "1: Bank5 of the SRAM is retained in LPM3 and LPM4"]
    BNK5_RET_1 = 1,
}
impl From<BNK5_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK5_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK5_RET` reader - Bank5 retention"]
pub struct BNK5_RET_R(crate::FieldReader<bool, BNK5_RET_A>);
impl BNK5_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK5_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK5_RET_A {
        match self.bits {
            false => BNK5_RET_A::BNK5_RET_0,
            true => BNK5_RET_A::BNK5_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_0`"]
    #[inline(always)]
    pub fn is_bnk5_ret_0(&self) -> bool {
        **self == BNK5_RET_A::BNK5_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK5_RET_1`"]
    #[inline(always)]
    pub fn is_bnk5_ret_1(&self) -> bool {
        **self == BNK5_RET_A::BNK5_RET_1
    }
}
impl core::ops::Deref for BNK5_RET_R {
    type Target = crate::FieldReader<bool, BNK5_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK5_RET` writer - Bank5 retention"]
pub struct BNK5_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK5_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK5_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank5 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_0(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_0)
    }
    #[doc = "Bank5 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk5_ret_1(self) -> &'a mut W {
        self.variant(BNK5_RET_A::BNK5_RET_1)
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
#[doc = "Bank6 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK6_RET_A {
    #[doc = "0: Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    BNK6_RET_0 = 0,
    #[doc = "1: Bank6 of the SRAM is retained in LPM3 and LPM4"]
    BNK6_RET_1 = 1,
}
impl From<BNK6_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK6_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK6_RET` reader - Bank6 retention"]
pub struct BNK6_RET_R(crate::FieldReader<bool, BNK6_RET_A>);
impl BNK6_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK6_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK6_RET_A {
        match self.bits {
            false => BNK6_RET_A::BNK6_RET_0,
            true => BNK6_RET_A::BNK6_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_0`"]
    #[inline(always)]
    pub fn is_bnk6_ret_0(&self) -> bool {
        **self == BNK6_RET_A::BNK6_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK6_RET_1`"]
    #[inline(always)]
    pub fn is_bnk6_ret_1(&self) -> bool {
        **self == BNK6_RET_A::BNK6_RET_1
    }
}
impl core::ops::Deref for BNK6_RET_R {
    type Target = crate::FieldReader<bool, BNK6_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK6_RET` writer - Bank6 retention"]
pub struct BNK6_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK6_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK6_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank6 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_0(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_0)
    }
    #[doc = "Bank6 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk6_ret_1(self) -> &'a mut W {
        self.variant(BNK6_RET_A::BNK6_RET_1)
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
#[doc = "Bank7 retention\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK7_RET_A {
    #[doc = "0: Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    BNK7_RET_0 = 0,
    #[doc = "1: Bank7 of the SRAM is retained in LPM3 and LPM4"]
    BNK7_RET_1 = 1,
}
impl From<BNK7_RET_A> for bool {
    #[inline(always)]
    fn from(variant: BNK7_RET_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK7_RET` reader - Bank7 retention"]
pub struct BNK7_RET_R(crate::FieldReader<bool, BNK7_RET_A>);
impl BNK7_RET_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK7_RET_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK7_RET_A {
        match self.bits {
            false => BNK7_RET_A::BNK7_RET_0,
            true => BNK7_RET_A::BNK7_RET_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_0`"]
    #[inline(always)]
    pub fn is_bnk7_ret_0(&self) -> bool {
        **self == BNK7_RET_A::BNK7_RET_0
    }
    #[doc = "Checks if the value of the field is `BNK7_RET_1`"]
    #[inline(always)]
    pub fn is_bnk7_ret_1(&self) -> bool {
        **self == BNK7_RET_A::BNK7_RET_1
    }
}
impl core::ops::Deref for BNK7_RET_R {
    type Target = crate::FieldReader<bool, BNK7_RET_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BNK7_RET` writer - Bank7 retention"]
pub struct BNK7_RET_W<'a> {
    w: &'a mut W,
}
impl<'a> BNK7_RET_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BNK7_RET_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bank7 of the SRAM is not retained in LPM3 or LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_0(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_0)
    }
    #[doc = "Bank7 of the SRAM is retained in LPM3 and LPM4"]
    #[inline(always)]
    pub fn bnk7_ret_1(self) -> &'a mut W {
        self.variant(BNK7_RET_A::BNK7_RET_1)
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
#[doc = "SRAM ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAM_RDY_A {
    #[doc = "0: SRAM banks are being set up for retention. Entry into LPM3, LPM4 should not be attempted until this bit is set to 1"]
    SRAM_RDY_0 = 0,
    #[doc = "1: SRAM is ready for accesses. All SRAM banks are enabled/disabled for retention according to values of bits 7:0 of this register"]
    SRAM_RDY_1 = 1,
}
impl From<SRAM_RDY_A> for bool {
    #[inline(always)]
    fn from(variant: SRAM_RDY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAM_RDY` reader - SRAM ready"]
pub struct SRAM_RDY_R(crate::FieldReader<bool, SRAM_RDY_A>);
impl SRAM_RDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_RDY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAM_RDY_A {
        match self.bits {
            false => SRAM_RDY_A::SRAM_RDY_0,
            true => SRAM_RDY_A::SRAM_RDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_0`"]
    #[inline(always)]
    pub fn is_sram_rdy_0(&self) -> bool {
        **self == SRAM_RDY_A::SRAM_RDY_0
    }
    #[doc = "Checks if the value of the field is `SRAM_RDY_1`"]
    #[inline(always)]
    pub fn is_sram_rdy_1(&self) -> bool {
        **self == SRAM_RDY_A::SRAM_RDY_1
    }
}
impl core::ops::Deref for SRAM_RDY_R {
    type Target = crate::FieldReader<bool, SRAM_RDY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bank0 retention"]
    #[inline(always)]
    pub fn bnk0_ret(&self) -> BNK0_RET_R {
        BNK0_RET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&self) -> BNK1_RET_R {
        BNK1_RET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&self) -> BNK2_RET_R {
        BNK2_RET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&self) -> BNK3_RET_R {
        BNK3_RET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&self) -> BNK4_RET_R {
        BNK4_RET_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&self) -> BNK5_RET_R {
        BNK5_RET_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&self) -> BNK6_RET_R {
        BNK6_RET_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&self) -> BNK7_RET_R {
        BNK7_RET_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - SRAM ready"]
    #[inline(always)]
    pub fn sram_rdy(&self) -> SRAM_RDY_R {
        SRAM_RDY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Bank1 retention"]
    #[inline(always)]
    pub fn bnk1_ret(&mut self) -> BNK1_RET_W {
        BNK1_RET_W { w: self }
    }
    #[doc = "Bit 2 - Bank2 retention"]
    #[inline(always)]
    pub fn bnk2_ret(&mut self) -> BNK2_RET_W {
        BNK2_RET_W { w: self }
    }
    #[doc = "Bit 3 - Bank3 retention"]
    #[inline(always)]
    pub fn bnk3_ret(&mut self) -> BNK3_RET_W {
        BNK3_RET_W { w: self }
    }
    #[doc = "Bit 4 - Bank4 retention"]
    #[inline(always)]
    pub fn bnk4_ret(&mut self) -> BNK4_RET_W {
        BNK4_RET_W { w: self }
    }
    #[doc = "Bit 5 - Bank5 retention"]
    #[inline(always)]
    pub fn bnk5_ret(&mut self) -> BNK5_RET_W {
        BNK5_RET_W { w: self }
    }
    #[doc = "Bit 6 - Bank6 retention"]
    #[inline(always)]
    pub fn bnk6_ret(&mut self) -> BNK6_RET_W {
        BNK6_RET_W { w: self }
    }
    #[doc = "Bit 7 - Bank7 retention"]
    #[inline(always)]
    pub fn bnk7_ret(&mut self) -> BNK7_RET_W {
        BNK7_RET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Bank Retention Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_bankret](index.html) module"]
pub struct SYS_SRAM_BANKRET_SPEC;
impl crate::RegisterSpec for SYS_SRAM_BANKRET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_bankret::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_BANKRET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_sram_bankret::W](W) writer structure"]
impl crate::Writable for SYS_SRAM_BANKRET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_SRAM_BANKRET to value 0xff"]
impl crate::Resettable for SYS_SRAM_BANKRET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
