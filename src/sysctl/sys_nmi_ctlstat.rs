#[doc = "Register `SYS_NMI_CTLSTAT` reader"]
pub struct R(crate::R<SYS_NMI_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_NMI_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_NMI_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_NMI_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_NMI_CTLSTAT` writer"]
pub struct W(crate::W<SYS_NMI_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_NMI_CTLSTAT_SPEC>;
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
impl From<crate::W<SYS_NMI_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_NMI_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "CS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_SRC_A {
    #[doc = "0: Disables CS interrupt as a source of NMI"]
    CS_SRC_0 = 0,
    #[doc = "1: Enables CS interrupt as a source of NMI"]
    CS_SRC_1 = 1,
}
impl From<CS_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: CS_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_SRC` reader - CS interrupt as a source of NMI"]
pub struct CS_SRC_R(crate::FieldReader<bool, CS_SRC_A>);
impl CS_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_SRC_A {
        match self.bits {
            false => CS_SRC_A::CS_SRC_0,
            true => CS_SRC_A::CS_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `CS_SRC_0`"]
    #[inline(always)]
    pub fn is_cs_src_0(&self) -> bool {
        **self == CS_SRC_A::CS_SRC_0
    }
    #[doc = "Checks if the value of the field is `CS_SRC_1`"]
    #[inline(always)]
    pub fn is_cs_src_1(&self) -> bool {
        **self == CS_SRC_A::CS_SRC_1
    }
}
impl core::ops::Deref for CS_SRC_R {
    type Target = crate::FieldReader<bool, CS_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS_SRC` writer - CS interrupt as a source of NMI"]
pub struct CS_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_0(self) -> &'a mut W {
        self.variant(CS_SRC_A::CS_SRC_0)
    }
    #[doc = "Enables CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src_1(self) -> &'a mut W {
        self.variant(CS_SRC_A::CS_SRC_1)
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
#[doc = "PSS interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSS_SRC_A {
    #[doc = "0: Disables the PSS interrupt as a source of NMI"]
    PSS_SRC_0 = 0,
    #[doc = "1: Enables the PSS interrupt as a source of NMI"]
    PSS_SRC_1 = 1,
}
impl From<PSS_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PSS_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_SRC` reader - PSS interrupt as a source of NMI"]
pub struct PSS_SRC_R(crate::FieldReader<bool, PSS_SRC_A>);
impl PSS_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSS_SRC_A {
        match self.bits {
            false => PSS_SRC_A::PSS_SRC_0,
            true => PSS_SRC_A::PSS_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSS_SRC_0`"]
    #[inline(always)]
    pub fn is_pss_src_0(&self) -> bool {
        **self == PSS_SRC_A::PSS_SRC_0
    }
    #[doc = "Checks if the value of the field is `PSS_SRC_1`"]
    #[inline(always)]
    pub fn is_pss_src_1(&self) -> bool {
        **self == PSS_SRC_A::PSS_SRC_1
    }
}
impl core::ops::Deref for PSS_SRC_R {
    type Target = crate::FieldReader<bool, PSS_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSS_SRC` writer - PSS interrupt as a source of NMI"]
pub struct PSS_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PSS_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSS_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_0(self) -> &'a mut W {
        self.variant(PSS_SRC_A::PSS_SRC_0)
    }
    #[doc = "Enables the PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src_1(self) -> &'a mut W {
        self.variant(PSS_SRC_A::PSS_SRC_1)
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
#[doc = "PCM interrupt as a source of NMI\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_SRC_A {
    #[doc = "0: Disbles the PCM interrupt as a source of NMI"]
    PCM_SRC_0 = 0,
    #[doc = "1: Enables the PCM interrupt as a source of NMI"]
    PCM_SRC_1 = 1,
}
impl From<PCM_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_SRC` reader - PCM interrupt as a source of NMI"]
pub struct PCM_SRC_R(crate::FieldReader<bool, PCM_SRC_A>);
impl PCM_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCM_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_SRC_A {
        match self.bits {
            false => PCM_SRC_A::PCM_SRC_0,
            true => PCM_SRC_A::PCM_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_SRC_0`"]
    #[inline(always)]
    pub fn is_pcm_src_0(&self) -> bool {
        **self == PCM_SRC_A::PCM_SRC_0
    }
    #[doc = "Checks if the value of the field is `PCM_SRC_1`"]
    #[inline(always)]
    pub fn is_pcm_src_1(&self) -> bool {
        **self == PCM_SRC_A::PCM_SRC_1
    }
}
impl core::ops::Deref for PCM_SRC_R {
    type Target = crate::FieldReader<bool, PCM_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCM_SRC` writer - PCM interrupt as a source of NMI"]
pub struct PCM_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCM_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCM_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disbles the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_0(self) -> &'a mut W {
        self.variant(PCM_SRC_A::PCM_SRC_0)
    }
    #[doc = "Enables the PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src_1(self) -> &'a mut W {
        self.variant(PCM_SRC_A::PCM_SRC_1)
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
#[doc = "RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_SRC_A {
    #[doc = "0: Configures the RSTn_NMI pin as a source of POR Class Reset"]
    PIN_SRC_0 = 0,
    #[doc = "1: Configures the RSTn_NMI pin as a source of NMI"]
    PIN_SRC_1 = 1,
}
impl From<PIN_SRC_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_SRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_SRC` reader - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub struct PIN_SRC_R(crate::FieldReader<bool, PIN_SRC_A>);
impl PIN_SRC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_SRC_A {
        match self.bits {
            false => PIN_SRC_A::PIN_SRC_0,
            true => PIN_SRC_A::PIN_SRC_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_SRC_0`"]
    #[inline(always)]
    pub fn is_pin_src_0(&self) -> bool {
        **self == PIN_SRC_A::PIN_SRC_0
    }
    #[doc = "Checks if the value of the field is `PIN_SRC_1`"]
    #[inline(always)]
    pub fn is_pin_src_1(&self) -> bool {
        **self == PIN_SRC_A::PIN_SRC_1
    }
}
impl core::ops::Deref for PIN_SRC_R {
    type Target = crate::FieldReader<bool, PIN_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_SRC` writer - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
pub struct PIN_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_SRC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configures the RSTn_NMI pin as a source of POR Class Reset"]
    #[inline(always)]
    pub fn pin_src_0(self) -> &'a mut W {
        self.variant(PIN_SRC_A::PIN_SRC_0)
    }
    #[doc = "Configures the RSTn_NMI pin as a source of NMI"]
    #[inline(always)]
    pub fn pin_src_1(self) -> &'a mut W {
        self.variant(PIN_SRC_A::PIN_SRC_1)
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
#[doc = "CS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_FLG_A {
    #[doc = "0: indicates CS interrupt was not the source of NMI"]
    CS_FLG_0 = 0,
    #[doc = "1: indicates CS interrupt was the source of NMI"]
    CS_FLG_1 = 1,
}
impl From<CS_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: CS_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_FLG` reader - CS interrupt was the source of NMI"]
pub struct CS_FLG_R(crate::FieldReader<bool, CS_FLG_A>);
impl CS_FLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_FLG_A {
        match self.bits {
            false => CS_FLG_A::CS_FLG_0,
            true => CS_FLG_A::CS_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `CS_FLG_0`"]
    #[inline(always)]
    pub fn is_cs_flg_0(&self) -> bool {
        **self == CS_FLG_A::CS_FLG_0
    }
    #[doc = "Checks if the value of the field is `CS_FLG_1`"]
    #[inline(always)]
    pub fn is_cs_flg_1(&self) -> bool {
        **self == CS_FLG_A::CS_FLG_1
    }
}
impl core::ops::Deref for CS_FLG_R {
    type Target = crate::FieldReader<bool, CS_FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PSS interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PSS_FLG_A {
    #[doc = "0: indicates the PSS interrupt was not the source of NMI"]
    PSS_FLG_0 = 0,
    #[doc = "1: indicates the PSS interrupt was the source of NMI"]
    PSS_FLG_1 = 1,
}
impl From<PSS_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PSS_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PSS_FLG` reader - PSS interrupt was the source of NMI"]
pub struct PSS_FLG_R(crate::FieldReader<bool, PSS_FLG_A>);
impl PSS_FLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSS_FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSS_FLG_A {
        match self.bits {
            false => PSS_FLG_A::PSS_FLG_0,
            true => PSS_FLG_A::PSS_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PSS_FLG_0`"]
    #[inline(always)]
    pub fn is_pss_flg_0(&self) -> bool {
        **self == PSS_FLG_A::PSS_FLG_0
    }
    #[doc = "Checks if the value of the field is `PSS_FLG_1`"]
    #[inline(always)]
    pub fn is_pss_flg_1(&self) -> bool {
        **self == PSS_FLG_A::PSS_FLG_1
    }
}
impl core::ops::Deref for PSS_FLG_R {
    type Target = crate::FieldReader<bool, PSS_FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "PCM interrupt was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PCM_FLG_A {
    #[doc = "0: indicates the PCM interrupt was not the source of NMI"]
    PCM_FLG_0 = 0,
    #[doc = "1: indicates the PCM interrupt was the source of NMI"]
    PCM_FLG_1 = 1,
}
impl From<PCM_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PCM_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PCM_FLG` reader - PCM interrupt was the source of NMI"]
pub struct PCM_FLG_R(crate::FieldReader<bool, PCM_FLG_A>);
impl PCM_FLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCM_FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCM_FLG_A {
        match self.bits {
            false => PCM_FLG_A::PCM_FLG_0,
            true => PCM_FLG_A::PCM_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PCM_FLG_0`"]
    #[inline(always)]
    pub fn is_pcm_flg_0(&self) -> bool {
        **self == PCM_FLG_A::PCM_FLG_0
    }
    #[doc = "Checks if the value of the field is `PCM_FLG_1`"]
    #[inline(always)]
    pub fn is_pcm_flg_1(&self) -> bool {
        **self == PCM_FLG_A::PCM_FLG_1
    }
}
impl core::ops::Deref for PCM_FLG_R {
    type Target = crate::FieldReader<bool, PCM_FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "RSTn/NMI pin was the source of NMI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PIN_FLG_A {
    #[doc = "0: Indicates the RSTn_NMI pin was not the source of NMI"]
    PIN_FLG_0 = 0,
    #[doc = "1: Indicates the RSTn_NMI pin was the source of NMI"]
    PIN_FLG_1 = 1,
}
impl From<PIN_FLG_A> for bool {
    #[inline(always)]
    fn from(variant: PIN_FLG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PIN_FLG` reader - RSTn/NMI pin was the source of NMI"]
pub struct PIN_FLG_R(crate::FieldReader<bool, PIN_FLG_A>);
impl PIN_FLG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIN_FLG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIN_FLG_A {
        match self.bits {
            false => PIN_FLG_A::PIN_FLG_0,
            true => PIN_FLG_A::PIN_FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PIN_FLG_0`"]
    #[inline(always)]
    pub fn is_pin_flg_0(&self) -> bool {
        **self == PIN_FLG_A::PIN_FLG_0
    }
    #[doc = "Checks if the value of the field is `PIN_FLG_1`"]
    #[inline(always)]
    pub fn is_pin_flg_1(&self) -> bool {
        **self == PIN_FLG_A::PIN_FLG_1
    }
}
impl core::ops::Deref for PIN_FLG_R {
    type Target = crate::FieldReader<bool, PIN_FLG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIN_FLG` writer - RSTn/NMI pin was the source of NMI"]
pub struct PIN_FLG_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN_FLG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PIN_FLG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Indicates the RSTn_NMI pin was not the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_0(self) -> &'a mut W {
        self.variant(PIN_FLG_A::PIN_FLG_0)
    }
    #[doc = "Indicates the RSTn_NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg_1(self) -> &'a mut W {
        self.variant(PIN_FLG_A::PIN_FLG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&self) -> CS_SRC_R {
        CS_SRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&self) -> PSS_SRC_R {
        PSS_SRC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&self) -> PCM_SRC_R {
        PCM_SRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&self) -> PIN_SRC_R {
        PIN_SRC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - CS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn cs_flg(&self) -> CS_FLG_R {
        CS_FLG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - PSS interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pss_flg(&self) -> PSS_FLG_R {
        PSS_FLG_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - PCM interrupt was the source of NMI"]
    #[inline(always)]
    pub fn pcm_flg(&self) -> PCM_FLG_R {
        PCM_FLG_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&self) -> PIN_FLG_R {
        PIN_FLG_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn cs_src(&mut self) -> CS_SRC_W {
        CS_SRC_W { w: self }
    }
    #[doc = "Bit 1 - PSS interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pss_src(&mut self) -> PSS_SRC_W {
        PSS_SRC_W { w: self }
    }
    #[doc = "Bit 2 - PCM interrupt as a source of NMI"]
    #[inline(always)]
    pub fn pcm_src(&mut self) -> PCM_SRC_W {
        PCM_SRC_W { w: self }
    }
    #[doc = "Bit 3 - RSTn/NMI pin configuration Note: When the device enters LPM3/LPM4 modes of operation, the functionality selected by this bit is retained. If selected as an NMI, activity on this pin in LPM3/LPM4 wakes the device and processes the interrupt, without causing a POR. If selected as a Reset, activity on this pin in LPM3/LPM4 causes a device-level POR When the device enters LPM3.5/LPM4.5 modes of operation, this bit is always cleared to 0. In other words, the RSTn/NMI pin always assumes a reset functionality in LPM3.5/LPM4.5 modes."]
    #[inline(always)]
    pub fn pin_src(&mut self) -> PIN_SRC_W {
        PIN_SRC_W { w: self }
    }
    #[doc = "Bit 19 - RSTn/NMI pin was the source of NMI"]
    #[inline(always)]
    pub fn pin_flg(&mut self) -> PIN_FLG_W {
        PIN_FLG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NMI Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_nmi_ctlstat](index.html) module"]
pub struct SYS_NMI_CTLSTAT_SPEC;
impl crate::RegisterSpec for SYS_NMI_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_nmi_ctlstat::R](R) reader structure"]
impl crate::Readable for SYS_NMI_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_nmi_ctlstat::W](W) writer structure"]
impl crate::Writable for SYS_NMI_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_NMI_CTLSTAT to value 0x07"]
impl crate::Resettable for SYS_NMI_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x07
    }
}
