#[doc = "Register `RTCIV` reader"]
pub struct R(crate::R<RTCIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Real-time clock interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum RTCIV_A {
    #[doc = "0: No interrupt pending"]
    RTCIV_0 = 0,
    #[doc = "2: Interrupt Source: RTC oscillator failure; Interrupt Flag: RTCOFIFG; Interrupt Priority: Highest"]
    RTCIV_2 = 2,
    #[doc = "4: Interrupt Source: RTC ready; Interrupt Flag: RTCRDYIFG"]
    RTCIV_4 = 4,
    #[doc = "6: Interrupt Source: RTC interval timer; Interrupt Flag: RTCTEVIFG"]
    RTCIV_6 = 6,
    #[doc = "8: Interrupt Source: RTC user alarm; Interrupt Flag: RTCAIFG"]
    RTCIV_8 = 8,
    #[doc = "10: Interrupt Source: RTC prescaler 0; Interrupt Flag: RT0PSIFG"]
    RTCIV_10 = 10,
    #[doc = "12: Interrupt Source: RTC prescaler 1; Interrupt Flag: RT1PSIFG"]
    RTCIV_12 = 12,
}
impl From<RTCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: RTCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RTCIV` reader - Real-time clock interrupt vector value"]
pub struct RTCIV_R(crate::FieldReader<u16, RTCIV_A>);
impl RTCIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        RTCIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RTCIV_A> {
        match self.bits {
            0 => Some(RTCIV_A::RTCIV_0),
            2 => Some(RTCIV_A::RTCIV_2),
            4 => Some(RTCIV_A::RTCIV_4),
            6 => Some(RTCIV_A::RTCIV_6),
            8 => Some(RTCIV_A::RTCIV_8),
            10 => Some(RTCIV_A::RTCIV_10),
            12 => Some(RTCIV_A::RTCIV_12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RTCIV_0`"]
    #[inline(always)]
    pub fn is_rtciv_0(&self) -> bool {
        **self == RTCIV_A::RTCIV_0
    }
    #[doc = "Checks if the value of the field is `RTCIV_2`"]
    #[inline(always)]
    pub fn is_rtciv_2(&self) -> bool {
        **self == RTCIV_A::RTCIV_2
    }
    #[doc = "Checks if the value of the field is `RTCIV_4`"]
    #[inline(always)]
    pub fn is_rtciv_4(&self) -> bool {
        **self == RTCIV_A::RTCIV_4
    }
    #[doc = "Checks if the value of the field is `RTCIV_6`"]
    #[inline(always)]
    pub fn is_rtciv_6(&self) -> bool {
        **self == RTCIV_A::RTCIV_6
    }
    #[doc = "Checks if the value of the field is `RTCIV_8`"]
    #[inline(always)]
    pub fn is_rtciv_8(&self) -> bool {
        **self == RTCIV_A::RTCIV_8
    }
    #[doc = "Checks if the value of the field is `RTCIV_10`"]
    #[inline(always)]
    pub fn is_rtciv_10(&self) -> bool {
        **self == RTCIV_A::RTCIV_10
    }
    #[doc = "Checks if the value of the field is `RTCIV_12`"]
    #[inline(always)]
    pub fn is_rtciv_12(&self) -> bool {
        **self == RTCIV_A::RTCIV_12
    }
}
impl core::ops::Deref for RTCIV_R {
    type Target = crate::FieldReader<u16, RTCIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Real-time clock interrupt vector value"]
    #[inline(always)]
    pub fn rtciv(&self) -> RTCIV_R {
        RTCIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Real-Time Clock Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtciv](index.html) module"]
pub struct RTCIV_SPEC;
impl crate::RegisterSpec for RTCIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtciv::R](R) reader structure"]
impl crate::Readable for RTCIV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RTCIV to value 0"]
impl crate::Resettable for RTCIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
