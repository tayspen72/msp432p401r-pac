#[doc = "Register `SYS_REBOOT_CTL` reader"]
pub struct R(crate::R<SYS_REBOOT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_REBOOT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_REBOOT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_REBOOT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_REBOOT_CTL` writer"]
pub struct W(crate::W<SYS_REBOOT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_REBOOT_CTL_SPEC>;
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
impl From<crate::W<SYS_REBOOT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_REBOOT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REBOOT` reader - Write 1 initiates a Reboot of the device"]
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
#[doc = "Field `REBOOT` writer - Write 1 initiates a Reboot of the device"]
pub struct REBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> REBOOT_W<'a> {
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
#[doc = "Field `WKEY` writer - Key to enable writes to bit 0"]
pub struct WKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> WKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&self) -> REBOOT_R {
        REBOOT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 initiates a Reboot of the device"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W {
        REBOOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Key to enable writes to bit 0"]
    #[inline(always)]
    pub fn wkey(&mut self) -> WKEY_W {
        WKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reboot Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reboot_ctl](index.html) module"]
pub struct SYS_REBOOT_CTL_SPEC;
impl crate::RegisterSpec for SYS_REBOOT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reboot_ctl::R](R) reader structure"]
impl crate::Readable for SYS_REBOOT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reboot_ctl::W](W) writer structure"]
impl crate::Writable for SYS_REBOOT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_REBOOT_CTL to value 0xfe"]
impl crate::Resettable for SYS_REBOOT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xfe
    }
}
