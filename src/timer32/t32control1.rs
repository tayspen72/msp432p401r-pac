#[doc = "Register `T32CONTROL1` reader"]
pub struct R(crate::R<T32CONTROL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32CONTROL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32CONTROL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32CONTROL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T32CONTROL1` writer"]
pub struct W(crate::W<T32CONTROL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32CONTROL1_SPEC>;
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
impl From<crate::W<T32CONTROL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32CONTROL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects one-shot or wrapping counter mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ONESHOT_A {
    #[doc = "0: wrapping mode"]
    ONESHOT_0 = 0,
    #[doc = "1: one-shot mode"]
    ONESHOT_1 = 1,
}
impl From<ONESHOT_A> for bool {
    #[inline(always)]
    fn from(variant: ONESHOT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ONESHOT` reader - Selects one-shot or wrapping counter mode"]
pub struct ONESHOT_R(crate::FieldReader<bool, ONESHOT_A>);
impl ONESHOT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ONESHOT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONESHOT_A {
        match self.bits {
            false => ONESHOT_A::ONESHOT_0,
            true => ONESHOT_A::ONESHOT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ONESHOT_0`"]
    #[inline(always)]
    pub fn is_oneshot_0(&self) -> bool {
        **self == ONESHOT_A::ONESHOT_0
    }
    #[doc = "Checks if the value of the field is `ONESHOT_1`"]
    #[inline(always)]
    pub fn is_oneshot_1(&self) -> bool {
        **self == ONESHOT_A::ONESHOT_1
    }
}
impl core::ops::Deref for ONESHOT_R {
    type Target = crate::FieldReader<bool, ONESHOT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ONESHOT` writer - Selects one-shot or wrapping counter mode"]
pub struct ONESHOT_W<'a> {
    w: &'a mut W,
}
impl<'a> ONESHOT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ONESHOT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "wrapping mode"]
    #[inline(always)]
    pub fn oneshot_0(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_0)
    }
    #[doc = "one-shot mode"]
    #[inline(always)]
    pub fn oneshot_1(self) -> &'a mut W {
        self.variant(ONESHOT_A::ONESHOT_1)
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
#[doc = "Selects 16 or 32 bit counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIZE_A {
    #[doc = "0: 16-bit counter"]
    SIZE_0 = 0,
    #[doc = "1: 32-bit counter"]
    SIZE_1 = 1,
}
impl From<SIZE_A> for bool {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SIZE` reader - Selects 16 or 32 bit counter operation"]
pub struct SIZE_R(crate::FieldReader<bool, SIZE_A>);
impl SIZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIZE_A {
        match self.bits {
            false => SIZE_A::SIZE_0,
            true => SIZE_A::SIZE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_0`"]
    #[inline(always)]
    pub fn is_size_0(&self) -> bool {
        **self == SIZE_A::SIZE_0
    }
    #[doc = "Checks if the value of the field is `SIZE_1`"]
    #[inline(always)]
    pub fn is_size_1(&self) -> bool {
        **self == SIZE_A::SIZE_1
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<bool, SIZE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIZE` writer - Selects 16 or 32 bit counter operation"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "16-bit counter"]
    #[inline(always)]
    pub fn size_0(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_0)
    }
    #[doc = "32-bit counter"]
    #[inline(always)]
    pub fn size_1(self) -> &'a mut W {
        self.variant(SIZE_A::SIZE_1)
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
#[doc = "Prescale bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESCALE_A {
    #[doc = "0: 0 stages of prescale, clock is divided by 1"]
    PRESCALE_0 = 0,
    #[doc = "1: 4 stages of prescale, clock is divided by 16"]
    PRESCALE_1 = 1,
    #[doc = "2: 8 stages of prescale, clock is divided by 256"]
    PRESCALE_2 = 2,
}
impl From<PRESCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRESCALE` reader - Prescale bits"]
pub struct PRESCALE_R(crate::FieldReader<u8, PRESCALE_A>);
impl PRESCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PRESCALE_A> {
        match self.bits {
            0 => Some(PRESCALE_A::PRESCALE_0),
            1 => Some(PRESCALE_A::PRESCALE_1),
            2 => Some(PRESCALE_A::PRESCALE_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRESCALE_0`"]
    #[inline(always)]
    pub fn is_prescale_0(&self) -> bool {
        **self == PRESCALE_A::PRESCALE_0
    }
    #[doc = "Checks if the value of the field is `PRESCALE_1`"]
    #[inline(always)]
    pub fn is_prescale_1(&self) -> bool {
        **self == PRESCALE_A::PRESCALE_1
    }
    #[doc = "Checks if the value of the field is `PRESCALE_2`"]
    #[inline(always)]
    pub fn is_prescale_2(&self) -> bool {
        **self == PRESCALE_A::PRESCALE_2
    }
}
impl core::ops::Deref for PRESCALE_R {
    type Target = crate::FieldReader<u8, PRESCALE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRESCALE` writer - Prescale bits"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCALE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "0 stages of prescale, clock is divided by 1"]
    #[inline(always)]
    pub fn prescale_0(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_0)
    }
    #[doc = "4 stages of prescale, clock is divided by 16"]
    #[inline(always)]
    pub fn prescale_1(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_1)
    }
    #[doc = "8 stages of prescale, clock is divided by 256"]
    #[inline(always)]
    pub fn prescale_2(self) -> &'a mut W {
        self.variant(PRESCALE_A::PRESCALE_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Interrupt enable bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IE_A {
    #[doc = "0: Timer interrupt disabled"]
    IE_0 = 0,
    #[doc = "1: Timer interrupt enabled"]
    IE_1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IE` reader - Interrupt enable bit"]
pub struct IE_R(crate::FieldReader<bool, IE_A>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::IE_0,
            true => IE_A::IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IE_0`"]
    #[inline(always)]
    pub fn is_ie_0(&self) -> bool {
        **self == IE_A::IE_0
    }
    #[doc = "Checks if the value of the field is `IE_1`"]
    #[inline(always)]
    pub fn is_ie_1(&self) -> bool {
        **self == IE_A::IE_1
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, IE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Interrupt enable bit"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer interrupt disabled"]
    #[inline(always)]
    pub fn ie_0(self) -> &'a mut W {
        self.variant(IE_A::IE_0)
    }
    #[doc = "Timer interrupt enabled"]
    #[inline(always)]
    pub fn ie_1(self) -> &'a mut W {
        self.variant(IE_A::IE_1)
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
#[doc = "Mode bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MODE_A {
    #[doc = "0: Timer is in free-running mode"]
    MODE_0 = 0,
    #[doc = "1: Timer is in periodic mode"]
    MODE_1 = 1,
}
impl From<MODE_A> for bool {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Mode bit"]
pub struct MODE_R(crate::FieldReader<bool, MODE_A>);
impl MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            false => MODE_A::MODE_0,
            true => MODE_A::MODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE_0`"]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        **self == MODE_A::MODE_0
    }
    #[doc = "Checks if the value of the field is `MODE_1`"]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        **self == MODE_A::MODE_1
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<bool, MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE` writer - Mode bit"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer is in free-running mode"]
    #[inline(always)]
    pub fn mode_0(self) -> &'a mut W {
        self.variant(MODE_A::MODE_0)
    }
    #[doc = "Timer is in periodic mode"]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut W {
        self.variant(MODE_A::MODE_1)
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
#[doc = "Enable bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENABLE_A {
    #[doc = "0: Timer disabled"]
    ENABLE_0 = 0,
    #[doc = "1: Timer enabled"]
    ENABLE_1 = 1,
}
impl From<ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable bit"]
pub struct ENABLE_R(crate::FieldReader<bool, ENABLE_A>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENABLE_A {
        match self.bits {
            false => ENABLE_A::ENABLE_0,
            true => ENABLE_A::ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE_0`"]
    #[inline(always)]
    pub fn is_enable_0(&self) -> bool {
        **self == ENABLE_A::ENABLE_0
    }
    #[doc = "Checks if the value of the field is `ENABLE_1`"]
    #[inline(always)]
    pub fn is_enable_1(&self) -> bool {
        **self == ENABLE_A::ENABLE_1
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, ENABLE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable bit"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENABLE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Timer disabled"]
    #[inline(always)]
    pub fn enable_0(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_0)
    }
    #[doc = "Timer enabled"]
    #[inline(always)]
    pub fn enable_1(self) -> &'a mut W {
        self.variant(ENABLE_A::ENABLE_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&self) -> ONESHOT_R {
        ONESHOT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects one-shot or wrapping counter mode"]
    #[inline(always)]
    pub fn oneshot(&mut self) -> ONESHOT_W {
        ONESHOT_W { w: self }
    }
    #[doc = "Bit 1 - Selects 16 or 32 bit counter operation"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 2:3 - Prescale bits"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 5 - Interrupt enable bit"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bit 6 - Mode bit"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 7 - Enable bit"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 1 Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32control1](index.html) module"]
pub struct T32CONTROL1_SPEC;
impl crate::RegisterSpec for T32CONTROL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32control1::R](R) reader structure"]
impl crate::Readable for T32CONTROL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t32control1::W](W) writer structure"]
impl crate::Writable for T32CONTROL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32CONTROL1 to value 0x20"]
impl crate::Resettable for T32CONTROL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
