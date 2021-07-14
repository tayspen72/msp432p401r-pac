#[doc = "Register `UCAxIE` reader"]
pub struct R(crate::R<UCAXIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxIE` writer"]
pub struct W(crate::W<UCAXIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXIE_SPEC>;
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
impl From<crate::W<UCAXIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Receive interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCRXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCRXIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCRXIE_1 = 1,
}
impl From<UCRXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCRXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCRXIE` reader - Receive interrupt enable"]
pub struct UCRXIE_R(crate::FieldReader<bool, UCRXIE_A>);
impl UCRXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCRXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCRXIE_A {
        match self.bits {
            false => UCRXIE_A::UCRXIE_0,
            true => UCRXIE_A::UCRXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCRXIE_0`"]
    #[inline(always)]
    pub fn is_ucrxie_0(&self) -> bool {
        **self == UCRXIE_A::UCRXIE_0
    }
    #[doc = "Checks if the value of the field is `UCRXIE_1`"]
    #[inline(always)]
    pub fn is_ucrxie_1(&self) -> bool {
        **self == UCRXIE_A::UCRXIE_1
    }
}
impl core::ops::Deref for UCRXIE_R {
    type Target = crate::FieldReader<bool, UCRXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCRXIE` writer - Receive interrupt enable"]
pub struct UCRXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCRXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCRXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucrxie_0(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucrxie_1(self) -> &'a mut W {
        self.variant(UCRXIE_A::UCRXIE_1)
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
#[doc = "Transmit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXIE_1 = 1,
}
impl From<UCTXIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXIE` reader - Transmit interrupt enable"]
pub struct UCTXIE_R(crate::FieldReader<bool, UCTXIE_A>);
impl UCTXIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXIE_A {
        match self.bits {
            false => UCTXIE_A::UCTXIE_0,
            true => UCTXIE_A::UCTXIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXIE_0`"]
    #[inline(always)]
    pub fn is_uctxie_0(&self) -> bool {
        **self == UCTXIE_A::UCTXIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXIE_1`"]
    #[inline(always)]
    pub fn is_uctxie_1(&self) -> bool {
        **self == UCTXIE_A::UCTXIE_1
    }
}
impl core::ops::Deref for UCTXIE_R {
    type Target = crate::FieldReader<bool, UCTXIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXIE` writer - Transmit interrupt enable"]
pub struct UCTXIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxie_0(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxie_1(self) -> &'a mut W {
        self.variant(UCTXIE_A::UCTXIE_1)
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
#[doc = "Start bit interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCSTTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCSTTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCSTTIE_1 = 1,
}
impl From<UCSTTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCSTTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCSTTIE` reader - Start bit interrupt enable"]
pub struct UCSTTIE_R(crate::FieldReader<bool, UCSTTIE_A>);
impl UCSTTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCSTTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCSTTIE_A {
        match self.bits {
            false => UCSTTIE_A::UCSTTIE_0,
            true => UCSTTIE_A::UCSTTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_0`"]
    #[inline(always)]
    pub fn is_ucsttie_0(&self) -> bool {
        **self == UCSTTIE_A::UCSTTIE_0
    }
    #[doc = "Checks if the value of the field is `UCSTTIE_1`"]
    #[inline(always)]
    pub fn is_ucsttie_1(&self) -> bool {
        **self == UCSTTIE_A::UCSTTIE_1
    }
}
impl core::ops::Deref for UCSTTIE_R {
    type Target = crate::FieldReader<bool, UCSTTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCSTTIE` writer - Start bit interrupt enable"]
pub struct UCSTTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCSTTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCSTTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ucsttie_0(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ucsttie_1(self) -> &'a mut W {
        self.variant(UCSTTIE_A::UCSTTIE_1)
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
#[doc = "Transmit complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UCTXCPTIE_A {
    #[doc = "0: Interrupt disabled"]
    UCTXCPTIE_0 = 0,
    #[doc = "1: Interrupt enabled"]
    UCTXCPTIE_1 = 1,
}
impl From<UCTXCPTIE_A> for bool {
    #[inline(always)]
    fn from(variant: UCTXCPTIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UCTXCPTIE` reader - Transmit complete interrupt enable"]
pub struct UCTXCPTIE_R(crate::FieldReader<bool, UCTXCPTIE_A>);
impl UCTXCPTIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        UCTXCPTIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCTXCPTIE_A {
        match self.bits {
            false => UCTXCPTIE_A::UCTXCPTIE_0,
            true => UCTXCPTIE_A::UCTXCPTIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_0`"]
    #[inline(always)]
    pub fn is_uctxcptie_0(&self) -> bool {
        **self == UCTXCPTIE_A::UCTXCPTIE_0
    }
    #[doc = "Checks if the value of the field is `UCTXCPTIE_1`"]
    #[inline(always)]
    pub fn is_uctxcptie_1(&self) -> bool {
        **self == UCTXCPTIE_A::UCTXCPTIE_1
    }
}
impl core::ops::Deref for UCTXCPTIE_R {
    type Target = crate::FieldReader<bool, UCTXCPTIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTXCPTIE` writer - Transmit complete interrupt enable"]
pub struct UCTXCPTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTXCPTIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCTXCPTIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn uctxcptie_0(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn uctxcptie_1(self) -> &'a mut W {
        self.variant(UCTXCPTIE_A::UCTXCPTIE_1)
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
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&self) -> UCRXIE_R {
        UCRXIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&self) -> UCTXIE_R {
        UCTXIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&self) -> UCSTTIE_R {
        UCSTTIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&self) -> UCTXCPTIE_R {
        UCTXCPTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive interrupt enable"]
    #[inline(always)]
    pub fn ucrxie(&mut self) -> UCRXIE_W {
        UCRXIE_W { w: self }
    }
    #[doc = "Bit 1 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn uctxie(&mut self) -> UCTXIE_W {
        UCTXIE_W { w: self }
    }
    #[doc = "Bit 2 - Start bit interrupt enable"]
    #[inline(always)]
    pub fn ucsttie(&mut self) -> UCSTTIE_W {
        UCSTTIE_W { w: self }
    }
    #[doc = "Bit 3 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn uctxcptie(&mut self) -> UCTXCPTIE_W {
        UCTXCPTIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ie](index.html) module"]
pub struct UCAXIE_SPEC;
impl crate::RegisterSpec for UCAXIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_ie::R](R) reader structure"]
impl crate::Readable for UCAXIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_ie::W](W) writer structure"]
impl crate::Writable for UCAXIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxIE to value 0"]
impl crate::Resettable for UCAXIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
