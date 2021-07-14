#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT` reader"]
pub struct R(crate::R<FLCTL_BANK1_MAIN_WEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BANK1_MAIN_WEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BANK1_MAIN_WEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BANK1_MAIN_WEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BANK1_MAIN_WEPROT` writer"]
pub struct W(crate::W<FLCTL_BANK1_MAIN_WEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BANK1_MAIN_WEPROT_SPEC>;
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
impl From<crate::W<FLCTL_BANK1_MAIN_WEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BANK1_MAIN_WEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT0` reader - Protects Sector 0 from program or erase operations"]
pub struct PROT0_R(crate::FieldReader<bool, bool>);
impl PROT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT0` writer - Protects Sector 0 from program or erase operations"]
pub struct PROT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT0_W<'a> {
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
#[doc = "Field `PROT1` reader - Protects Sector 1 from program or erase operations"]
pub struct PROT1_R(crate::FieldReader<bool, bool>);
impl PROT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT1` writer - Protects Sector 1 from program or erase operations"]
pub struct PROT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT1_W<'a> {
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
#[doc = "Field `PROT2` reader - Protects Sector 2 from program or erase operations"]
pub struct PROT2_R(crate::FieldReader<bool, bool>);
impl PROT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT2` writer - Protects Sector 2 from program or erase operations"]
pub struct PROT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT2_W<'a> {
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
#[doc = "Field `PROT3` reader - Protects Sector 3 from program or erase operations"]
pub struct PROT3_R(crate::FieldReader<bool, bool>);
impl PROT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT3` writer - Protects Sector 3 from program or erase operations"]
pub struct PROT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT3_W<'a> {
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
#[doc = "Field `PROT4` reader - Protects Sector 4 from program or erase operations"]
pub struct PROT4_R(crate::FieldReader<bool, bool>);
impl PROT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT4` writer - Protects Sector 4 from program or erase operations"]
pub struct PROT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT4_W<'a> {
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
#[doc = "Field `PROT5` reader - Protects Sector 5 from program or erase operations"]
pub struct PROT5_R(crate::FieldReader<bool, bool>);
impl PROT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT5` writer - Protects Sector 5 from program or erase operations"]
pub struct PROT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT5_W<'a> {
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
#[doc = "Field `PROT6` reader - Protects Sector 6 from program or erase operations"]
pub struct PROT6_R(crate::FieldReader<bool, bool>);
impl PROT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT6` writer - Protects Sector 6 from program or erase operations"]
pub struct PROT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT6_W<'a> {
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
#[doc = "Field `PROT7` reader - Protects Sector 7 from program or erase operations"]
pub struct PROT7_R(crate::FieldReader<bool, bool>);
impl PROT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT7` writer - Protects Sector 7 from program or erase operations"]
pub struct PROT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT7_W<'a> {
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
#[doc = "Field `PROT8` reader - Protects Sector 8 from program or erase operations"]
pub struct PROT8_R(crate::FieldReader<bool, bool>);
impl PROT8_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT8` writer - Protects Sector 8 from program or erase operations"]
pub struct PROT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT8_W<'a> {
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
#[doc = "Field `PROT9` reader - Protects Sector 9 from program or erase operations"]
pub struct PROT9_R(crate::FieldReader<bool, bool>);
impl PROT9_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT9` writer - Protects Sector 9 from program or erase operations"]
pub struct PROT9_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT9_W<'a> {
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
#[doc = "Field `PROT10` reader - Protects Sector 10 from program or erase operations"]
pub struct PROT10_R(crate::FieldReader<bool, bool>);
impl PROT10_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT10` writer - Protects Sector 10 from program or erase operations"]
pub struct PROT10_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT10_W<'a> {
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
#[doc = "Field `PROT11` reader - Protects Sector 11 from program or erase operations"]
pub struct PROT11_R(crate::FieldReader<bool, bool>);
impl PROT11_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT11` writer - Protects Sector 11 from program or erase operations"]
pub struct PROT11_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT11_W<'a> {
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
#[doc = "Field `PROT12` reader - Protects Sector 12 from program or erase operations"]
pub struct PROT12_R(crate::FieldReader<bool, bool>);
impl PROT12_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT12` writer - Protects Sector 12 from program or erase operations"]
pub struct PROT12_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT12_W<'a> {
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
#[doc = "Field `PROT13` reader - Protects Sector 13 from program or erase operations"]
pub struct PROT13_R(crate::FieldReader<bool, bool>);
impl PROT13_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT13` writer - Protects Sector 13 from program or erase operations"]
pub struct PROT13_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT13_W<'a> {
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
#[doc = "Field `PROT14` reader - Protects Sector 14 from program or erase operations"]
pub struct PROT14_R(crate::FieldReader<bool, bool>);
impl PROT14_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT14` writer - Protects Sector 14 from program or erase operations"]
pub struct PROT14_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT14_W<'a> {
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
#[doc = "Field `PROT15` reader - Protects Sector 15 from program or erase operations"]
pub struct PROT15_R(crate::FieldReader<bool, bool>);
impl PROT15_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT15` writer - Protects Sector 15 from program or erase operations"]
pub struct PROT15_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT15_W<'a> {
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
#[doc = "Field `PROT16` reader - Protects Sector 16 from program or erase operations"]
pub struct PROT16_R(crate::FieldReader<bool, bool>);
impl PROT16_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT16` writer - Protects Sector 16 from program or erase operations"]
pub struct PROT16_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT16_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `PROT17` reader - Protects Sector 17 from program or erase operations"]
pub struct PROT17_R(crate::FieldReader<bool, bool>);
impl PROT17_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT17` writer - Protects Sector 17 from program or erase operations"]
pub struct PROT17_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT17_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `PROT18` reader - Protects Sector 18 from program or erase operations"]
pub struct PROT18_R(crate::FieldReader<bool, bool>);
impl PROT18_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT18` writer - Protects Sector 18 from program or erase operations"]
pub struct PROT18_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT18_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PROT19` reader - Protects Sector 19 from program or erase operations"]
pub struct PROT19_R(crate::FieldReader<bool, bool>);
impl PROT19_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT19` writer - Protects Sector 19 from program or erase operations"]
pub struct PROT19_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT19_W<'a> {
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
#[doc = "Field `PROT20` reader - Protects Sector 20 from program or erase operations"]
pub struct PROT20_R(crate::FieldReader<bool, bool>);
impl PROT20_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT20` writer - Protects Sector 20 from program or erase operations"]
pub struct PROT20_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT20_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `PROT21` reader - Protects Sector 21 from program or erase operations"]
pub struct PROT21_R(crate::FieldReader<bool, bool>);
impl PROT21_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT21` writer - Protects Sector 21 from program or erase operations"]
pub struct PROT21_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT21_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `PROT22` reader - Protects Sector 22 from program or erase operations"]
pub struct PROT22_R(crate::FieldReader<bool, bool>);
impl PROT22_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT22` writer - Protects Sector 22 from program or erase operations"]
pub struct PROT22_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT22_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `PROT23` reader - Protects Sector 23 from program or erase operations"]
pub struct PROT23_R(crate::FieldReader<bool, bool>);
impl PROT23_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT23` writer - Protects Sector 23 from program or erase operations"]
pub struct PROT23_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT23_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `PROT24` reader - Protects Sector 24 from program or erase operations"]
pub struct PROT24_R(crate::FieldReader<bool, bool>);
impl PROT24_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT24` writer - Protects Sector 24 from program or erase operations"]
pub struct PROT24_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT24_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `PROT25` reader - Protects Sector 25 from program or erase operations"]
pub struct PROT25_R(crate::FieldReader<bool, bool>);
impl PROT25_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT25` writer - Protects Sector 25 from program or erase operations"]
pub struct PROT25_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT25_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `PROT26` reader - Protects Sector 26 from program or erase operations"]
pub struct PROT26_R(crate::FieldReader<bool, bool>);
impl PROT26_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT26` writer - Protects Sector 26 from program or erase operations"]
pub struct PROT26_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT26_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `PROT27` reader - Protects Sector 27 from program or erase operations"]
pub struct PROT27_R(crate::FieldReader<bool, bool>);
impl PROT27_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT27` writer - Protects Sector 27 from program or erase operations"]
pub struct PROT27_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT27_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `PROT28` reader - Protects Sector 28 from program or erase operations"]
pub struct PROT28_R(crate::FieldReader<bool, bool>);
impl PROT28_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT28` writer - Protects Sector 28 from program or erase operations"]
pub struct PROT28_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT28_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `PROT29` reader - Protects Sector 29 from program or erase operations"]
pub struct PROT29_R(crate::FieldReader<bool, bool>);
impl PROT29_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT29` writer - Protects Sector 29 from program or erase operations"]
pub struct PROT29_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT29_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `PROT30` reader - Protects Sector 30 from program or erase operations"]
pub struct PROT30_R(crate::FieldReader<bool, bool>);
impl PROT30_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT30` writer - Protects Sector 30 from program or erase operations"]
pub struct PROT30_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT30_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `PROT31` reader - Protects Sector 31 from program or erase operations"]
pub struct PROT31_R(crate::FieldReader<bool, bool>);
impl PROT31_R {
    pub(crate) fn new(bits: bool) -> Self {
        PROT31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PROT31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PROT31` writer - Protects Sector 31 from program or erase operations"]
pub struct PROT31_W<'a> {
    w: &'a mut W,
}
impl<'a> PROT31_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&self) -> PROT0_R {
        PROT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&self) -> PROT1_R {
        PROT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase operations"]
    #[inline(always)]
    pub fn prot2(&self) -> PROT2_R {
        PROT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase operations"]
    #[inline(always)]
    pub fn prot3(&self) -> PROT3_R {
        PROT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase operations"]
    #[inline(always)]
    pub fn prot4(&self) -> PROT4_R {
        PROT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase operations"]
    #[inline(always)]
    pub fn prot5(&self) -> PROT5_R {
        PROT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase operations"]
    #[inline(always)]
    pub fn prot6(&self) -> PROT6_R {
        PROT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase operations"]
    #[inline(always)]
    pub fn prot7(&self) -> PROT7_R {
        PROT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase operations"]
    #[inline(always)]
    pub fn prot8(&self) -> PROT8_R {
        PROT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase operations"]
    #[inline(always)]
    pub fn prot9(&self) -> PROT9_R {
        PROT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase operations"]
    #[inline(always)]
    pub fn prot10(&self) -> PROT10_R {
        PROT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase operations"]
    #[inline(always)]
    pub fn prot11(&self) -> PROT11_R {
        PROT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase operations"]
    #[inline(always)]
    pub fn prot12(&self) -> PROT12_R {
        PROT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase operations"]
    #[inline(always)]
    pub fn prot13(&self) -> PROT13_R {
        PROT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase operations"]
    #[inline(always)]
    pub fn prot14(&self) -> PROT14_R {
        PROT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase operations"]
    #[inline(always)]
    pub fn prot15(&self) -> PROT15_R {
        PROT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase operations"]
    #[inline(always)]
    pub fn prot16(&self) -> PROT16_R {
        PROT16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase operations"]
    #[inline(always)]
    pub fn prot17(&self) -> PROT17_R {
        PROT17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase operations"]
    #[inline(always)]
    pub fn prot18(&self) -> PROT18_R {
        PROT18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase operations"]
    #[inline(always)]
    pub fn prot19(&self) -> PROT19_R {
        PROT19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase operations"]
    #[inline(always)]
    pub fn prot20(&self) -> PROT20_R {
        PROT20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase operations"]
    #[inline(always)]
    pub fn prot21(&self) -> PROT21_R {
        PROT21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase operations"]
    #[inline(always)]
    pub fn prot22(&self) -> PROT22_R {
        PROT22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase operations"]
    #[inline(always)]
    pub fn prot23(&self) -> PROT23_R {
        PROT23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase operations"]
    #[inline(always)]
    pub fn prot24(&self) -> PROT24_R {
        PROT24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase operations"]
    #[inline(always)]
    pub fn prot25(&self) -> PROT25_R {
        PROT25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase operations"]
    #[inline(always)]
    pub fn prot26(&self) -> PROT26_R {
        PROT26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase operations"]
    #[inline(always)]
    pub fn prot27(&self) -> PROT27_R {
        PROT27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase operations"]
    #[inline(always)]
    pub fn prot28(&self) -> PROT28_R {
        PROT28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase operations"]
    #[inline(always)]
    pub fn prot29(&self) -> PROT29_R {
        PROT29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase operations"]
    #[inline(always)]
    pub fn prot30(&self) -> PROT30_R {
        PROT30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase operations"]
    #[inline(always)]
    pub fn prot31(&self) -> PROT31_R {
        PROT31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protects Sector 0 from program or erase operations"]
    #[inline(always)]
    pub fn prot0(&mut self) -> PROT0_W {
        PROT0_W { w: self }
    }
    #[doc = "Bit 1 - Protects Sector 1 from program or erase operations"]
    #[inline(always)]
    pub fn prot1(&mut self) -> PROT1_W {
        PROT1_W { w: self }
    }
    #[doc = "Bit 2 - Protects Sector 2 from program or erase operations"]
    #[inline(always)]
    pub fn prot2(&mut self) -> PROT2_W {
        PROT2_W { w: self }
    }
    #[doc = "Bit 3 - Protects Sector 3 from program or erase operations"]
    #[inline(always)]
    pub fn prot3(&mut self) -> PROT3_W {
        PROT3_W { w: self }
    }
    #[doc = "Bit 4 - Protects Sector 4 from program or erase operations"]
    #[inline(always)]
    pub fn prot4(&mut self) -> PROT4_W {
        PROT4_W { w: self }
    }
    #[doc = "Bit 5 - Protects Sector 5 from program or erase operations"]
    #[inline(always)]
    pub fn prot5(&mut self) -> PROT5_W {
        PROT5_W { w: self }
    }
    #[doc = "Bit 6 - Protects Sector 6 from program or erase operations"]
    #[inline(always)]
    pub fn prot6(&mut self) -> PROT6_W {
        PROT6_W { w: self }
    }
    #[doc = "Bit 7 - Protects Sector 7 from program or erase operations"]
    #[inline(always)]
    pub fn prot7(&mut self) -> PROT7_W {
        PROT7_W { w: self }
    }
    #[doc = "Bit 8 - Protects Sector 8 from program or erase operations"]
    #[inline(always)]
    pub fn prot8(&mut self) -> PROT8_W {
        PROT8_W { w: self }
    }
    #[doc = "Bit 9 - Protects Sector 9 from program or erase operations"]
    #[inline(always)]
    pub fn prot9(&mut self) -> PROT9_W {
        PROT9_W { w: self }
    }
    #[doc = "Bit 10 - Protects Sector 10 from program or erase operations"]
    #[inline(always)]
    pub fn prot10(&mut self) -> PROT10_W {
        PROT10_W { w: self }
    }
    #[doc = "Bit 11 - Protects Sector 11 from program or erase operations"]
    #[inline(always)]
    pub fn prot11(&mut self) -> PROT11_W {
        PROT11_W { w: self }
    }
    #[doc = "Bit 12 - Protects Sector 12 from program or erase operations"]
    #[inline(always)]
    pub fn prot12(&mut self) -> PROT12_W {
        PROT12_W { w: self }
    }
    #[doc = "Bit 13 - Protects Sector 13 from program or erase operations"]
    #[inline(always)]
    pub fn prot13(&mut self) -> PROT13_W {
        PROT13_W { w: self }
    }
    #[doc = "Bit 14 - Protects Sector 14 from program or erase operations"]
    #[inline(always)]
    pub fn prot14(&mut self) -> PROT14_W {
        PROT14_W { w: self }
    }
    #[doc = "Bit 15 - Protects Sector 15 from program or erase operations"]
    #[inline(always)]
    pub fn prot15(&mut self) -> PROT15_W {
        PROT15_W { w: self }
    }
    #[doc = "Bit 16 - Protects Sector 16 from program or erase operations"]
    #[inline(always)]
    pub fn prot16(&mut self) -> PROT16_W {
        PROT16_W { w: self }
    }
    #[doc = "Bit 17 - Protects Sector 17 from program or erase operations"]
    #[inline(always)]
    pub fn prot17(&mut self) -> PROT17_W {
        PROT17_W { w: self }
    }
    #[doc = "Bit 18 - Protects Sector 18 from program or erase operations"]
    #[inline(always)]
    pub fn prot18(&mut self) -> PROT18_W {
        PROT18_W { w: self }
    }
    #[doc = "Bit 19 - Protects Sector 19 from program or erase operations"]
    #[inline(always)]
    pub fn prot19(&mut self) -> PROT19_W {
        PROT19_W { w: self }
    }
    #[doc = "Bit 20 - Protects Sector 20 from program or erase operations"]
    #[inline(always)]
    pub fn prot20(&mut self) -> PROT20_W {
        PROT20_W { w: self }
    }
    #[doc = "Bit 21 - Protects Sector 21 from program or erase operations"]
    #[inline(always)]
    pub fn prot21(&mut self) -> PROT21_W {
        PROT21_W { w: self }
    }
    #[doc = "Bit 22 - Protects Sector 22 from program or erase operations"]
    #[inline(always)]
    pub fn prot22(&mut self) -> PROT22_W {
        PROT22_W { w: self }
    }
    #[doc = "Bit 23 - Protects Sector 23 from program or erase operations"]
    #[inline(always)]
    pub fn prot23(&mut self) -> PROT23_W {
        PROT23_W { w: self }
    }
    #[doc = "Bit 24 - Protects Sector 24 from program or erase operations"]
    #[inline(always)]
    pub fn prot24(&mut self) -> PROT24_W {
        PROT24_W { w: self }
    }
    #[doc = "Bit 25 - Protects Sector 25 from program or erase operations"]
    #[inline(always)]
    pub fn prot25(&mut self) -> PROT25_W {
        PROT25_W { w: self }
    }
    #[doc = "Bit 26 - Protects Sector 26 from program or erase operations"]
    #[inline(always)]
    pub fn prot26(&mut self) -> PROT26_W {
        PROT26_W { w: self }
    }
    #[doc = "Bit 27 - Protects Sector 27 from program or erase operations"]
    #[inline(always)]
    pub fn prot27(&mut self) -> PROT27_W {
        PROT27_W { w: self }
    }
    #[doc = "Bit 28 - Protects Sector 28 from program or erase operations"]
    #[inline(always)]
    pub fn prot28(&mut self) -> PROT28_W {
        PROT28_W { w: self }
    }
    #[doc = "Bit 29 - Protects Sector 29 from program or erase operations"]
    #[inline(always)]
    pub fn prot29(&mut self) -> PROT29_W {
        PROT29_W { w: self }
    }
    #[doc = "Bit 30 - Protects Sector 30 from program or erase operations"]
    #[inline(always)]
    pub fn prot30(&mut self) -> PROT30_W {
        PROT30_W { w: self }
    }
    #[doc = "Bit 31 - Protects Sector 31 from program or erase operations"]
    #[inline(always)]
    pub fn prot31(&mut self) -> PROT31_W {
        PROT31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Memory Bank1 Write/Erase Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bank1_main_weprot](index.html) module"]
pub struct FLCTL_BANK1_MAIN_WEPROT_SPEC;
impl crate::RegisterSpec for FLCTL_BANK1_MAIN_WEPROT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bank1_main_weprot::R](R) reader structure"]
impl crate::Readable for FLCTL_BANK1_MAIN_WEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bank1_main_weprot::W](W) writer structure"]
impl crate::Writable for FLCTL_BANK1_MAIN_WEPROT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BANK1_MAIN_WEPROT to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_BANK1_MAIN_WEPROT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
