#[doc = "Register `RTCPS1CTL` reader"]
pub struct R(crate::R<RTCPS1CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCPS1CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCPS1CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCPS1CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCPS1CTL` writer"]
pub struct W(crate::W<RTCPS1CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCPS1CTL_SPEC>;
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
impl From<crate::W<RTCPS1CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCPS1CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Prescale timer 1 interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT1PSIFG_A {
    #[doc = "0: No time event occurred"]
    RT1PSIFG_0 = 0,
    #[doc = "1: Time event occurred"]
    RT1PSIFG_1 = 1,
}
impl From<RT1PSIFG_A> for bool {
    #[inline(always)]
    fn from(variant: RT1PSIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIFG` reader - Prescale timer 1 interrupt flag"]
pub struct RT1PSIFG_R(crate::FieldReader<bool, RT1PSIFG_A>);
impl RT1PSIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT1PSIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSIFG_A {
        match self.bits {
            false => RT1PSIFG_A::RT1PSIFG_0,
            true => RT1PSIFG_A::RT1PSIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT1PSIFG_0`"]
    #[inline(always)]
    pub fn is_rt1psifg_0(&self) -> bool {
        **self == RT1PSIFG_A::RT1PSIFG_0
    }
    #[doc = "Checks if the value of the field is `RT1PSIFG_1`"]
    #[inline(always)]
    pub fn is_rt1psifg_1(&self) -> bool {
        **self == RT1PSIFG_A::RT1PSIFG_1
    }
}
impl core::ops::Deref for RT1PSIFG_R {
    type Target = crate::FieldReader<bool, RT1PSIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT1PSIFG` writer - Prescale timer 1 interrupt flag"]
pub struct RT1PSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSIFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_0(self) -> &'a mut W {
        self.variant(RT1PSIFG_A::RT1PSIFG_0)
    }
    #[doc = "Time event occurred"]
    #[inline(always)]
    pub fn rt1psifg_1(self) -> &'a mut W {
        self.variant(RT1PSIFG_A::RT1PSIFG_1)
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
#[doc = "Prescale timer 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RT1PSIE_A {
    #[doc = "0: Interrupt not enabled"]
    RT1PSIE_0 = 0,
    #[doc = "1: Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    RT1PSIE_1 = 1,
}
impl From<RT1PSIE_A> for bool {
    #[inline(always)]
    fn from(variant: RT1PSIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RT1PSIE` reader - Prescale timer 1 interrupt enable"]
pub struct RT1PSIE_R(crate::FieldReader<bool, RT1PSIE_A>);
impl RT1PSIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RT1PSIE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1PSIE_A {
        match self.bits {
            false => RT1PSIE_A::RT1PSIE_0,
            true => RT1PSIE_A::RT1PSIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RT1PSIE_0`"]
    #[inline(always)]
    pub fn is_rt1psie_0(&self) -> bool {
        **self == RT1PSIE_A::RT1PSIE_0
    }
    #[doc = "Checks if the value of the field is `RT1PSIE_1`"]
    #[inline(always)]
    pub fn is_rt1psie_1(&self) -> bool {
        **self == RT1PSIE_A::RT1PSIE_1
    }
}
impl core::ops::Deref for RT1PSIE_R {
    type Target = crate::FieldReader<bool, RT1PSIE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT1PSIE` writer - Prescale timer 1 interrupt enable"]
pub struct RT1PSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1PSIE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1PSIE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Interrupt not enabled"]
    #[inline(always)]
    pub fn rt1psie_0(self) -> &'a mut W {
        self.variant(RT1PSIE_A::RT1PSIE_0)
    }
    #[doc = "Interrupt enabled (LPM3/LPM3.5 wake-up enabled)"]
    #[inline(always)]
    pub fn rt1psie_1(self) -> &'a mut W {
        self.variant(RT1PSIE_A::RT1PSIE_1)
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
#[doc = "Prescale timer 1 interrupt interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RT1IP_A {
    #[doc = "0: Divide by 2"]
    RT1IP_0 = 0,
    #[doc = "1: Divide by 4"]
    RT1IP_1 = 1,
    #[doc = "2: Divide by 8"]
    RT1IP_2 = 2,
    #[doc = "3: Divide by 16"]
    RT1IP_3 = 3,
    #[doc = "4: Divide by 32"]
    RT1IP_4 = 4,
    #[doc = "5: Divide by 64"]
    RT1IP_5 = 5,
    #[doc = "6: Divide by 128"]
    RT1IP_6 = 6,
    #[doc = "7: Divide by 256"]
    RT1IP_7 = 7,
}
impl From<RT1IP_A> for u8 {
    #[inline(always)]
    fn from(variant: RT1IP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RT1IP` reader - Prescale timer 1 interrupt interval"]
pub struct RT1IP_R(crate::FieldReader<u8, RT1IP_A>);
impl RT1IP_R {
    pub(crate) fn new(bits: u8) -> Self {
        RT1IP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RT1IP_A {
        match self.bits {
            0 => RT1IP_A::RT1IP_0,
            1 => RT1IP_A::RT1IP_1,
            2 => RT1IP_A::RT1IP_2,
            3 => RT1IP_A::RT1IP_3,
            4 => RT1IP_A::RT1IP_4,
            5 => RT1IP_A::RT1IP_5,
            6 => RT1IP_A::RT1IP_6,
            7 => RT1IP_A::RT1IP_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RT1IP_0`"]
    #[inline(always)]
    pub fn is_rt1ip_0(&self) -> bool {
        **self == RT1IP_A::RT1IP_0
    }
    #[doc = "Checks if the value of the field is `RT1IP_1`"]
    #[inline(always)]
    pub fn is_rt1ip_1(&self) -> bool {
        **self == RT1IP_A::RT1IP_1
    }
    #[doc = "Checks if the value of the field is `RT1IP_2`"]
    #[inline(always)]
    pub fn is_rt1ip_2(&self) -> bool {
        **self == RT1IP_A::RT1IP_2
    }
    #[doc = "Checks if the value of the field is `RT1IP_3`"]
    #[inline(always)]
    pub fn is_rt1ip_3(&self) -> bool {
        **self == RT1IP_A::RT1IP_3
    }
    #[doc = "Checks if the value of the field is `RT1IP_4`"]
    #[inline(always)]
    pub fn is_rt1ip_4(&self) -> bool {
        **self == RT1IP_A::RT1IP_4
    }
    #[doc = "Checks if the value of the field is `RT1IP_5`"]
    #[inline(always)]
    pub fn is_rt1ip_5(&self) -> bool {
        **self == RT1IP_A::RT1IP_5
    }
    #[doc = "Checks if the value of the field is `RT1IP_6`"]
    #[inline(always)]
    pub fn is_rt1ip_6(&self) -> bool {
        **self == RT1IP_A::RT1IP_6
    }
    #[doc = "Checks if the value of the field is `RT1IP_7`"]
    #[inline(always)]
    pub fn is_rt1ip_7(&self) -> bool {
        **self == RT1IP_A::RT1IP_7
    }
}
impl core::ops::Deref for RT1IP_R {
    type Target = crate::FieldReader<u8, RT1IP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RT1IP` writer - Prescale timer 1 interrupt interval"]
pub struct RT1IP_W<'a> {
    w: &'a mut W,
}
impl<'a> RT1IP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RT1IP_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn rt1ip_0(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_0)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn rt1ip_1(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_1)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn rt1ip_2(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_2)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn rt1ip_3(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_3)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn rt1ip_4(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_4)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn rt1ip_5(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_5)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn rt1ip_6(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_6)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn rt1ip_7(self) -> &'a mut W {
        self.variant(RT1IP_A::RT1IP_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u16 & 0x07) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&self) -> RT1PSIFG_R {
        RT1PSIFG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&self) -> RT1PSIE_R {
        RT1PSIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&self) -> RT1IP_R {
        RT1IP_R::new(((self.bits >> 2) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Prescale timer 1 interrupt flag"]
    #[inline(always)]
    pub fn rt1psifg(&mut self) -> RT1PSIFG_W {
        RT1PSIFG_W { w: self }
    }
    #[doc = "Bit 1 - Prescale timer 1 interrupt enable"]
    #[inline(always)]
    pub fn rt1psie(&mut self) -> RT1PSIE_W {
        RT1PSIE_W { w: self }
    }
    #[doc = "Bits 2:4 - Prescale timer 1 interrupt interval"]
    #[inline(always)]
    pub fn rt1ip(&mut self) -> RT1IP_W {
        RT1IP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real-Time Clock Prescale Timer 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcps1ctl](index.html) module"]
pub struct RTCPS1CTL_SPEC;
impl crate::RegisterSpec for RTCPS1CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcps1ctl::R](R) reader structure"]
impl crate::Readable for RTCPS1CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcps1ctl::W](W) writer structure"]
impl crate::Writable for RTCPS1CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCPS1CTL to value 0"]
impl crate::Resettable for RTCPS1CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
