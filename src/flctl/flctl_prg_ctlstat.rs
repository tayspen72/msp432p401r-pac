#[doc = "Register `FLCTL_PRG_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_PRG_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_PRG_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_PRG_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_PRG_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_PRG_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_PRG_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_PRG_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_PRG_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_PRG_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Master control for all word program operations\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Word program operation disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Word program operation enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Master control for all word program operations"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        **self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        **self == ENABLE_A::ENABLE_1
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Master control for all word program operations"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Word program operation disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Word program operation enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
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
#[doc = "Write mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Write immediate mode. Starts program operation immediately on each write to the Flash"]
    MODE_0 = 0,
    #[doc = "1: Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Write mode"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        **self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        **self == MODE_A::MODE_1
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Write mode"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write immediate mode. Starts program operation immediately on each write to the Flash"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Full word write mode. Flash controller collates data over multiple writes to compose the full 128bit word before initiating the program operation"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
#[doc = "Controls automatic pre program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_PRE_A {
    #[doc = "0: No pre program verification"]
    VER_PRE_0 = 0,
    #[doc = "1: Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VER_PRE_1 = 1,
}
impl From<VER_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: VER_PRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER_PRE` reader - Controls automatic pre program verify operations"]
pub struct VER_PRE_R(crate::FieldReader<bool, VER_PRE_A>);
impl VER_PRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        VER_PRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_PRE_A {
        match self.bits {
            false => VER_PRE_A::VER_PRE_0,
            true => VER_PRE_A::VER_PRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `VER_PRE_0`"]
    #[inline(always)]
    pub fn is_ver_pre_0(&self) -> bool {
        **self == VER_PRE_A::VER_PRE_0
    }
    #[doc = "Checks if the value of the field is `VER_PRE_1`"]
    #[inline(always)]
    pub fn is_ver_pre_1(&self) -> bool {
        **self == VER_PRE_A::VER_PRE_1
    }
}
impl core::ops::Deref for VER_PRE_R {
    type Target = crate::FieldReader<bool, VER_PRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VER_PRE` writer - Controls automatic pre program verify operations"]
pub struct VER_PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_PRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VER_PRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No pre program verification"]
    #[inline(always)]
    pub fn ver_pre_0(self) -> &'a mut W {
        self.variant(VER_PRE_A::VER_PRE_0)
    }
    #[doc = "Pre verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pre_1(self) -> &'a mut W {
        self.variant(VER_PRE_A::VER_PRE_1)
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
#[doc = "Controls automatic post program verify operations\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VER_PST_A {
    #[doc = "0: No post program verification"]
    VER_PST_0 = 0,
    #[doc = "1: Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    VER_PST_1 = 1,
}
impl From<VER_PST_A> for bool {
    #[inline(always)]
    fn from(variant: VER_PST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VER_PST` reader - Controls automatic post program verify operations"]
pub struct VER_PST_R(crate::FieldReader<bool, VER_PST_A>);
impl VER_PST_R {
    pub(crate) fn new(bits: bool) -> Self {
        VER_PST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VER_PST_A {
        match self.bits {
            false => VER_PST_A::VER_PST_0,
            true => VER_PST_A::VER_PST_1,
        }
    }
    #[doc = "Checks if the value of the field is `VER_PST_0`"]
    #[inline(always)]
    pub fn is_ver_pst_0(&self) -> bool {
        **self == VER_PST_A::VER_PST_0
    }
    #[doc = "Checks if the value of the field is `VER_PST_1`"]
    #[inline(always)]
    pub fn is_ver_pst_1(&self) -> bool {
        **self == VER_PST_A::VER_PST_1
    }
}
impl core::ops::Deref for VER_PST_R {
    type Target = crate::FieldReader<bool, VER_PST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VER_PST` writer - Controls automatic post program verify operations"]
pub struct VER_PST_W<'a> {
    w: &'a mut W,
}
impl<'a> VER_PST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VER_PST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No post program verification"]
    #[inline(always)]
    pub fn ver_pst_0(self) -> &'a mut W {
        self.variant(VER_PST_A::VER_PST_0)
    }
    #[doc = "Post verify feature automatically invoked for each write operation (irrespective of the mode)"]
    #[inline(always)]
    pub fn ver_pst_1(self) -> &'a mut W {
        self.variant(VER_PST_A::VER_PST_1)
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
#[doc = "Status of program operations in the Flash memory\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATUS_A {
    #[doc = "0: Idle (no program operation currently active)"]
    STATUS_0 = 0,
    #[doc = "1: Single word program operation triggered, but pending"]
    STATUS_1 = 1,
    #[doc = "2: Single word program in progress"]
    STATUS_2 = 2,
}
impl From<STATUS_A> for u8 {
    #[inline(always)]
    fn from(variant: STATUS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATUS` reader - Status of program operations in the Flash memory"]
pub struct STATUS_R(crate::FieldReader<u8, STATUS_A>);
impl STATUS_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATUS_A> {
        match self.bits {
            0 => Some(STATUS_A::STATUS_0),
            1 => Some(STATUS_A::STATUS_1),
            2 => Some(STATUS_A::STATUS_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATUS_0`"]
    #[inline(always)]
    pub fn is_status_0(&self) -> bool {
        **self == STATUS_A::STATUS_0
    }
    #[doc = "Checks if the value of the field is `STATUS_1`"]
    #[inline(always)]
    pub fn is_status_1(&self) -> bool {
        **self == STATUS_A::STATUS_1
    }
    #[doc = "Checks if the value of the field is `STATUS_2`"]
    #[inline(always)]
    pub fn is_status_2(&self) -> bool {
        **self == STATUS_A::STATUS_2
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<u8, STATUS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Bank active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BNK_ACT_A {
    #[doc = "0: Word in Bank0 being programmed"]
    BNK_ACT_0 = 0,
    #[doc = "1: Word in Bank1 being programmed"]
    BNK_ACT_1 = 1,
}
impl From<BNK_ACT_A> for bool {
    #[inline(always)]
    fn from(variant: BNK_ACT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BNK_ACT` reader - Bank active"]
pub struct BNK_ACT_R(crate::FieldReader<bool, BNK_ACT_A>);
impl BNK_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        BNK_ACT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BNK_ACT_A {
        match self.bits {
            false => BNK_ACT_A::BNK_ACT_0,
            true => BNK_ACT_A::BNK_ACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BNK_ACT_0`"]
    #[inline(always)]
    pub fn is_bnk_act_0(&self) -> bool {
        **self == BNK_ACT_A::BNK_ACT_0
    }
    #[doc = "Checks if the value of the field is `BNK_ACT_1`"]
    #[inline(always)]
    pub fn is_bnk_act_1(&self) -> bool {
        **self == BNK_ACT_A::BNK_ACT_1
    }
}
impl core::ops::Deref for BNK_ACT_R {
    type Target = crate::FieldReader<bool, BNK_ACT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&self) -> VER_PRE_R {
        VER_PRE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&self) -> VER_PST_R {
        VER_PST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Status of program operations in the Flash memory"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Bank active"]
    #[inline(always)]
    pub fn bnk_act(&self) -> BNK_ACT_R {
        BNK_ACT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master control for all word program operations"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - Write mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 2 - Controls automatic pre program verify operations"]
    #[inline(always)]
    pub fn ver_pre(&mut self) -> VER_PRE_W {
        VER_PRE_W { w: self }
    }
    #[doc = "Bit 3 - Controls automatic post program verify operations"]
    #[inline(always)]
    pub fn ver_pst(&mut self) -> VER_PST_W {
        VER_PST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prg_ctlstat](index.html) module"]
pub struct FLCTL_PRG_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_PRG_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_prg_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_PRG_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_prg_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_PRG_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_PRG_CTLSTAT to value 0x0c"]
impl crate::Resettable for FLCTL_PRG_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0c
    }
}
