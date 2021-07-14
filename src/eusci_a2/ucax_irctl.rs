#[doc = "Register `UCAxIRCTL` reader"]
pub struct R(crate::R<UCAXIRCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXIRCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXIRCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXIRCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxIRCTL` writer"]
pub struct W(crate::W<UCAXIRCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXIRCTL_SPEC>;
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
impl From<crate::W<UCAXIRCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXIRCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IrDA encoder/decoder enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIREN_A {
    #[doc = "0: IrDA encoder/decoder disabled"]
    UCIREN_0 = 0,
    #[doc = "1: IrDA encoder/decoder enabled"]
    UCIREN_1 = 1,
}
impl From<UCIREN_A> for bool {
    #[inline(always)]
    fn from(variant: UCIREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIREN` reader - IrDA encoder/decoder enable"]
pub struct UCIREN_R(crate::FieldReader<bool, UCIREN_A>);
impl UCIREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCIREN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIREN_A {
        match self.bits {
            false => UCIREN_A::UCIREN_0,
            true => UCIREN_A::UCIREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIREN_0`"]
    #[inline(always)]
    pub fn is_uciren_0(&self) -> bool {
        **self == UCIREN_A::UCIREN_0
    }
    #[doc = "Checks if the value of the field is `UCIREN_1`"]
    #[inline(always)]
    pub fn is_uciren_1(&self) -> bool {
        **self == UCIREN_A::UCIREN_1
    }
}
impl core::ops::Deref for UCIREN_R {
    type Target = crate::FieldReader<bool, UCIREN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIREN` writer - IrDA encoder/decoder enable"]
pub struct UCIREN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIREN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIREN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA encoder/decoder disabled"]
    #[inline(always)]
    pub fn uciren_0(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_0)
    }
    #[doc = "IrDA encoder/decoder enabled"]
    #[inline(always)]
    pub fn uciren_1(self) -> &'a mut W {
        self.variant(UCIREN_A::UCIREN_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u16 & 0x01);
        self.w
    }
}
#[doc = "IrDA transmit pulse clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRTXCLK_A {
    #[doc = "0: BRCLK"]
    UCIRTXCLK_0 = 0,
    #[doc = "1: BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    UCIRTXCLK_1 = 1,
}
impl From<UCIRTXCLK_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRTXCLK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRTXCLK` reader - IrDA transmit pulse clock select"]
pub struct UCIRTXCLK_R(crate::FieldReader<bool, UCIRTXCLK_A>);
impl UCIRTXCLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCIRTXCLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRTXCLK_A {
        match self.bits {
            false => UCIRTXCLK_A::UCIRTXCLK_0,
            true => UCIRTXCLK_A::UCIRTXCLK_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_0`"]
    #[inline(always)]
    pub fn is_ucirtxclk_0(&self) -> bool {
        **self == UCIRTXCLK_A::UCIRTXCLK_0
    }
    #[doc = "Checks if the value of the field is `UCIRTXCLK_1`"]
    #[inline(always)]
    pub fn is_ucirtxclk_1(&self) -> bool {
        **self == UCIRTXCLK_A::UCIRTXCLK_1
    }
}
impl core::ops::Deref for UCIRTXCLK_R {
    type Target = crate::FieldReader<bool, UCIRTXCLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXCLK` writer - IrDA transmit pulse clock select"]
pub struct UCIRTXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXCLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRTXCLK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "BRCLK"]
    #[inline(always)]
    pub fn ucirtxclk_0(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_0)
    }
    #[doc = "BITCLK16 when UCOS16 = 1. Otherwise, BRCLK."]
    #[inline(always)]
    pub fn ucirtxclk_1(self) -> &'a mut W {
        self.variant(UCIRTXCLK_A::UCIRTXCLK_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `UCIRTXPL` reader - Transmit pulse length"]
pub struct UCIRTXPL_R(crate::FieldReader<u8, u8>);
impl UCIRTXPL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCIRTXPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRTXPL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRTXPL` writer - Transmit pulse length"]
pub struct UCIRTXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRTXPL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | ((value as u16 & 0x3f) << 2);
        self.w
    }
}
#[doc = "IrDA receive filter enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRRXFE_A {
    #[doc = "0: Receive filter disabled"]
    UCIRRXFE_0 = 0,
    #[doc = "1: Receive filter enabled"]
    UCIRRXFE_1 = 1,
}
impl From<UCIRRXFE_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRRXFE` reader - IrDA receive filter enabled"]
pub struct UCIRRXFE_R(crate::FieldReader<bool, UCIRRXFE_A>);
impl UCIRRXFE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXFE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXFE_A {
        match self.bits {
            false => UCIRRXFE_A::UCIRRXFE_0,
            true => UCIRRXFE_A::UCIRRXFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_0`"]
    #[inline(always)]
    pub fn is_ucirrxfe_0(&self) -> bool {
        **self == UCIRRXFE_A::UCIRRXFE_0
    }
    #[doc = "Checks if the value of the field is `UCIRRXFE_1`"]
    #[inline(always)]
    pub fn is_ucirrxfe_1(&self) -> bool {
        **self == UCIRRXFE_A::UCIRRXFE_1
    }
}
impl core::ops::Deref for UCIRRXFE_R {
    type Target = crate::FieldReader<bool, UCIRRXFE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFE` writer - IrDA receive filter enabled"]
pub struct UCIRRXFE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRRXFE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Receive filter disabled"]
    #[inline(always)]
    pub fn ucirrxfe_0(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_0)
    }
    #[doc = "Receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe_1(self) -> &'a mut W {
        self.variant(UCIRRXFE_A::UCIRRXFE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u16 & 0x01) << 8);
        self.w
    }
}
#[doc = "IrDA receive input UCAxRXD polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCIRRXPL_A {
    #[doc = "0: IrDA transceiver delivers a high pulse when a light pulse is seen"]
    UCIRRXPL_0 = 0,
    #[doc = "1: IrDA transceiver delivers a low pulse when a light pulse is seen"]
    UCIRRXPL_1 = 1,
}
impl From<UCIRRXPL_A> for bool {
    #[inline(always)]
    fn from(variant: UCIRRXPL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCIRRXPL` reader - IrDA receive input UCAxRXD polarity"]
pub struct UCIRRXPL_R(crate::FieldReader<bool, UCIRRXPL_A>);
impl UCIRRXPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCIRRXPL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCIRRXPL_A {
        match self.bits {
            false => UCIRRXPL_A::UCIRRXPL_0,
            true => UCIRRXPL_A::UCIRRXPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCIRRXPL_0`"]
    #[inline(always)]
    pub fn is_ucirrxpl_0(&self) -> bool {
        **self == UCIRRXPL_A::UCIRRXPL_0
    }
    #[doc = "Checks if the value of the field is `UCIRRXPL_1`"]
    #[inline(always)]
    pub fn is_ucirrxpl_1(&self) -> bool {
        **self == UCIRRXPL_A::UCIRRXPL_1
    }
}
impl core::ops::Deref for UCIRRXPL_R {
    type Target = crate::FieldReader<bool, UCIRRXPL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXPL` writer - IrDA receive input UCAxRXD polarity"]
pub struct UCIRRXPL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXPL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCIRRXPL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "IrDA transceiver delivers a high pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn ucirrxpl_0(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::UCIRRXPL_0)
    }
    #[doc = "IrDA transceiver delivers a low pulse when a light pulse is seen"]
    #[inline(always)]
    pub fn ucirrxpl_1(self) -> &'a mut W {
        self.variant(UCIRRXPL_A::UCIRRXPL_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u16 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `UCIRRXFL` reader - Receive filter length"]
pub struct UCIRRXFL_R(crate::FieldReader<u8, u8>);
impl UCIRRXFL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCIRRXFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCIRRXFL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCIRRXFL` writer - Receive filter length"]
pub struct UCIRRXFL_W<'a> {
    w: &'a mut W,
}
impl<'a> UCIRRXFL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 10)) | ((value as u16 & 0x0f) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&self) -> UCIREN_R {
        UCIREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&self) -> UCIRTXCLK_R {
        UCIRTXCLK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&self) -> UCIRTXPL_R {
        UCIRTXPL_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&self) -> UCIRRXFE_R {
        UCIRRXFE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&self) -> UCIRRXPL_R {
        UCIRRXPL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 10:13 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&self) -> UCIRRXFL_R {
        UCIRRXFL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA encoder/decoder enable"]
    #[inline(always)]
    pub fn uciren(&mut self) -> UCIREN_W {
        UCIREN_W { w: self }
    }
    #[doc = "Bit 1 - IrDA transmit pulse clock select"]
    #[inline(always)]
    pub fn ucirtxclk(&mut self) -> UCIRTXCLK_W {
        UCIRTXCLK_W { w: self }
    }
    #[doc = "Bits 2:7 - Transmit pulse length"]
    #[inline(always)]
    pub fn ucirtxpl(&mut self) -> UCIRTXPL_W {
        UCIRTXPL_W { w: self }
    }
    #[doc = "Bit 8 - IrDA receive filter enabled"]
    #[inline(always)]
    pub fn ucirrxfe(&mut self) -> UCIRRXFE_W {
        UCIRRXFE_W { w: self }
    }
    #[doc = "Bit 9 - IrDA receive input UCAxRXD polarity"]
    #[inline(always)]
    pub fn ucirrxpl(&mut self) -> UCIRRXPL_W {
        UCIRRXPL_W { w: self }
    }
    #[doc = "Bits 10:13 - Receive filter length"]
    #[inline(always)]
    pub fn ucirrxfl(&mut self) -> UCIRRXFL_W {
        UCIRRXFL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax IrDA Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_irctl](index.html) module"]
pub struct UCAXIRCTL_SPEC;
impl crate::RegisterSpec for UCAXIRCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_irctl::R](R) reader structure"]
impl crate::Readable for UCAXIRCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_irctl::W](W) writer structure"]
impl crate::Writable for UCAXIRCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxIRCTL to value 0"]
impl crate::Resettable for UCAXIRCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
