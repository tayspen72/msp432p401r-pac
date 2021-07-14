#[doc = "Register `AESASTAT` reader"]
pub struct R(crate::R<AESASTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AESASTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AESASTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AESASTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AESASTAT` writer"]
pub struct W(crate::W<AESASTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESASTAT_SPEC>;
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
impl From<crate::W<AESASTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESASTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "AES accelerator module busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESBUSY_A {
    #[doc = "0: Not busy"]
    AESBUSY_0 = 0,
    #[doc = "1: Busy"]
    AESBUSY_1 = 1,
}
impl From<AESBUSY_A> for bool {
    #[inline(always)]
    fn from(variant: AESBUSY_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESBUSY` reader - AES accelerator module busy"]
pub struct AESBUSY_R(crate::FieldReader<bool, AESBUSY_A>);
impl AESBUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESBUSY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESBUSY_A {
        match self.bits {
            false => AESBUSY_A::AESBUSY_0,
            true => AESBUSY_A::AESBUSY_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESBUSY_0`"]
    #[inline(always)]
    pub fn is_aesbusy_0(&self) -> bool {
        **self == AESBUSY_A::AESBUSY_0
    }
    #[doc = "Checks if the value of the field is `AESBUSY_1`"]
    #[inline(always)]
    pub fn is_aesbusy_1(&self) -> bool {
        **self == AESBUSY_A::AESBUSY_1
    }
}
impl core::ops::Deref for AESBUSY_R {
    type Target = crate::FieldReader<bool, AESBUSY_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESBUSY` writer - AES accelerator module busy"]
pub struct AESBUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> AESBUSY_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESBUSY_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not busy"]
    #[inline(always)]
    pub fn aesbusy_0(self) -> &'a mut W {
        self.variant(AESBUSY_A::AESBUSY_0)
    }
    #[doc = "Busy"]
    #[inline(always)]
    pub fn aesbusy_1(self) -> &'a mut W {
        self.variant(AESBUSY_A::AESBUSY_1)
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
#[doc = "All 16 bytes written to AESAKEY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESKEYWR_A {
    #[doc = "0: Not all bytes written"]
    AESKEYWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESKEYWR_1 = 1,
}
impl From<AESKEYWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESKEYWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESKEYWR` reader - All 16 bytes written to AESAKEY"]
pub struct AESKEYWR_R(crate::FieldReader<bool, AESKEYWR_A>);
impl AESKEYWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESKEYWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESKEYWR_A {
        match self.bits {
            false => AESKEYWR_A::AESKEYWR_0,
            true => AESKEYWR_A::AESKEYWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_0`"]
    #[inline(always)]
    pub fn is_aeskeywr_0(&self) -> bool {
        **self == AESKEYWR_A::AESKEYWR_0
    }
    #[doc = "Checks if the value of the field is `AESKEYWR_1`"]
    #[inline(always)]
    pub fn is_aeskeywr_1(&self) -> bool {
        **self == AESKEYWR_A::AESKEYWR_1
    }
}
impl core::ops::Deref for AESKEYWR_R {
    type Target = crate::FieldReader<bool, AESKEYWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESKEYWR` writer - All 16 bytes written to AESAKEY"]
pub struct AESKEYWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESKEYWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESKEYWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aeskeywr_0(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aeskeywr_1(self) -> &'a mut W {
        self.variant(AESKEYWR_A::AESKEYWR_1)
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
#[doc = "All 16 bytes written to AESADIN, AESAXDIN or AESAXIN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDINWR_A {
    #[doc = "0: Not all bytes written"]
    AESDINWR_0 = 0,
    #[doc = "1: All bytes written"]
    AESDINWR_1 = 1,
}
impl From<AESDINWR_A> for bool {
    #[inline(always)]
    fn from(variant: AESDINWR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESDINWR` reader - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub struct AESDINWR_R(crate::FieldReader<bool, AESDINWR_A>);
impl AESDINWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESDINWR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDINWR_A {
        match self.bits {
            false => AESDINWR_A::AESDINWR_0,
            true => AESDINWR_A::AESDINWR_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDINWR_0`"]
    #[inline(always)]
    pub fn is_aesdinwr_0(&self) -> bool {
        **self == AESDINWR_A::AESDINWR_0
    }
    #[doc = "Checks if the value of the field is `AESDINWR_1`"]
    #[inline(always)]
    pub fn is_aesdinwr_1(&self) -> bool {
        **self == AESDINWR_A::AESDINWR_1
    }
}
impl core::ops::Deref for AESDINWR_R {
    type Target = crate::FieldReader<bool, AESDINWR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESDINWR` writer - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
pub struct AESDINWR_W<'a> {
    w: &'a mut W,
}
impl<'a> AESDINWR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AESDINWR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Not all bytes written"]
    #[inline(always)]
    pub fn aesdinwr_0(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_0)
    }
    #[doc = "All bytes written"]
    #[inline(always)]
    pub fn aesdinwr_1(self) -> &'a mut W {
        self.variant(AESDINWR_A::AESDINWR_1)
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
#[doc = "All 16 bytes read from AESADOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AESDOUTRD_A {
    #[doc = "0: Not all bytes read"]
    AESDOUTRD_0 = 0,
    #[doc = "1: All bytes read"]
    AESDOUTRD_1 = 1,
}
impl From<AESDOUTRD_A> for bool {
    #[inline(always)]
    fn from(variant: AESDOUTRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AESDOUTRD` reader - All 16 bytes read from AESADOUT"]
pub struct AESDOUTRD_R(crate::FieldReader<bool, AESDOUTRD_A>);
impl AESDOUTRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        AESDOUTRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AESDOUTRD_A {
        match self.bits {
            false => AESDOUTRD_A::AESDOUTRD_0,
            true => AESDOUTRD_A::AESDOUTRD_1,
        }
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_0`"]
    #[inline(always)]
    pub fn is_aesdoutrd_0(&self) -> bool {
        **self == AESDOUTRD_A::AESDOUTRD_0
    }
    #[doc = "Checks if the value of the field is `AESDOUTRD_1`"]
    #[inline(always)]
    pub fn is_aesdoutrd_1(&self) -> bool {
        **self == AESDOUTRD_A::AESDOUTRD_1
    }
}
impl core::ops::Deref for AESDOUTRD_R {
    type Target = crate::FieldReader<bool, AESDOUTRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESKEYCNTx` reader - Bytes written via AESAKEY for AESKLx=00, half-words written via AESAKEY"]
pub struct AESKEYCNTX_R(crate::FieldReader<u8, u8>);
impl AESKEYCNTX_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESKEYCNTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESKEYCNTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESDINCNTx` reader - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
pub struct AESDINCNTX_R(crate::FieldReader<u8, u8>);
impl AESDINCNTX_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESDINCNTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESDINCNTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AESDOUTCNTx` reader - Bytes read via AESADOUT"]
pub struct AESDOUTCNTX_R(crate::FieldReader<u8, u8>);
impl AESDOUTCNTX_R {
    pub(crate) fn new(bits: u8) -> Self {
        AESDOUTCNTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AESDOUTCNTX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&self) -> AESBUSY_R {
        AESBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&self) -> AESKEYWR_R {
        AESKEYWR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&self) -> AESDINWR_R {
        AESDINWR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - All 16 bytes read from AESADOUT"]
    #[inline(always)]
    pub fn aesdoutrd(&self) -> AESDOUTRD_R {
        AESDOUTRD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Bytes written via AESAKEY for AESKLx=00, half-words written via AESAKEY"]
    #[inline(always)]
    pub fn aeskeycntx(&self) -> AESKEYCNTX_R {
        AESKEYCNTX_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Bytes written via AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdincntx(&self) -> AESDINCNTX_R {
        AESDINCNTX_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Bytes read via AESADOUT"]
    #[inline(always)]
    pub fn aesdoutcntx(&self) -> AESDOUTCNTX_R {
        AESDOUTCNTX_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AES accelerator module busy"]
    #[inline(always)]
    pub fn aesbusy(&mut self) -> AESBUSY_W {
        AESBUSY_W { w: self }
    }
    #[doc = "Bit 1 - All 16 bytes written to AESAKEY"]
    #[inline(always)]
    pub fn aeskeywr(&mut self) -> AESKEYWR_W {
        AESKEYWR_W { w: self }
    }
    #[doc = "Bit 2 - All 16 bytes written to AESADIN, AESAXDIN or AESAXIN"]
    #[inline(always)]
    pub fn aesdinwr(&mut self) -> AESDINWR_W {
        AESDINWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesastat](index.html) module"]
pub struct AESASTAT_SPEC;
impl crate::RegisterSpec for AESASTAT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [aesastat::R](R) reader structure"]
impl crate::Readable for AESASTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aesastat::W](W) writer structure"]
impl crate::Writable for AESASTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESASTAT to value 0"]
impl crate::Resettable for AESASTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
