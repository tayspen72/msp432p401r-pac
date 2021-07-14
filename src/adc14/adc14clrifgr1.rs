#[doc = "Register `ADC14CLRIFGR1` reader"]
pub struct R(crate::R<ADC14CLRIFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14CLRIFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14CLRIFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14CLRIFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14CLRIFGR1` writer"]
pub struct W(crate::W<ADC14CLRIFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14CLRIFGR1_SPEC>;
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
impl From<crate::W<ADC14CLRIFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14CLRIFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "clear ADC14INIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14INIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14INIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14INIFG_1 = 1,
}
impl From<CLRADC14INIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14INIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14INIFG` writer - clear ADC14INIFG"]
pub struct CLRADC14INIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14INIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14INIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14inifg_0(self) -> &'a mut W {
        self.variant(CLRADC14INIFG_AW::CLRADC14INIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14inifg_1(self) -> &'a mut W {
        self.variant(CLRADC14INIFG_AW::CLRADC14INIFG_1)
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
#[doc = "clear ADC14LOIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14LOIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14LOIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14LOIFG_1 = 1,
}
impl From<CLRADC14LOIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14LOIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14LOIFG` writer - clear ADC14LOIFG"]
pub struct CLRADC14LOIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14LOIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14LOIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14loifg_0(self) -> &'a mut W {
        self.variant(CLRADC14LOIFG_AW::CLRADC14LOIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14loifg_1(self) -> &'a mut W {
        self.variant(CLRADC14LOIFG_AW::CLRADC14LOIFG_1)
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
#[doc = "clear ADC14HIIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14HIIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14HIIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14HIIFG_1 = 1,
}
impl From<CLRADC14HIIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14HIIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14HIIFG` writer - clear ADC14HIIFG"]
pub struct CLRADC14HIIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14HIIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14HIIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14hiifg_0(self) -> &'a mut W {
        self.variant(CLRADC14HIIFG_AW::CLRADC14HIIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14hiifg_1(self) -> &'a mut W {
        self.variant(CLRADC14HIIFG_AW::CLRADC14HIIFG_1)
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
#[doc = "clear ADC14OVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14OVIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14OVIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14OVIFG_1 = 1,
}
impl From<CLRADC14OVIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14OVIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14OVIFG` writer - clear ADC14OVIFG"]
pub struct CLRADC14OVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14OVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14OVIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14ovifg_0(self) -> &'a mut W {
        self.variant(CLRADC14OVIFG_AW::CLRADC14OVIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14ovifg_1(self) -> &'a mut W {
        self.variant(CLRADC14OVIFG_AW::CLRADC14OVIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "clear ADC14TOVIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14TOVIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14TOVIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14TOVIFG_1 = 1,
}
impl From<CLRADC14TOVIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14TOVIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14TOVIFG` writer - clear ADC14TOVIFG"]
pub struct CLRADC14TOVIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14TOVIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14TOVIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14tovifg_0(self) -> &'a mut W {
        self.variant(CLRADC14TOVIFG_AW::CLRADC14TOVIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14tovifg_1(self) -> &'a mut W {
        self.variant(CLRADC14TOVIFG_AW::CLRADC14TOVIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "clear ADC14RDYIFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLRADC14RDYIFG_AW {
    #[doc = "0: no effect"]
    CLRADC14RDYIFG_0 = 0,
    #[doc = "1: clear pending interrupt flag"]
    CLRADC14RDYIFG_1 = 1,
}
impl From<CLRADC14RDYIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRADC14RDYIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRADC14RDYIFG` writer - clear ADC14RDYIFG"]
pub struct CLRADC14RDYIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLRADC14RDYIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLRADC14RDYIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "no effect"]
    #[inline(always)]
    pub fn clradc14rdyifg_0(self) -> &'a mut W {
        self.variant(CLRADC14RDYIFG_AW::CLRADC14RDYIFG_0)
    }
    #[doc = "clear pending interrupt flag"]
    #[inline(always)]
    pub fn clradc14rdyifg_1(self) -> &'a mut W {
        self.variant(CLRADC14RDYIFG_AW::CLRADC14RDYIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl W {
    #[doc = "Bit 1 - clear ADC14INIFG"]
    #[inline(always)]
    pub fn clradc14inifg(&mut self) -> CLRADC14INIFG_W {
        CLRADC14INIFG_W { w: self }
    }
    #[doc = "Bit 2 - clear ADC14LOIFG"]
    #[inline(always)]
    pub fn clradc14loifg(&mut self) -> CLRADC14LOIFG_W {
        CLRADC14LOIFG_W { w: self }
    }
    #[doc = "Bit 3 - clear ADC14HIIFG"]
    #[inline(always)]
    pub fn clradc14hiifg(&mut self) -> CLRADC14HIIFG_W {
        CLRADC14HIIFG_W { w: self }
    }
    #[doc = "Bit 4 - clear ADC14OVIFG"]
    #[inline(always)]
    pub fn clradc14ovifg(&mut self) -> CLRADC14OVIFG_W {
        CLRADC14OVIFG_W { w: self }
    }
    #[doc = "Bit 5 - clear ADC14TOVIFG"]
    #[inline(always)]
    pub fn clradc14tovifg(&mut self) -> CLRADC14TOVIFG_W {
        CLRADC14TOVIFG_W { w: self }
    }
    #[doc = "Bit 6 - clear ADC14RDYIFG"]
    #[inline(always)]
    pub fn clradc14rdyifg(&mut self) -> CLRADC14RDYIFG_W {
        CLRADC14RDYIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14clrifgr1](index.html) module"]
pub struct ADC14CLRIFGR1_SPEC;
impl crate::RegisterSpec for ADC14CLRIFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14clrifgr1::R](R) reader structure"]
impl crate::Readable for ADC14CLRIFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14clrifgr1::W](W) writer structure"]
impl crate::Writable for ADC14CLRIFGR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14CLRIFGR1 to value 0"]
impl crate::Resettable for ADC14CLRIFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
