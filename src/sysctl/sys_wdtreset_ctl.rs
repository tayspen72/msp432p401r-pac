#[doc = "Register `SYS_WDTRESET_CTL` reader"]
pub struct R(crate::R<SYS_WDTRESET_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_WDTRESET_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_WDTRESET_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_WDTRESET_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_WDTRESET_CTL` writer"]
pub struct W(crate::W<SYS_WDTRESET_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_WDTRESET_CTL_SPEC>;
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
impl From<crate::W<SYS_WDTRESET_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_WDTRESET_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "WDT timeout reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMEOUT_A {
    #[doc = "0: WDT timeout event generates Soft reset"]
    TIMEOUT_0 = 0,
    #[doc = "1: WDT timeout event generates Hard reset"]
    TIMEOUT_1 = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMEOUT` reader - WDT timeout reset type"]
pub struct TIMEOUT_R(crate::FieldReader<bool, TIMEOUT_A>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::TIMEOUT_0,
            true => TIMEOUT_A::TIMEOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_0`"]
    #[inline(always)]
    pub fn is_timeout_0(&self) -> bool {
        **self == TIMEOUT_A::TIMEOUT_0
    }
    #[doc = "Checks if the value of the field is `TIMEOUT_1`"]
    #[inline(always)]
    pub fn is_timeout_1(&self) -> bool {
        **self == TIMEOUT_A::TIMEOUT_1
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, TIMEOUT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - WDT timeout reset type"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMEOUT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT timeout event generates Soft reset"]
    #[inline(always)]
    pub fn timeout_0(self) -> &'a mut W {
        self.variant(TIMEOUT_A::TIMEOUT_0)
    }
    #[doc = "WDT timeout event generates Hard reset"]
    #[inline(always)]
    pub fn timeout_1(self) -> &'a mut W {
        self.variant(TIMEOUT_A::TIMEOUT_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "WDT password violation reset type\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VIOLATION_A {
    #[doc = "0: WDT password violation event generates Soft reset"]
    VIOLATION_0 = 0,
    #[doc = "1: WDT password violation event generates Hard reset"]
    VIOLATION_1 = 1,
}
impl From<VIOLATION_A> for bool {
    #[inline(always)]
    fn from(variant: VIOLATION_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIOLATION` reader - WDT password violation reset type"]
pub struct VIOLATION_R(crate::FieldReader<bool, VIOLATION_A>);
impl VIOLATION_R {
    pub(crate) fn new(bits: bool) -> Self {
        VIOLATION_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VIOLATION_A {
        match self.bits {
            false => VIOLATION_A::VIOLATION_0,
            true => VIOLATION_A::VIOLATION_1,
        }
    }
    #[doc = "Checks if the value of the field is `VIOLATION_0`"]
    #[inline(always)]
    pub fn is_violation_0(&self) -> bool {
        **self == VIOLATION_A::VIOLATION_0
    }
    #[doc = "Checks if the value of the field is `VIOLATION_1`"]
    #[inline(always)]
    pub fn is_violation_1(&self) -> bool {
        **self == VIOLATION_A::VIOLATION_1
    }
}
impl core::ops::Deref for VIOLATION_R {
    type Target = crate::FieldReader<bool, VIOLATION_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VIOLATION` writer - WDT password violation reset type"]
pub struct VIOLATION_W<'a> {
    w: &'a mut W,
}
impl<'a> VIOLATION_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: VIOLATION_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "WDT password violation event generates Soft reset"]
    #[inline(always)]
    pub fn violation_0(self) -> &'a mut W {
        self.variant(VIOLATION_A::VIOLATION_0)
    }
    #[doc = "WDT password violation event generates Hard reset"]
    #[inline(always)]
    pub fn violation_1(self) -> &'a mut W {
        self.variant(VIOLATION_A::VIOLATION_1)
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
impl R {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&self) -> VIOLATION_R {
        VIOLATION_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WDT timeout reset type"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - WDT password violation reset type"]
    #[inline(always)]
    pub fn violation(&mut self) -> VIOLATION_W {
        VIOLATION_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_wdtreset_ctl](index.html) module"]
pub struct SYS_WDTRESET_CTL_SPEC;
impl crate::RegisterSpec for SYS_WDTRESET_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_wdtreset_ctl::R](R) reader structure"]
impl crate::Readable for SYS_WDTRESET_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_wdtreset_ctl::W](W) writer structure"]
impl crate::Writable for SYS_WDTRESET_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_WDTRESET_CTL to value 0x03"]
impl crate::Resettable for SYS_WDTRESET_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
