#[doc = "Register `P3IV` reader"]
pub struct R(crate::R<P3IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P3IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P3IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P3IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port 3 interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P3IV_A {
    #[doc = "0: No interrupt pending"]
    P3IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 3.0 interrupt; Interrupt Flag: P3IFG0; Interrupt Priority: Highest"]
    P3IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 3.1 interrupt; Interrupt Flag: P3IFG1"]
    P3IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 3.2 interrupt; Interrupt Flag: P3IFG2"]
    P3IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 3.3 interrupt; Interrupt Flag: P3IFG3"]
    P3IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 3.4 interrupt; Interrupt Flag: P3IFG4"]
    P3IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 3.5 interrupt; Interrupt Flag: P3IFG5"]
    P3IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 3.6 interrupt; Interrupt Flag: P3IFG6"]
    P3IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 3.7 interrupt; Interrupt Flag: P3IFG7; Interrupt Priority: Lowest"]
    P3IV_16 = 16,
}
impl From<P3IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P3IV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `P3IV` reader - Port 3 interrupt vector value"]
pub struct P3IV_R(crate::FieldReader<u8, P3IV_A>);
impl P3IV_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3IV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P3IV_A> {
        match self.bits {
            0 => Some(P3IV_A::P3IV_0),
            2 => Some(P3IV_A::P3IV_2),
            4 => Some(P3IV_A::P3IV_4),
            6 => Some(P3IV_A::P3IV_6),
            8 => Some(P3IV_A::P3IV_8),
            10 => Some(P3IV_A::P3IV_10),
            12 => Some(P3IV_A::P3IV_12),
            14 => Some(P3IV_A::P3IV_14),
            16 => Some(P3IV_A::P3IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P3IV_0`"]
    #[inline(always)]
    pub fn is_p3iv_0(&self) -> bool {
        **self == P3IV_A::P3IV_0
    }
    #[doc = "Checks if the value of the field is `P3IV_2`"]
    #[inline(always)]
    pub fn is_p3iv_2(&self) -> bool {
        **self == P3IV_A::P3IV_2
    }
    #[doc = "Checks if the value of the field is `P3IV_4`"]
    #[inline(always)]
    pub fn is_p3iv_4(&self) -> bool {
        **self == P3IV_A::P3IV_4
    }
    #[doc = "Checks if the value of the field is `P3IV_6`"]
    #[inline(always)]
    pub fn is_p3iv_6(&self) -> bool {
        **self == P3IV_A::P3IV_6
    }
    #[doc = "Checks if the value of the field is `P3IV_8`"]
    #[inline(always)]
    pub fn is_p3iv_8(&self) -> bool {
        **self == P3IV_A::P3IV_8
    }
    #[doc = "Checks if the value of the field is `P3IV_10`"]
    #[inline(always)]
    pub fn is_p3iv_10(&self) -> bool {
        **self == P3IV_A::P3IV_10
    }
    #[doc = "Checks if the value of the field is `P3IV_12`"]
    #[inline(always)]
    pub fn is_p3iv_12(&self) -> bool {
        **self == P3IV_A::P3IV_12
    }
    #[doc = "Checks if the value of the field is `P3IV_14`"]
    #[inline(always)]
    pub fn is_p3iv_14(&self) -> bool {
        **self == P3IV_A::P3IV_14
    }
    #[doc = "Checks if the value of the field is `P3IV_16`"]
    #[inline(always)]
    pub fn is_p3iv_16(&self) -> bool {
        **self == P3IV_A::P3IV_16
    }
}
impl core::ops::Deref for P3IV_R {
    type Target = crate::FieldReader<u8, P3IV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 3 interrupt vector value"]
    #[inline(always)]
    pub fn p3iv(&self) -> P3IV_R {
        P3IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 3 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p3iv](index.html) module"]
pub struct P3IV_SPEC;
impl crate::RegisterSpec for P3IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p3iv::R](R) reader structure"]
impl crate::Readable for P3IV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets P3IV to value 0"]
impl crate::Resettable for P3IV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
