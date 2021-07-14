#[doc = "Register `RSTCTL_HARDRESET_SET` reader"]
pub struct R(crate::R<RSTCTL_HARDRESET_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_HARDRESET_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_HARDRESET_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_HARDRESET_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCTL_HARDRESET_SET` writer"]
pub struct W(crate::W<RSTCTL_HARDRESET_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCTL_HARDRESET_SET_SPEC>;
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
impl From<crate::W<RSTCTL_HARDRESET_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCTL_HARDRESET_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SRC0` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC0_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC0_W<'a> {
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
#[doc = "Field `SRC1` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC1_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC1_W<'a> {
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
#[doc = "Field `SRC2` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC2_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC2_W<'a> {
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
#[doc = "Field `SRC3` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC3_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC3_W<'a> {
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
#[doc = "Field `SRC4` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC4_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC4_W<'a> {
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
#[doc = "Field `SRC5` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC5_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC5_W<'a> {
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
#[doc = "Field `SRC6` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC6_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC6_W<'a> {
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
#[doc = "Field `SRC7` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC7_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC7_W<'a> {
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
#[doc = "Field `SRC8` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC8_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC8_W<'a> {
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
#[doc = "Field `SRC9` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC9_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC9_W<'a> {
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
#[doc = "Field `SRC10` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC10_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC10_W<'a> {
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
#[doc = "Field `SRC11` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC11_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC11_W<'a> {
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
#[doc = "Field `SRC12` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC12_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC12_W<'a> {
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
#[doc = "Field `SRC13` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC13_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC13_W<'a> {
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
#[doc = "Field `SRC14` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC14_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC14_W<'a> {
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
#[doc = "Field `SRC15` writer - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
pub struct SRC15_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC15_W<'a> {
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
impl W {
    #[doc = "Bit 0 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src0(&mut self) -> SRC0_W {
        SRC0_W { w: self }
    }
    #[doc = "Bit 1 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src1(&mut self) -> SRC1_W {
        SRC1_W { w: self }
    }
    #[doc = "Bit 2 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src2(&mut self) -> SRC2_W {
        SRC2_W { w: self }
    }
    #[doc = "Bit 3 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src3(&mut self) -> SRC3_W {
        SRC3_W { w: self }
    }
    #[doc = "Bit 4 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src4(&mut self) -> SRC4_W {
        SRC4_W { w: self }
    }
    #[doc = "Bit 5 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src5(&mut self) -> SRC5_W {
        SRC5_W { w: self }
    }
    #[doc = "Bit 6 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src6(&mut self) -> SRC6_W {
        SRC6_W { w: self }
    }
    #[doc = "Bit 7 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src7(&mut self) -> SRC7_W {
        SRC7_W { w: self }
    }
    #[doc = "Bit 8 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src8(&mut self) -> SRC8_W {
        SRC8_W { w: self }
    }
    #[doc = "Bit 9 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src9(&mut self) -> SRC9_W {
        SRC9_W { w: self }
    }
    #[doc = "Bit 10 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src10(&mut self) -> SRC10_W {
        SRC10_W { w: self }
    }
    #[doc = "Bit 11 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src11(&mut self) -> SRC11_W {
        SRC11_W { w: self }
    }
    #[doc = "Bit 12 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src12(&mut self) -> SRC12_W {
        SRC12_W { w: self }
    }
    #[doc = "Bit 13 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src13(&mut self) -> SRC13_W {
        SRC13_W { w: self }
    }
    #[doc = "Bit 14 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src14(&mut self) -> SRC14_W {
        SRC14_W { w: self }
    }
    #[doc = "Bit 15 - Write 1 sets the corresponding bit in the RSTCTL_HARDRESET_STAT (and initiates a Hard Reset)"]
    #[inline(always)]
    pub fn src15(&mut self) -> SRC15_W {
        SRC15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Hard Reset Status Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_hardreset_set](index.html) module"]
pub struct RSTCTL_HARDRESET_SET_SPEC;
impl crate::RegisterSpec for RSTCTL_HARDRESET_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_hardreset_set::R](R) reader structure"]
impl crate::Readable for RSTCTL_HARDRESET_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstctl_hardreset_set::W](W) writer structure"]
impl crate::Writable for RSTCTL_HARDRESET_SET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RSTCTL_HARDRESET_SET to value 0"]
impl crate::Resettable for RSTCTL_HARDRESET_SET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
