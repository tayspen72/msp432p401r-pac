#[doc = "Register `PSSCTL0` reader"]
pub struct R(crate::R<PSSCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PSSCTL0` writer"]
pub struct W(crate::W<PSSCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSSCTL0_SPEC>;
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
impl From<crate::W<PSSCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSSCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SVSM high-side off\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHOFF_A {
    #[doc = "0: The SVSMH is on"]
    SVSMHOFF_0 = 0,
    #[doc = "1: The SVSMH is off"]
    SVSMHOFF_1 = 1,
}
impl From<SVSMHOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHOFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHOFF` reader - SVSM high-side off"]
pub struct SVSMHOFF_R(crate::FieldReader<bool, SVSMHOFF_A>);
impl SVSMHOFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSMHOFF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHOFF_A {
        match self.bits {
            false => SVSMHOFF_A::SVSMHOFF_0,
            true => SVSMHOFF_A::SVSMHOFF_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHOFF_0`"]
    #[inline(always)]
    pub fn is_svsmhoff_0(&self) -> bool {
        **self == SVSMHOFF_A::SVSMHOFF_0
    }
    #[doc = "Checks if the value of the field is `SVSMHOFF_1`"]
    #[inline(always)]
    pub fn is_svsmhoff_1(&self) -> bool {
        **self == SVSMHOFF_A::SVSMHOFF_1
    }
}
impl core::ops::Deref for SVSMHOFF_R {
    type Target = crate::FieldReader<bool, SVSMHOFF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMHOFF` writer - SVSM high-side off"]
pub struct SVSMHOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHOFF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMHOFF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The SVSMH is on"]
    #[inline(always)]
    pub fn svsmhoff_0(self) -> &'a mut W {
        self.variant(SVSMHOFF_A::SVSMHOFF_0)
    }
    #[doc = "The SVSMH is off"]
    #[inline(always)]
    pub fn svsmhoff_1(self) -> &'a mut W {
        self.variant(SVSMHOFF_A::SVSMHOFF_1)
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
#[doc = "SVSM high-side low power normal performance mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHLP_A {
    #[doc = "0: Full performance mode. See the device-specific data sheet for response times."]
    SVSMHLP_0 = 0,
    #[doc = "1: Low power normal performance mode in LPM3, LPM4, and LPMx.5, full performance in all other modes. See the device-specific data sheet for response times."]
    SVSMHLP_1 = 1,
}
impl From<SVSMHLP_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHLP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHLP` reader - SVSM high-side low power normal performance mode"]
pub struct SVSMHLP_R(crate::FieldReader<bool, SVSMHLP_A>);
impl SVSMHLP_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSMHLP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHLP_A {
        match self.bits {
            false => SVSMHLP_A::SVSMHLP_0,
            true => SVSMHLP_A::SVSMHLP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHLP_0`"]
    #[inline(always)]
    pub fn is_svsmhlp_0(&self) -> bool {
        **self == SVSMHLP_A::SVSMHLP_0
    }
    #[doc = "Checks if the value of the field is `SVSMHLP_1`"]
    #[inline(always)]
    pub fn is_svsmhlp_1(&self) -> bool {
        **self == SVSMHLP_A::SVSMHLP_1
    }
}
impl core::ops::Deref for SVSMHLP_R {
    type Target = crate::FieldReader<bool, SVSMHLP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMHLP` writer - SVSM high-side low power normal performance mode"]
pub struct SVSMHLP_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHLP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMHLP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Full performance mode. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn svsmhlp_0(self) -> &'a mut W {
        self.variant(SVSMHLP_A::SVSMHLP_0)
    }
    #[doc = "Low power normal performance mode in LPM3, LPM4, and LPMx.5, full performance in all other modes. See the device-specific data sheet for response times."]
    #[inline(always)]
    pub fn svsmhlp_1(self) -> &'a mut W {
        self.variant(SVSMHLP_A::SVSMHLP_1)
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
#[doc = "Supply supervisor or monitor selection for the high-side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHS_A {
    #[doc = "0: Configure as SVSH"]
    SVSMHS_0 = 0,
    #[doc = "1: Configure as SVMH"]
    SVSMHS_1 = 1,
}
impl From<SVSMHS_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHS` reader - Supply supervisor or monitor selection for the high-side"]
pub struct SVSMHS_R(crate::FieldReader<bool, SVSMHS_A>);
impl SVSMHS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSMHS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHS_A {
        match self.bits {
            false => SVSMHS_A::SVSMHS_0,
            true => SVSMHS_A::SVSMHS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHS_0`"]
    #[inline(always)]
    pub fn is_svsmhs_0(&self) -> bool {
        **self == SVSMHS_A::SVSMHS_0
    }
    #[doc = "Checks if the value of the field is `SVSMHS_1`"]
    #[inline(always)]
    pub fn is_svsmhs_1(&self) -> bool {
        **self == SVSMHS_A::SVSMHS_1
    }
}
impl core::ops::Deref for SVSMHS_R {
    type Target = crate::FieldReader<bool, SVSMHS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMHS` writer - Supply supervisor or monitor selection for the high-side"]
pub struct SVSMHS_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVSMHS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configure as SVSH"]
    #[inline(always)]
    pub fn svsmhs_0(self) -> &'a mut W {
        self.variant(SVSMHS_A::SVSMHS_0)
    }
    #[doc = "Configure as SVMH"]
    #[inline(always)]
    pub fn svsmhs_1(self) -> &'a mut W {
        self.variant(SVSMHS_A::SVSMHS_1)
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
#[doc = "Field `SVSMHTH` reader - SVSM high-side reset voltage level"]
pub struct SVSMHTH_R(crate::FieldReader<u8, u8>);
impl SVSMHTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        SVSMHTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SVSMHTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVSMHTH` writer - SVSM high-side reset voltage level"]
pub struct SVSMHTH_W<'a> {
    w: &'a mut W,
}
impl<'a> SVSMHTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "SVSM high-side output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVMHOE_A {
    #[doc = "0: SVSMHIFG bit is not output"]
    SVMHOE_0 = 0,
    #[doc = "1: SVSMHIFG bit is output to the device SVMHOUT pin. The device-specific port logic must be configured accordingly"]
    SVMHOE_1 = 1,
}
impl From<SVMHOE_A> for bool {
    #[inline(always)]
    fn from(variant: SVMHOE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVMHOE` reader - SVSM high-side output enable"]
pub struct SVMHOE_R(crate::FieldReader<bool, SVMHOE_A>);
impl SVMHOE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVMHOE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVMHOE_A {
        match self.bits {
            false => SVMHOE_A::SVMHOE_0,
            true => SVMHOE_A::SVMHOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVMHOE_0`"]
    #[inline(always)]
    pub fn is_svmhoe_0(&self) -> bool {
        **self == SVMHOE_A::SVMHOE_0
    }
    #[doc = "Checks if the value of the field is `SVMHOE_1`"]
    #[inline(always)]
    pub fn is_svmhoe_1(&self) -> bool {
        **self == SVMHOE_A::SVMHOE_1
    }
}
impl core::ops::Deref for SVMHOE_R {
    type Target = crate::FieldReader<bool, SVMHOE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMHOE` writer - SVSM high-side output enable"]
pub struct SVMHOE_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHOE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVMHOE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SVSMHIFG bit is not output"]
    #[inline(always)]
    pub fn svmhoe_0(self) -> &'a mut W {
        self.variant(SVMHOE_A::SVMHOE_0)
    }
    #[doc = "SVSMHIFG bit is output to the device SVMHOUT pin. The device-specific port logic must be configured accordingly"]
    #[inline(always)]
    pub fn svmhoe_1(self) -> &'a mut W {
        self.variant(SVMHOE_A::SVMHOE_1)
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
#[doc = "SVMHOUT pin polarity active low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVMHOUTPOLAL_A {
    #[doc = "0: SVMHOUT is active high. An error condition is signaled by a 1 at the SVMHOUT pin"]
    SVMHOUTPOLAL_0 = 0,
    #[doc = "1: SVMHOUT is active low. An error condition is signaled by a 0 at the SVMHOUT pin"]
    SVMHOUTPOLAL_1 = 1,
}
impl From<SVMHOUTPOLAL_A> for bool {
    #[inline(always)]
    fn from(variant: SVMHOUTPOLAL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVMHOUTPOLAL` reader - SVMHOUT pin polarity active low"]
pub struct SVMHOUTPOLAL_R(crate::FieldReader<bool, SVMHOUTPOLAL_A>);
impl SVMHOUTPOLAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVMHOUTPOLAL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVMHOUTPOLAL_A {
        match self.bits {
            false => SVMHOUTPOLAL_A::SVMHOUTPOLAL_0,
            true => SVMHOUTPOLAL_A::SVMHOUTPOLAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVMHOUTPOLAL_0`"]
    #[inline(always)]
    pub fn is_svmhoutpolal_0(&self) -> bool {
        **self == SVMHOUTPOLAL_A::SVMHOUTPOLAL_0
    }
    #[doc = "Checks if the value of the field is `SVMHOUTPOLAL_1`"]
    #[inline(always)]
    pub fn is_svmhoutpolal_1(&self) -> bool {
        **self == SVMHOUTPOLAL_A::SVMHOUTPOLAL_1
    }
}
impl core::ops::Deref for SVMHOUTPOLAL_R {
    type Target = crate::FieldReader<bool, SVMHOUTPOLAL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SVMHOUTPOLAL` writer - SVMHOUT pin polarity active low"]
pub struct SVMHOUTPOLAL_W<'a> {
    w: &'a mut W,
}
impl<'a> SVMHOUTPOLAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVMHOUTPOLAL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "SVMHOUT is active high. An error condition is signaled by a 1 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn svmhoutpolal_0(self) -> &'a mut W {
        self.variant(SVMHOUTPOLAL_A::SVMHOUTPOLAL_0)
    }
    #[doc = "SVMHOUT is active low. An error condition is signaled by a 0 at the SVMHOUT pin"]
    #[inline(always)]
    pub fn svmhoutpolal_1(self) -> &'a mut W {
        self.variant(SVMHOUTPOLAL_A::SVMHOUTPOLAL_1)
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
#[doc = "Force DC-DC regulator operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDC_FORCE_A {
    #[doc = "0: DC-DC regulator operation not forced. Automatic fail-safe mechanism switches the core voltage regulator from DC-DC to LDO when the supply voltage falls below the minimum supply voltage necessary for DC-DC operation."]
    DCDC_FORCE_0 = 0,
    #[doc = "1: DC-DC regulator operation forced. Automatic fail-safe mechanism is disabled and device continues to operate out of DC-DC regulator."]
    DCDC_FORCE_1 = 1,
}
impl From<DCDC_FORCE_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_FORCE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCDC_FORCE` reader - Force DC-DC regulator operation"]
pub struct DCDC_FORCE_R(crate::FieldReader<bool, DCDC_FORCE_A>);
impl DCDC_FORCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDC_FORCE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_FORCE_A {
        match self.bits {
            false => DCDC_FORCE_A::DCDC_FORCE_0,
            true => DCDC_FORCE_A::DCDC_FORCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_FORCE_0`"]
    #[inline(always)]
    pub fn is_dcdc_force_0(&self) -> bool {
        **self == DCDC_FORCE_A::DCDC_FORCE_0
    }
    #[doc = "Checks if the value of the field is `DCDC_FORCE_1`"]
    #[inline(always)]
    pub fn is_dcdc_force_1(&self) -> bool {
        **self == DCDC_FORCE_A::DCDC_FORCE_1
    }
}
impl core::ops::Deref for DCDC_FORCE_R {
    type Target = crate::FieldReader<bool, DCDC_FORCE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDC_FORCE` writer - Force DC-DC regulator operation"]
pub struct DCDC_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDC_FORCE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCDC_FORCE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "DC-DC regulator operation not forced. Automatic fail-safe mechanism switches the core voltage regulator from DC-DC to LDO when the supply voltage falls below the minimum supply voltage necessary for DC-DC operation."]
    #[inline(always)]
    pub fn dcdc_force_0(self) -> &'a mut W {
        self.variant(DCDC_FORCE_A::DCDC_FORCE_0)
    }
    #[doc = "DC-DC regulator operation forced. Automatic fail-safe mechanism is disabled and device continues to operate out of DC-DC regulator."]
    #[inline(always)]
    pub fn dcdc_force_1(self) -> &'a mut W {
        self.variant(DCDC_FORCE_A::DCDC_FORCE_1)
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
#[doc = "Controls core voltage level transition time\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VCORETRAN_A {
    #[doc = "0: 32 s / 100 mV"]
    VCORETRAN_0 = 0,
    #[doc = "1: 64 s / 100 mV"]
    VCORETRAN_1 = 1,
    #[doc = "2: 128 s / 100 mV (default)"]
    VCORETRAN_2 = 2,
    #[doc = "3: 256 s / 100 mV"]
    VCORETRAN_3 = 3,
}
impl From<VCORETRAN_A> for u8 {
    #[inline(always)]
    fn from(variant: VCORETRAN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VCORETRAN` reader - Controls core voltage level transition time"]
pub struct VCORETRAN_R(crate::FieldReader<u8, VCORETRAN_A>);
impl VCORETRAN_R {
    pub(crate) fn new(bits: u8) -> Self {
        VCORETRAN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCORETRAN_A {
        match self.bits {
            0 => VCORETRAN_A::VCORETRAN_0,
            1 => VCORETRAN_A::VCORETRAN_1,
            2 => VCORETRAN_A::VCORETRAN_2,
            3 => VCORETRAN_A::VCORETRAN_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VCORETRAN_0`"]
    #[inline(always)]
    pub fn is_vcoretran_0(&self) -> bool {
        **self == VCORETRAN_A::VCORETRAN_0
    }
    #[doc = "Checks if the value of the field is `VCORETRAN_1`"]
    #[inline(always)]
    pub fn is_vcoretran_1(&self) -> bool {
        **self == VCORETRAN_A::VCORETRAN_1
    }
    #[doc = "Checks if the value of the field is `VCORETRAN_2`"]
    #[inline(always)]
    pub fn is_vcoretran_2(&self) -> bool {
        **self == VCORETRAN_A::VCORETRAN_2
    }
    #[doc = "Checks if the value of the field is `VCORETRAN_3`"]
    #[inline(always)]
    pub fn is_vcoretran_3(&self) -> bool {
        **self == VCORETRAN_A::VCORETRAN_3
    }
}
impl core::ops::Deref for VCORETRAN_R {
    type Target = crate::FieldReader<u8, VCORETRAN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VCORETRAN` writer - Controls core voltage level transition time"]
pub struct VCORETRAN_W<'a> {
    w: &'a mut W,
}
impl<'a> VCORETRAN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VCORETRAN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "32 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_0(self) -> &'a mut W {
        self.variant(VCORETRAN_A::VCORETRAN_0)
    }
    #[doc = "64 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_1(self) -> &'a mut W {
        self.variant(VCORETRAN_A::VCORETRAN_1)
    }
    #[doc = "128 s / 100 mV (default)"]
    #[inline(always)]
    pub fn vcoretran_2(self) -> &'a mut W {
        self.variant(VCORETRAN_A::VCORETRAN_2)
    }
    #[doc = "256 s / 100 mV"]
    #[inline(always)]
    pub fn vcoretran_3(self) -> &'a mut W {
        self.variant(VCORETRAN_A::VCORETRAN_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SVSM high-side off"]
    #[inline(always)]
    pub fn svsmhoff(&self) -> SVSMHOFF_R {
        SVSMHOFF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SVSM high-side low power normal performance mode"]
    #[inline(always)]
    pub fn svsmhlp(&self) -> SVSMHLP_R {
        SVSMHLP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Supply supervisor or monitor selection for the high-side"]
    #[inline(always)]
    pub fn svsmhs(&self) -> SVSMHS_R {
        SVSMHS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 3:5 - SVSM high-side reset voltage level"]
    #[inline(always)]
    pub fn svsmhth(&self) -> SVSMHTH_R {
        SVSMHTH_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - SVSM high-side output enable"]
    #[inline(always)]
    pub fn svmhoe(&self) -> SVMHOE_R {
        SVMHOE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SVMHOUT pin polarity active low"]
    #[inline(always)]
    pub fn svmhoutpolal(&self) -> SVMHOUTPOLAL_R {
        SVMHOUTPOLAL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Force DC-DC regulator operation"]
    #[inline(always)]
    pub fn dcdc_force(&self) -> DCDC_FORCE_R {
        DCDC_FORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Controls core voltage level transition time"]
    #[inline(always)]
    pub fn vcoretran(&self) -> VCORETRAN_R {
        VCORETRAN_R::new(((self.bits >> 12) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SVSM high-side off"]
    #[inline(always)]
    pub fn svsmhoff(&mut self) -> SVSMHOFF_W {
        SVSMHOFF_W { w: self }
    }
    #[doc = "Bit 1 - SVSM high-side low power normal performance mode"]
    #[inline(always)]
    pub fn svsmhlp(&mut self) -> SVSMHLP_W {
        SVSMHLP_W { w: self }
    }
    #[doc = "Bit 2 - Supply supervisor or monitor selection for the high-side"]
    #[inline(always)]
    pub fn svsmhs(&mut self) -> SVSMHS_W {
        SVSMHS_W { w: self }
    }
    #[doc = "Bits 3:5 - SVSM high-side reset voltage level"]
    #[inline(always)]
    pub fn svsmhth(&mut self) -> SVSMHTH_W {
        SVSMHTH_W { w: self }
    }
    #[doc = "Bit 6 - SVSM high-side output enable"]
    #[inline(always)]
    pub fn svmhoe(&mut self) -> SVMHOE_W {
        SVMHOE_W { w: self }
    }
    #[doc = "Bit 7 - SVMHOUT pin polarity active low"]
    #[inline(always)]
    pub fn svmhoutpolal(&mut self) -> SVMHOUTPOLAL_W {
        SVMHOUTPOLAL_W { w: self }
    }
    #[doc = "Bit 10 - Force DC-DC regulator operation"]
    #[inline(always)]
    pub fn dcdc_force(&mut self) -> DCDC_FORCE_W {
        DCDC_FORCE_W { w: self }
    }
    #[doc = "Bits 12:13 - Controls core voltage level transition time"]
    #[inline(always)]
    pub fn vcoretran(&mut self) -> VCORETRAN_W {
        VCORETRAN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssctl0](index.html) module"]
pub struct PSSCTL0_SPEC;
impl crate::RegisterSpec for PSSCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssctl0::R](R) reader structure"]
impl crate::Readable for PSSCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pssctl0::W](W) writer structure"]
impl crate::Writable for PSSCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PSSCTL0 to value 0x2000"]
impl crate::Resettable for PSSCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2000
    }
}
