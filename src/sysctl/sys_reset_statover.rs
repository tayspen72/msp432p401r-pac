#[doc = "Register `SYS_RESET_STATOVER` reader"]
pub struct R(crate::R<SYS_RESET_STATOVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_RESET_STATOVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_RESET_STATOVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_RESET_STATOVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_RESET_STATOVER` writer"]
pub struct W(crate::W<SYS_RESET_STATOVER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_RESET_STATOVER_SPEC>;
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
impl From<crate::W<SYS_RESET_STATOVER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_RESET_STATOVER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT` reader - Indicates if SOFT Reset is active"]
pub struct SOFT_R(crate::FieldReader<bool, bool>);
impl SOFT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HARD` reader - Indicates if HARD Reset is active"]
pub struct HARD_R(crate::FieldReader<bool, bool>);
impl HARD_R {
    pub(crate) fn new(bits: bool) -> Self {
        HARD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HARD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REBOOT` reader - Indicates if Reboot Reset is active"]
pub struct REBOOT_R(crate::FieldReader<bool, bool>);
impl REBOOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        REBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_OVER` reader - SOFT_Reset overwrite request"]
pub struct SOFT_OVER_R(crate::FieldReader<bool, bool>);
impl SOFT_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFT_OVER` writer - SOFT_Reset overwrite request"]
pub struct SOFT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFT_OVER_W<'a> {
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
#[doc = "Field `HARD_OVER` reader - HARD_Reset overwrite request"]
pub struct HARD_OVER_R(crate::FieldReader<bool, bool>);
impl HARD_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        HARD_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HARD_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HARD_OVER` writer - HARD_Reset overwrite request"]
pub struct HARD_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> HARD_OVER_W<'a> {
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
#[doc = "Field `RBT_OVER` reader - Reboot Reset overwrite request"]
pub struct RBT_OVER_R(crate::FieldReader<bool, bool>);
impl RBT_OVER_R {
    pub(crate) fn new(bits: bool) -> Self {
        RBT_OVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RBT_OVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RBT_OVER` writer - Reboot Reset overwrite request"]
pub struct RBT_OVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RBT_OVER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Indicates if SOFT Reset is active"]
    #[inline(always)]
    pub fn soft(&self) -> SOFT_R {
        SOFT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates if HARD Reset is active"]
    #[inline(always)]
    pub fn hard(&self) -> HARD_R {
        HARD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates if Reboot Reset is active"]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&self) -> SOFT_OVER_R {
        SOFT_OVER_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&self) -> HARD_OVER_R {
        HARD_OVER_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&self) -> RBT_OVER_R {
        RBT_OVER_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - SOFT_Reset overwrite request"]
    #[inline(always)]
    pub fn soft_over(&mut self) -> SOFT_OVER_W {
        SOFT_OVER_W { w: self }
    }
    #[doc = "Bit 9 - HARD_Reset overwrite request"]
    #[inline(always)]
    pub fn hard_over(&mut self) -> HARD_OVER_W {
        HARD_OVER_W { w: self }
    }
    #[doc = "Bit 10 - Reboot Reset overwrite request"]
    #[inline(always)]
    pub fn rbt_over(&mut self) -> RBT_OVER_W {
        RBT_OVER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reset Status and Override Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reset_statover](index.html) module"]
pub struct SYS_RESET_STATOVER_SPEC;
impl crate::RegisterSpec for SYS_RESET_STATOVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reset_statover::R](R) reader structure"]
impl crate::Readable for SYS_RESET_STATOVER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reset_statover::W](W) writer structure"]
impl crate::Writable for SYS_RESET_STATOVER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_RESET_STATOVER to value 0"]
impl crate::Resettable for SYS_RESET_STATOVER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
