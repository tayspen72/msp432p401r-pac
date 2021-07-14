#[doc = "Register `PCMCTL1` reader"]
pub struct R(crate::R<PCMCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMCTL1` writer"]
pub struct W(crate::W<PCMCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMCTL1_SPEC>;
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
impl From<crate::W<PCMCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Lock LPM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKLPM5_A {
    #[doc = "0: LPMx.5 configuration defaults to reset condition"]
    LOCKLPM5_0 = 0,
    #[doc = "1: LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    LOCKLPM5_1 = 1,
}
impl From<LOCKLPM5_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKLPM5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKLPM5` reader - Lock LPM5"]
pub struct LOCKLPM5_R(crate::FieldReader<bool, LOCKLPM5_A>);
impl LOCKLPM5_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKLPM5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKLPM5_A {
        match self.bits {
            false => LOCKLPM5_A::LOCKLPM5_0,
            true => LOCKLPM5_A::LOCKLPM5_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_0`"]
    #[inline(always)]
    pub fn is_locklpm5_0(&self) -> bool {
        **self == LOCKLPM5_A::LOCKLPM5_0
    }
    #[doc = "Checks if the value of the field is `LOCKLPM5_1`"]
    #[inline(always)]
    pub fn is_locklpm5_1(&self) -> bool {
        **self == LOCKLPM5_A::LOCKLPM5_1
    }
}
impl core::ops::Deref for LOCKLPM5_R {
    type Target = crate::FieldReader<bool, LOCKLPM5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKLPM5` writer - Lock LPM5"]
pub struct LOCKLPM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKLPM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKLPM5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "LPMx.5 configuration defaults to reset condition"]
    #[inline(always)]
    pub fn locklpm5_0(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_0)
    }
    #[doc = "LPMx.5 configuration remains locked during LPMx.5 entry and exit"]
    #[inline(always)]
    pub fn locklpm5_1(self) -> &'a mut W {
        self.variant(LOCKLPM5_A::LOCKLPM5_1)
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
#[doc = "Lock Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOCKBKUP_A {
    #[doc = "0: Backup domain configuration defaults to reset condition"]
    LOCKBKUP_0 = 0,
    #[doc = "1: Backup domain configuration remains locked during LPM3.5 entry and exit"]
    LOCKBKUP_1 = 1,
}
impl From<LOCKBKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKBKUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOCKBKUP` reader - Lock Backup"]
pub struct LOCKBKUP_R(crate::FieldReader<bool, LOCKBKUP_A>);
impl LOCKBKUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKBKUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKBKUP_A {
        match self.bits {
            false => LOCKBKUP_A::LOCKBKUP_0,
            true => LOCKBKUP_A::LOCKBKUP_1,
        }
    }
    #[doc = "Checks if the value of the field is `LOCKBKUP_0`"]
    #[inline(always)]
    pub fn is_lockbkup_0(&self) -> bool {
        **self == LOCKBKUP_A::LOCKBKUP_0
    }
    #[doc = "Checks if the value of the field is `LOCKBKUP_1`"]
    #[inline(always)]
    pub fn is_lockbkup_1(&self) -> bool {
        **self == LOCKBKUP_A::LOCKBKUP_1
    }
}
impl core::ops::Deref for LOCKBKUP_R {
    type Target = crate::FieldReader<bool, LOCKBKUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKBKUP` writer - Lock Backup"]
pub struct LOCKBKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKBKUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOCKBKUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Backup domain configuration defaults to reset condition"]
    #[inline(always)]
    pub fn lockbkup_0(self) -> &'a mut W {
        self.variant(LOCKBKUP_A::LOCKBKUP_0)
    }
    #[doc = "Backup domain configuration remains locked during LPM3.5 entry and exit"]
    #[inline(always)]
    pub fn lockbkup_1(self) -> &'a mut W {
        self.variant(LOCKBKUP_A::LOCKBKUP_1)
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
#[doc = "Force LPM entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FORCE_LPM_ENTRY_A {
    #[doc = "0: PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    FORCE_LPM_ENTRY_0 = 0,
    #[doc = "1: PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    FORCE_LPM_ENTRY_1 = 1,
}
impl From<FORCE_LPM_ENTRY_A> for bool {
    #[inline(always)]
    fn from(variant: FORCE_LPM_ENTRY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FORCE_LPM_ENTRY` reader - Force LPM entry"]
pub struct FORCE_LPM_ENTRY_R(crate::FieldReader<bool, FORCE_LPM_ENTRY_A>);
impl FORCE_LPM_ENTRY_R {
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_LPM_ENTRY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FORCE_LPM_ENTRY_A {
        match self.bits {
            false => FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0,
            true => FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1,
        }
    }
    #[doc = "Checks if the value of the field is `FORCE_LPM_ENTRY_0`"]
    #[inline(always)]
    pub fn is_force_lpm_entry_0(&self) -> bool {
        **self == FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0
    }
    #[doc = "Checks if the value of the field is `FORCE_LPM_ENTRY_1`"]
    #[inline(always)]
    pub fn is_force_lpm_entry_1(&self) -> bool {
        **self == FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1
    }
}
impl core::ops::Deref for FORCE_LPM_ENTRY_R {
    type Target = crate::FieldReader<bool, FORCE_LPM_ENTRY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FORCE_LPM_ENTRY` writer - Force LPM entry"]
pub struct FORCE_LPM_ENTRY_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_LPM_ENTRY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FORCE_LPM_ENTRY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PCM aborts LPM3/LPMx.5 transition if the active clock configuration does not meet the LPM3/LPMx.5 entry criteria. PCM generates the LPM_INVALID_CLK flag on abort to LPM3/LPMx.5 entry."]
    #[inline(always)]
    pub fn force_lpm_entry_0(self) -> &'a mut W {
        self.variant(FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_0)
    }
    #[doc = "PCM enters LPM3/LPMx.5 after shuting off the clocks forcefully. Application needs to ensure RTC and WDT are clocked using BCLK tree to keep these modules alive in LPM3/LPM3.5. In LPM4.5 all clocks are forcefully shutoff and the core voltage is turned off."]
    #[inline(always)]
    pub fn force_lpm_entry_1(self) -> &'a mut W {
        self.variant(FORCE_LPM_ENTRY_A::FORCE_LPM_ENTRY_1)
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
#[doc = "Field `PMR_BUSY` reader - Power mode request busy flag"]
pub struct PMR_BUSY_R(crate::FieldReader<bool, bool>);
impl PMR_BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMR_BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMR_BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMR_BUSY` writer - Power mode request busy flag"]
pub struct PMR_BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> PMR_BUSY_W<'a> {
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
#[doc = "Field `PCMKEY` reader - PCM key"]
pub struct PCMKEY_R(crate::FieldReader<u16, u16>);
impl PCMKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCMKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMKEY` writer - PCM key"]
pub struct PCMKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&self) -> LOCKLPM5_R {
        LOCKLPM5_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&self) -> LOCKBKUP_R {
        LOCKBKUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&self) -> FORCE_LPM_ENTRY_R {
        FORCE_LPM_ENTRY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&self) -> PMR_BUSY_R {
        PMR_BUSY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PCMKEY_R {
        PCMKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LPM5"]
    #[inline(always)]
    pub fn locklpm5(&mut self) -> LOCKLPM5_W {
        LOCKLPM5_W { w: self }
    }
    #[doc = "Bit 1 - Lock Backup"]
    #[inline(always)]
    pub fn lockbkup(&mut self) -> LOCKBKUP_W {
        LOCKBKUP_W { w: self }
    }
    #[doc = "Bit 2 - Force LPM entry"]
    #[inline(always)]
    pub fn force_lpm_entry(&mut self) -> FORCE_LPM_ENTRY_W {
        FORCE_LPM_ENTRY_W { w: self }
    }
    #[doc = "Bit 8 - Power mode request busy flag"]
    #[inline(always)]
    pub fn pmr_busy(&mut self) -> PMR_BUSY_W {
        PMR_BUSY_W { w: self }
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PCMKEY_W {
        PCMKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmctl1](index.html) module"]
pub struct PCMCTL1_SPEC;
impl crate::RegisterSpec for PCMCTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmctl1::R](R) reader structure"]
impl crate::Readable for PCMCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmctl1::W](W) writer structure"]
impl crate::Writable for PCMCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCMCTL1 to value 0xa596_0000"]
impl crate::Resettable for PCMCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa596_0000
    }
}
