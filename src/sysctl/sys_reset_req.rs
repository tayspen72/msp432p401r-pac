#[doc = "Register `SYS_RESET_REQ` reader"]
pub struct R(crate::R<SYS_RESET_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_RESET_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_RESET_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_RESET_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_RESET_REQ` writer"]
pub struct W(crate::W<SYS_RESET_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_RESET_REQ_SPEC>;
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
impl From<crate::W<SYS_RESET_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_RESET_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POR` writer - Generate POR"]
pub struct POR_W<'a> {
    w: &'a mut W,
}
impl<'a> POR_W<'a> {
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
#[doc = "Field `REBOOT` writer - Generate Reboot_Reset"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `WKEY` writer - Write key"]
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
impl W {
    #[doc = "Bit 0 - Generate POR"]
    #[inline(always)]
    pub fn por(&mut self) -> POR_W {
        POR_W { w: self }
    }
    #[doc = "Bit 1 - Generate Reboot_Reset"]
    #[inline(always)]
    pub fn reboot(&mut self) -> REBOOT_W {
        REBOOT_W { w: self }
    }
    #[doc = "Bits 8:15 - Write key"]
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
#[doc = "Reset Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_reset_req](index.html) module"]
pub struct SYS_RESET_REQ_SPEC;
impl crate::RegisterSpec for SYS_RESET_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_reset_req::R](R) reader structure"]
impl crate::Readable for SYS_RESET_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_reset_req::W](W) writer structure"]
impl crate::Writable for SYS_RESET_REQ_SPEC {
    type Writer = W;
}
