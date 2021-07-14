#[doc = "Register `CSCLKEN` reader"]
pub struct R(crate::R<CSCLKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCLKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCLKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCLKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCLKEN` writer"]
pub struct W(crate::W<CSCLKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCLKEN_SPEC>;
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
impl From<crate::W<CSCLKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCLKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ACLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ACLK_EN_A {
    #[doc = "0: ACLK disabled regardless of conditional clock requests"]
    ACLK_EN_0 = 0,
    #[doc = "1: ACLK enabled based on any conditional clock requests"]
    ACLK_EN_1 = 1,
}
impl From<ACLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACLK_EN` reader - ACLK system clock conditional request enable"]
pub struct ACLK_EN_R(crate::FieldReader<bool, ACLK_EN_A>);
impl ACLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACLK_EN_A {
        match self.bits {
            false => ACLK_EN_A::ACLK_EN_0,
            true => ACLK_EN_A::ACLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACLK_EN_0`"]
    #[inline(always)]
    pub fn is_aclk_en_0(&self) -> bool {
        **self == ACLK_EN_A::ACLK_EN_0
    }
    #[doc = "Checks if the value of the field is `ACLK_EN_1`"]
    #[inline(always)]
    pub fn is_aclk_en_1(&self) -> bool {
        **self == ACLK_EN_A::ACLK_EN_1
    }
}
impl core::ops::Deref for ACLK_EN_R {
    type Target = crate::FieldReader<bool, ACLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACLK_EN` writer - ACLK system clock conditional request enable"]
pub struct ACLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ACLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ACLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn aclk_en_0(self) -> &'a mut W {
        self.variant(ACLK_EN_A::ACLK_EN_0)
    }
    #[doc = "ACLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn aclk_en_1(self) -> &'a mut W {
        self.variant(ACLK_EN_A::ACLK_EN_1)
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
#[doc = "MCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MCLK_EN_A {
    #[doc = "0: MCLK disabled regardless of conditional clock requests"]
    MCLK_EN_0 = 0,
    #[doc = "1: MCLK enabled based on any conditional clock requests"]
    MCLK_EN_1 = 1,
}
impl From<MCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCLK_EN` reader - MCLK system clock conditional request enable"]
pub struct MCLK_EN_R(crate::FieldReader<bool, MCLK_EN_A>);
impl MCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLK_EN_A {
        match self.bits {
            false => MCLK_EN_A::MCLK_EN_0,
            true => MCLK_EN_A::MCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MCLK_EN_0`"]
    #[inline(always)]
    pub fn is_mclk_en_0(&self) -> bool {
        **self == MCLK_EN_A::MCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `MCLK_EN_1`"]
    #[inline(always)]
    pub fn is_mclk_en_1(&self) -> bool {
        **self == MCLK_EN_A::MCLK_EN_1
    }
}
impl core::ops::Deref for MCLK_EN_R {
    type Target = crate::FieldReader<bool, MCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLK_EN` writer - MCLK system clock conditional request enable"]
pub struct MCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn mclk_en_0(self) -> &'a mut W {
        self.variant(MCLK_EN_A::MCLK_EN_0)
    }
    #[doc = "MCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn mclk_en_1(self) -> &'a mut W {
        self.variant(MCLK_EN_A::MCLK_EN_1)
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
#[doc = "HSMCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HSMCLK_EN_A {
    #[doc = "0: HSMCLK disabled regardless of conditional clock requests"]
    HSMCLK_EN_0 = 0,
    #[doc = "1: HSMCLK enabled based on any conditional clock requests"]
    HSMCLK_EN_1 = 1,
}
impl From<HSMCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HSMCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSMCLK_EN` reader - HSMCLK system clock conditional request enable"]
pub struct HSMCLK_EN_R(crate::FieldReader<bool, HSMCLK_EN_A>);
impl HSMCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSMCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSMCLK_EN_A {
        match self.bits {
            false => HSMCLK_EN_A::HSMCLK_EN_0,
            true => HSMCLK_EN_A::HSMCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `HSMCLK_EN_0`"]
    #[inline(always)]
    pub fn is_hsmclk_en_0(&self) -> bool {
        **self == HSMCLK_EN_A::HSMCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `HSMCLK_EN_1`"]
    #[inline(always)]
    pub fn is_hsmclk_en_1(&self) -> bool {
        **self == HSMCLK_EN_A::HSMCLK_EN_1
    }
}
impl core::ops::Deref for HSMCLK_EN_R {
    type Target = crate::FieldReader<bool, HSMCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HSMCLK_EN` writer - HSMCLK system clock conditional request enable"]
pub struct HSMCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HSMCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HSMCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSMCLK disabled regardless of conditional clock requests"]
    #[inline(always)]
    pub fn hsmclk_en_0(self) -> &'a mut W {
        self.variant(HSMCLK_EN_A::HSMCLK_EN_0)
    }
    #[doc = "HSMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn hsmclk_en_1(self) -> &'a mut W {
        self.variant(HSMCLK_EN_A::HSMCLK_EN_1)
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
#[doc = "SMCLK system clock conditional request enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMCLK_EN_A {
    #[doc = "0: SMCLK disabled regardless of conditional clock requests."]
    SMCLK_EN_0 = 0,
    #[doc = "1: SMCLK enabled based on any conditional clock requests"]
    SMCLK_EN_1 = 1,
}
impl From<SMCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SMCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMCLK_EN` reader - SMCLK system clock conditional request enable"]
pub struct SMCLK_EN_R(crate::FieldReader<bool, SMCLK_EN_A>);
impl SMCLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMCLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMCLK_EN_A {
        match self.bits {
            false => SMCLK_EN_A::SMCLK_EN_0,
            true => SMCLK_EN_A::SMCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMCLK_EN_0`"]
    #[inline(always)]
    pub fn is_smclk_en_0(&self) -> bool {
        **self == SMCLK_EN_A::SMCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `SMCLK_EN_1`"]
    #[inline(always)]
    pub fn is_smclk_en_1(&self) -> bool {
        **self == SMCLK_EN_A::SMCLK_EN_1
    }
}
impl core::ops::Deref for SMCLK_EN_R {
    type Target = crate::FieldReader<bool, SMCLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMCLK_EN` writer - SMCLK system clock conditional request enable"]
pub struct SMCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMCLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SMCLK disabled regardless of conditional clock requests."]
    #[inline(always)]
    pub fn smclk_en_0(self) -> &'a mut W {
        self.variant(SMCLK_EN_A::SMCLK_EN_0)
    }
    #[doc = "SMCLK enabled based on any conditional clock requests"]
    #[inline(always)]
    pub fn smclk_en_1(self) -> &'a mut W {
        self.variant(SMCLK_EN_A::SMCLK_EN_1)
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
#[doc = "Turns on the VLO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VLO_EN_A {
    #[doc = "0: VLO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK."]
    VLO_EN_0 = 0,
    #[doc = "1: VLO is on"]
    VLO_EN_1 = 1,
}
impl From<VLO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: VLO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VLO_EN` reader - Turns on the VLO oscillator"]
pub struct VLO_EN_R(crate::FieldReader<bool, VLO_EN_A>);
impl VLO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VLO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLO_EN_A {
        match self.bits {
            false => VLO_EN_A::VLO_EN_0,
            true => VLO_EN_A::VLO_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLO_EN_0`"]
    #[inline(always)]
    pub fn is_vlo_en_0(&self) -> bool {
        **self == VLO_EN_A::VLO_EN_0
    }
    #[doc = "Checks if the value of the field is `VLO_EN_1`"]
    #[inline(always)]
    pub fn is_vlo_en_1(&self) -> bool {
        **self == VLO_EN_A::VLO_EN_1
    }
}
impl core::ops::Deref for VLO_EN_R {
    type Target = crate::FieldReader<bool, VLO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VLO_EN` writer - Turns on the VLO oscillator"]
pub struct VLO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> VLO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VLO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "VLO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK."]
    #[inline(always)]
    pub fn vlo_en_0(self) -> &'a mut W {
        self.variant(VLO_EN_A::VLO_EN_0)
    }
    #[doc = "VLO is on"]
    #[inline(always)]
    pub fn vlo_en_1(self) -> &'a mut W {
        self.variant(VLO_EN_A::VLO_EN_1)
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
#[doc = "Turns on the REFO oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFO_EN_A {
    #[doc = "0: REFO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    REFO_EN_0 = 0,
    #[doc = "1: REFO is on"]
    REFO_EN_1 = 1,
}
impl From<REFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: REFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFO_EN` reader - Turns on the REFO oscillator"]
pub struct REFO_EN_R(crate::FieldReader<bool, REFO_EN_A>);
impl REFO_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFO_EN_A {
        match self.bits {
            false => REFO_EN_A::REFO_EN_0,
            true => REFO_EN_A::REFO_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFO_EN_0`"]
    #[inline(always)]
    pub fn is_refo_en_0(&self) -> bool {
        **self == REFO_EN_A::REFO_EN_0
    }
    #[doc = "Checks if the value of the field is `REFO_EN_1`"]
    #[inline(always)]
    pub fn is_refo_en_1(&self) -> bool {
        **self == REFO_EN_A::REFO_EN_1
    }
}
impl core::ops::Deref for REFO_EN_R {
    type Target = crate::FieldReader<bool, REFO_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFO_EN` writer - Turns on the REFO oscillator"]
pub struct REFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "REFO is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn refo_en_0(self) -> &'a mut W {
        self.variant(REFO_EN_A::REFO_EN_0)
    }
    #[doc = "REFO is on"]
    #[inline(always)]
    pub fn refo_en_1(self) -> &'a mut W {
        self.variant(REFO_EN_A::REFO_EN_1)
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
#[doc = "Turns on the MODOSC oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODOSC_EN_A {
    #[doc = "0: MODOSC is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    MODOSC_EN_0 = 0,
    #[doc = "1: MODOSC is on"]
    MODOSC_EN_1 = 1,
}
impl From<MODOSC_EN_A> for bool {
    #[inline(always)]
    fn from(variant: MODOSC_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODOSC_EN` reader - Turns on the MODOSC oscillator"]
pub struct MODOSC_EN_R(crate::FieldReader<bool, MODOSC_EN_A>);
impl MODOSC_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODOSC_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODOSC_EN_A {
        match self.bits {
            false => MODOSC_EN_A::MODOSC_EN_0,
            true => MODOSC_EN_A::MODOSC_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODOSC_EN_0`"]
    #[inline(always)]
    pub fn is_modosc_en_0(&self) -> bool {
        **self == MODOSC_EN_A::MODOSC_EN_0
    }
    #[doc = "Checks if the value of the field is `MODOSC_EN_1`"]
    #[inline(always)]
    pub fn is_modosc_en_1(&self) -> bool {
        **self == MODOSC_EN_A::MODOSC_EN_1
    }
}
impl core::ops::Deref for MODOSC_EN_R {
    type Target = crate::FieldReader<bool, MODOSC_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODOSC_EN` writer - Turns on the MODOSC oscillator"]
pub struct MODOSC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODOSC_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODOSC_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MODOSC is on only if it is used as a source for ACLK, MCLK, HSMCLK or SMCLK"]
    #[inline(always)]
    pub fn modosc_en_0(self) -> &'a mut W {
        self.variant(MODOSC_EN_A::MODOSC_EN_0)
    }
    #[doc = "MODOSC is on"]
    #[inline(always)]
    pub fn modosc_en_1(self) -> &'a mut W {
        self.variant(MODOSC_EN_A::MODOSC_EN_1)
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
#[doc = "Selects REFO nominal frequency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum REFOFSEL_A {
    #[doc = "0: 32 kHz"]
    REFOFSEL_0 = 0,
    #[doc = "1: 128 kHz"]
    REFOFSEL_1 = 1,
}
impl From<REFOFSEL_A> for bool {
    #[inline(always)]
    fn from(variant: REFOFSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFOFSEL` reader - Selects REFO nominal frequency"]
pub struct REFOFSEL_R(crate::FieldReader<bool, REFOFSEL_A>);
impl REFOFSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        REFOFSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFOFSEL_A {
        match self.bits {
            false => REFOFSEL_A::REFOFSEL_0,
            true => REFOFSEL_A::REFOFSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `REFOFSEL_0`"]
    #[inline(always)]
    pub fn is_refofsel_0(&self) -> bool {
        **self == REFOFSEL_A::REFOFSEL_0
    }
    #[doc = "Checks if the value of the field is `REFOFSEL_1`"]
    #[inline(always)]
    pub fn is_refofsel_1(&self) -> bool {
        **self == REFOFSEL_A::REFOFSEL_1
    }
}
impl core::ops::Deref for REFOFSEL_R {
    type Target = crate::FieldReader<bool, REFOFSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REFOFSEL` writer - Selects REFO nominal frequency"]
pub struct REFOFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFOFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: REFOFSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "32 kHz"]
    #[inline(always)]
    pub fn refofsel_0(self) -> &'a mut W {
        self.variant(REFOFSEL_A::REFOFSEL_0)
    }
    #[doc = "128 kHz"]
    #[inline(always)]
    pub fn refofsel_1(self) -> &'a mut W {
        self.variant(REFOFSEL_A::REFOFSEL_1)
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
    #[doc = "Bit 0 - ACLK system clock conditional request enable"]
    #[inline(always)]
    pub fn aclk_en(&self) -> ACLK_EN_R {
        ACLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn mclk_en(&self) -> MCLK_EN_R {
        MCLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HSMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn hsmclk_en(&self) -> HSMCLK_EN_R {
        HSMCLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn smclk_en(&self) -> SMCLK_EN_R {
        SMCLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Turns on the VLO oscillator"]
    #[inline(always)]
    pub fn vlo_en(&self) -> VLO_EN_R {
        VLO_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Turns on the REFO oscillator"]
    #[inline(always)]
    pub fn refo_en(&self) -> REFO_EN_R {
        REFO_EN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Turns on the MODOSC oscillator"]
    #[inline(always)]
    pub fn modosc_en(&self) -> MODOSC_EN_R {
        MODOSC_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Selects REFO nominal frequency"]
    #[inline(always)]
    pub fn refofsel(&self) -> REFOFSEL_R {
        REFOFSEL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACLK system clock conditional request enable"]
    #[inline(always)]
    pub fn aclk_en(&mut self) -> ACLK_EN_W {
        ACLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - MCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn mclk_en(&mut self) -> MCLK_EN_W {
        MCLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - HSMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn hsmclk_en(&mut self) -> HSMCLK_EN_W {
        HSMCLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - SMCLK system clock conditional request enable"]
    #[inline(always)]
    pub fn smclk_en(&mut self) -> SMCLK_EN_W {
        SMCLK_EN_W { w: self }
    }
    #[doc = "Bit 8 - Turns on the VLO oscillator"]
    #[inline(always)]
    pub fn vlo_en(&mut self) -> VLO_EN_W {
        VLO_EN_W { w: self }
    }
    #[doc = "Bit 9 - Turns on the REFO oscillator"]
    #[inline(always)]
    pub fn refo_en(&mut self) -> REFO_EN_W {
        REFO_EN_W { w: self }
    }
    #[doc = "Bit 10 - Turns on the MODOSC oscillator"]
    #[inline(always)]
    pub fn modosc_en(&mut self) -> MODOSC_EN_W {
        MODOSC_EN_W { w: self }
    }
    #[doc = "Bit 15 - Selects REFO nominal frequency"]
    #[inline(always)]
    pub fn refofsel(&mut self) -> REFOFSEL_W {
        REFOFSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csclken](index.html) module"]
pub struct CSCLKEN_SPEC;
impl crate::RegisterSpec for CSCLKEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csclken::R](R) reader structure"]
impl crate::Readable for CSCLKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csclken::W](W) writer structure"]
impl crate::Writable for CSCLKEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCLKEN to value 0x0f"]
impl crate::Resettable for CSCLKEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
