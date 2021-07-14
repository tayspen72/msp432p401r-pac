#[doc = "Register `UCAxIV` reader"]
pub struct R(crate::R<UCAXIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "eUSCI_A interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum UCIV_A {
    #[doc = "0: No interrupt pending"]
    UCIV_0 = 0,
    #[doc = "2: Interrupt Source: Receive buffer full; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    UCIV_2 = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG"]
    UCIV_4 = 4,
    #[doc = "6: Interrupt Source: Start bit received; Interrupt Flag: UCSTTIFG"]
    UCIV_6 = 6,
    #[doc = "8: Interrupt Source: Transmit complete; Interrupt Flag: UCTXCPTIFG; Interrupt Priority: Lowest"]
    UCIV_8 = 8,
}
impl From<UCIV_A> for u16 {
    #[inline(always)]
    fn from(variant: UCIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCIV` reader - eUSCI_A interrupt vector value"]
pub struct UCIV_R(crate::FieldReader<u16, UCIV_A>);
impl UCIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        UCIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UCIV_A> {
        match self.bits {
            0 => Some(UCIV_A::UCIV_0),
            2 => Some(UCIV_A::UCIV_2),
            4 => Some(UCIV_A::UCIV_4),
            6 => Some(UCIV_A::UCIV_6),
            8 => Some(UCIV_A::UCIV_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UCIV_0`"]
    #[inline(always)]
    pub fn is_uciv_0(&self) -> bool {
        **self == UCIV_A::UCIV_0
    }
    #[doc = "Checks if the value of the field is `UCIV_2`"]
    #[inline(always)]
    pub fn is_uciv_2(&self) -> bool {
        **self == UCIV_A::UCIV_2
    }
    #[doc = "Checks if the value of the field is `UCIV_4`"]
    #[inline(always)]
    pub fn is_uciv_4(&self) -> bool {
        **self == UCIV_A::UCIV_4
    }
    #[doc = "Checks if the value of the field is `UCIV_6`"]
    #[inline(always)]
    pub fn is_uciv_6(&self) -> bool {
        **self == UCIV_A::UCIV_6
    }
    #[doc = "Checks if the value of the field is `UCIV_8`"]
    #[inline(always)]
    pub fn is_uciv_8(&self) -> bool {
        **self == UCIV_A::UCIV_8
    }
}
impl core::ops::Deref for UCIV_R {
    type Target = crate::FieldReader<u16, UCIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_A interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UCIV_R {
        UCIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "eUSCI_Ax Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_iv](index.html) module"]
pub struct UCAXIV_SPEC;
impl crate::RegisterSpec for UCAXIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_iv::R](R) reader structure"]
impl crate::Readable for UCAXIV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UCAxIV to value 0"]
impl crate::Resettable for UCAXIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
