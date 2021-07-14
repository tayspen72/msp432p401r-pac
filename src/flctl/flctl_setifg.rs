#[doc = "Register `FLCTL_SETIFG` writer"]
pub struct W(crate::W<FLCTL_SETIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_SETIFG_SPEC>;
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
impl From<crate::W<FLCTL_SETIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_SETIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDBRST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct RDBRST_W<'a> {
    w: &'a mut W,
}
impl<'a> RDBRST_W<'a> {
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
#[doc = "Field `AVPRE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct AVPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPRE_W<'a> {
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
#[doc = "Field `AVPST` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct AVPST_W<'a> {
    w: &'a mut W,
}
impl<'a> AVPST_W<'a> {
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
#[doc = "Field `PRG` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct PRG_W<'a> {
    w: &'a mut W,
}
impl<'a> PRG_W<'a> {
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
#[doc = "Field `PRGB` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct PRGB_W<'a> {
    w: &'a mut W,
}
impl<'a> PRGB_W<'a> {
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
#[doc = "Field `ERASE` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_W<'a> {
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
#[doc = "Field `BMRK` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> BMRK_W<'a> {
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
#[doc = "Field `PRG_ERR` writer - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
pub struct PRG_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRG_ERR_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W {
        RDBRST_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W {
        AVPRE_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W {
        AVPST_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W {
        PRG_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W {
        PRGB_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 8 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W {
        BMRK_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 clears the corresponding interrupt flag bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&mut self) -> PRG_ERR_W {
        PRG_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_setifg](index.html) module"]
pub struct FLCTL_SETIFG_SPEC;
impl crate::RegisterSpec for FLCTL_SETIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [flctl_setifg::W](W) writer structure"]
impl crate::Writable for FLCTL_SETIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_SETIFG to value 0"]
impl crate::Resettable for FLCTL_SETIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
