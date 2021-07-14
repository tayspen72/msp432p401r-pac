#[doc = "Register `UCAxIFG` reader"]
pub struct R(crate::R<UCAXIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxIFG` writer"]
pub struct W(crate::W<UCAXIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXIFG_SPEC>;
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
impl From<crate::W<UCAXIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCRXIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCRXIFG_1 = 1,
}
impl From<UCRXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIFG` reader - Receive interrupt flag"]
pub struct UCRXIFG_R(crate::FieldReader<bool, UCRXIFG_A>);
impl UCRXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIFG_A {
        match self.bits {
            false => UCRXIFG_A::UCRXIFG_0,
            true => UCRXIFG_A::UCRXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_0`"]
    #[inline(always)]
    pub fn is_ucrxifg_0(&self) -> bool {
        **self == UCRXIFG_A::UCRXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCRXIFG_1`"]
    #[inline(always)]
    pub fn is_ucrxifg_1(&self) -> bool {
        **self == UCRXIFG_A::UCRXIFG_1
    }
}
impl core::ops::Deref for UCRXIFG_R {
    type Target = crate::FieldReader<bool, UCRXIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIFG` writer - Receive interrupt flag"]
pub struct UCRXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_0(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucrxifg_1(self) -> &'a mut W {
        self.variant(UCRXIFG_A::UCRXIFG_1)
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
#[doc = "Transmit interrupt flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIFG_A {
    #[doc = "0: No interrupt pending"]
    UCTXIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXIFG_1 = 1,
}
impl From<UCTXIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIFG` reader - Transmit interrupt flag"]
pub struct UCTXIFG_R(crate::FieldReader<bool, UCTXIFG_A>);
impl UCTXIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIFG_A {
        match self.bits {
            false => UCTXIFG_A::UCTXIFG_0,
            true => UCTXIFG_A::UCTXIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_0`"]
    #[inline(always)]
    pub fn is_uctxifg_0(&self) -> bool {
        **self == UCTXIFG_A::UCTXIFG_0
    }
    #[doc = "Checks if the value of the field is `UCTXIFG_1`"]
    #[inline(always)]
    pub fn is_uctxifg_1(&self) -> bool {
        **self == UCTXIFG_A::UCTXIFG_1
    }
}
impl core::ops::Deref for UCTXIFG_R {
    type Target = crate::FieldReader<bool, UCTXIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIFG` writer - Transmit interrupt flag"]
pub struct UCTXIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_0(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxifg_1(self) -> &'a mut W {
        self.variant(UCTXIFG_A::UCTXIFG_1)
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
#[doc = "Start bit interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCSTTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCSTTIFG_1 = 1,
}
impl From<UCSTTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIFG` reader - Start bit interrupt flag"]
pub struct UCSTTIFG_R(crate::FieldReader<bool, UCSTTIFG_A>);
impl UCSTTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIFG_A {
        match self.bits {
            false => UCSTTIFG_A::UCSTTIFG_0,
            true => UCSTTIFG_A::UCSTTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_0`"]
    #[inline(always)]
    pub fn is_ucsttifg_0(&self) -> bool {
        **self == UCSTTIFG_A::UCSTTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIFG_1`"]
    #[inline(always)]
    pub fn is_ucsttifg_1(&self) -> bool {
        **self == UCSTTIFG_A::UCSTTIFG_1
    }
}
impl core::ops::Deref for UCSTTIFG_R {
    type Target = crate::FieldReader<bool, UCSTTIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIFG` writer - Start bit interrupt flag"]
pub struct UCSTTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTTIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_0(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn ucsttifg_1(self) -> &'a mut W {
        self.variant(UCSTTIFG_A::UCSTTIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u16 & 0x01) << 2);
        self.w
    }
}
#[doc = "Transmit ready interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXCPTIFG_A {
    #[doc = "0: No interrupt pending"]
    UCTXCPTIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    UCTXCPTIFG_1 = 1,
}
impl From<UCTXCPTIFG_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXCPTIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXCPTIFG` reader - Transmit ready interrupt enable"]
pub struct UCTXCPTIFG_R(crate::FieldReader<bool, UCTXCPTIFG_A>);
impl UCTXCPTIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXCPTIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXCPTIFG_A {
        match self.bits {
            false => UCTXCPTIFG_A::UCTXCPTIFG_0,
            true => UCTXCPTIFG_A::UCTXCPTIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG_0`"]
    #[inline(always)]
    pub fn is_uctxcptifg_0(&self) -> bool {
        **self == UCTXCPTIFG_A::UCTXCPTIFG_0
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIFG_1`"]
    #[inline(always)]
    pub fn is_uctxcptifg_1(&self) -> bool {
        **self == UCTXCPTIFG_A::UCTXCPTIFG_1
    }
}
impl core::ops::Deref for UCTXCPTIFG_R {
    type Target = crate::FieldReader<bool, UCTXCPTIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXCPTIFG` writer - Transmit ready interrupt enable"]
pub struct UCTXCPTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXCPTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXCPTIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_0(self) -> &'a mut W {
        self.variant(UCTXCPTIFG_A::UCTXCPTIFG_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn uctxcptifg_1(self) -> &'a mut W {
        self.variant(UCTXCPTIFG_A::UCTXCPTIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u16 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&self) -> UCRXIFG_R {
        UCRXIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&self) -> UCTXIFG_R {
        UCTXIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start bit interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&self) -> UCSTTIFG_R {
        UCSTTIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit ready interrupt enable"]
    #[inline(always)]
    pub fn uctxcptifg(&self) -> UCTXCPTIFG_R {
        UCTXCPTIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt flag"]
    #[inline(always)]
    pub fn ucrxifg(&mut self) -> UCRXIFG_W {
        UCRXIFG_W { w: self }
    }
    #[doc = "Bit 1 - Transmit interrupt flag"]
    #[inline(always)]
    pub fn uctxifg(&mut self) -> UCTXIFG_W {
        UCTXIFG_W { w: self }
    }
    #[doc = "Bit 2 - Start bit interrupt flag"]
    #[inline(always)]
    pub fn ucsttifg(&mut self) -> UCSTTIFG_W {
        UCSTTIFG_W { w: self }
    }
    #[doc = "Bit 3 - Transmit ready interrupt enable"]
    #[inline(always)]
    pub fn uctxcptifg(&mut self) -> UCTXCPTIFG_W {
        UCTXCPTIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ifg](index.html) module"]
pub struct UCAXIFG_SPEC;
impl crate::RegisterSpec for UCAXIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_ifg::R](R) reader structure"]
impl crate::Readable for UCAXIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_ifg::W](W) writer structure"]
impl crate::Writable for UCAXIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxIFG to value 0x02"]
impl crate::Resettable for UCAXIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}
