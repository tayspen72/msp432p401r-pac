#[doc = "Register `FLCTL_IE` reader"]
pub struct R(crate::R<FLCTL_IE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_IE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_IE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_IE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_IE` writer"]
pub struct W(crate::W<FLCTL_IE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_IE_SPEC>;
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
impl From<crate::W<FLCTL_IE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_IE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDBRST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct RDBRST_R(crate::FieldReader<bool, bool>);
impl RDBRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDBRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RDBRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDBRST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `AVPRE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct AVPRE_R(crate::FieldReader<bool, bool>);
impl AVPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVPRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVPRE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `AVPST` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct AVPST_R(crate::FieldReader<bool, bool>);
impl AVPST_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVPST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVPST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVPST` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `PRG` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct PRG_R(crate::FieldReader<bool, bool>);
impl PRG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRG` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `PRGB` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct PRGB_R(crate::FieldReader<bool, bool>);
impl PRGB_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRGB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRGB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRGB` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `ERASE` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct ERASE_R(crate::FieldReader<bool, bool>);
impl ERASE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASE` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `BMRK` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct BMRK_R(crate::FieldReader<bool, bool>);
impl BMRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        BMRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BMRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BMRK` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Field `PRG_ERR` reader - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
pub struct PRG_ERR_R(crate::FieldReader<bool, bool>);
impl PRG_ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRG_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRG_ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRG_ERR` writer - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
impl R {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&self) -> RDBRST_R {
        RDBRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&self) -> AVPRE_R {
        AVPRE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&self) -> AVPST_R {
        AVPST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&self) -> PRG_R {
        PRG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&self) -> PRGB_R {
        PRGB_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&self) -> BMRK_R {
        BMRK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg_err(&self) -> PRG_ERR_R {
        PRG_ERR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn rdbrst(&mut self) -> RDBRST_W {
        RDBRST_W { w: self }
    }
    #[doc = "Bit 1 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpre(&mut self) -> AVPRE_W {
        AVPRE_W { w: self }
    }
    #[doc = "Bit 2 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn avpst(&mut self) -> AVPST_W {
        AVPST_W { w: self }
    }
    #[doc = "Bit 3 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prg(&mut self) -> PRG_W {
        PRG_W { w: self }
    }
    #[doc = "Bit 4 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn prgb(&mut self) -> PRGB_W {
        PRGB_W { w: self }
    }
    #[doc = "Bit 5 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W { w: self }
    }
    #[doc = "Bit 8 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
    #[inline(always)]
    pub fn bmrk(&mut self) -> BMRK_W {
        BMRK_W { w: self }
    }
    #[doc = "Bit 9 - If set to 1, enables the Controller to generate an interrupt based on the corresponding bit in the FLCTL_IFG"]
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
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_ie](index.html) module"]
pub struct FLCTL_IE_SPEC;
impl crate::RegisterSpec for FLCTL_IE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_ie::R](R) reader structure"]
impl crate::Readable for FLCTL_IE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_ie::W](W) writer structure"]
impl crate::Writable for FLCTL_IE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_IE to value 0"]
impl crate::Resettable for FLCTL_IE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
