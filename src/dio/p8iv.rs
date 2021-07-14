#[doc = "Register `P8IV` reader"]
pub struct R(crate::R<P8IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P8IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P8IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P8IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port 8 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P8IV_A {
    #[doc = "0: No interrupt pending"]
    P8IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 8.0 interrupt; Interrupt Flag: P8IFG0; Interrupt Priority: Highest"]
    P8IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 8.1 interrupt; Interrupt Flag: P8IFG1"]
    P8IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 8.2 interrupt; Interrupt Flag: P8IFG2"]
    P8IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 8.3 interrupt; Interrupt Flag: P8IFG3"]
    P8IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 8.4 interrupt; Interrupt Flag: P8IFG4"]
    P8IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 8.5 interrupt; Interrupt Flag: P8IFG5"]
    P8IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 8.6 interrupt; Interrupt Flag: P8IFG6"]
    P8IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 8.7 interrupt; Interrupt Flag: P8IFG7; Interrupt Priority: Lowest"]
    P8IV_16 = 16,
}
impl From<P8IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P8IV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `P8IV` reader - Port 8 interrupt vector value"]
pub struct P8IV_R(crate::FieldReader<u8, P8IV_A>);
impl P8IV_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8IV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P8IV_A> {
        match self.bits {
            0 => Some(P8IV_A::P8IV_0),
            2 => Some(P8IV_A::P8IV_2),
            4 => Some(P8IV_A::P8IV_4),
            6 => Some(P8IV_A::P8IV_6),
            8 => Some(P8IV_A::P8IV_8),
            10 => Some(P8IV_A::P8IV_10),
            12 => Some(P8IV_A::P8IV_12),
            14 => Some(P8IV_A::P8IV_14),
            16 => Some(P8IV_A::P8IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P8IV_0`"]
    #[inline(always)]
    pub fn is_p8iv_0(&self) -> bool {
        **self == P8IV_A::P8IV_0
    }
    #[doc = "Checks if the value of the field is `P8IV_2`"]
    #[inline(always)]
    pub fn is_p8iv_2(&self) -> bool {
        **self == P8IV_A::P8IV_2
    }
    #[doc = "Checks if the value of the field is `P8IV_4`"]
    #[inline(always)]
    pub fn is_p8iv_4(&self) -> bool {
        **self == P8IV_A::P8IV_4
    }
    #[doc = "Checks if the value of the field is `P8IV_6`"]
    #[inline(always)]
    pub fn is_p8iv_6(&self) -> bool {
        **self == P8IV_A::P8IV_6
    }
    #[doc = "Checks if the value of the field is `P8IV_8`"]
    #[inline(always)]
    pub fn is_p8iv_8(&self) -> bool {
        **self == P8IV_A::P8IV_8
    }
    #[doc = "Checks if the value of the field is `P8IV_10`"]
    #[inline(always)]
    pub fn is_p8iv_10(&self) -> bool {
        **self == P8IV_A::P8IV_10
    }
    #[doc = "Checks if the value of the field is `P8IV_12`"]
    #[inline(always)]
    pub fn is_p8iv_12(&self) -> bool {
        **self == P8IV_A::P8IV_12
    }
    #[doc = "Checks if the value of the field is `P8IV_14`"]
    #[inline(always)]
    pub fn is_p8iv_14(&self) -> bool {
        **self == P8IV_A::P8IV_14
    }
    #[doc = "Checks if the value of the field is `P8IV_16`"]
    #[inline(always)]
    pub fn is_p8iv_16(&self) -> bool {
        **self == P8IV_A::P8IV_16
    }
}
impl core::ops::Deref for P8IV_R {
    type Target = crate::FieldReader<u8, P8IV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 8 interrupt vector value"]
    #[inline(always)]
    pub fn p8iv(&self) -> P8IV_R {
        P8IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 8 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p8iv](index.html) module"]
pub struct P8IV_SPEC;
impl crate::RegisterSpec for P8IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p8iv::R](R) reader structure"]
impl crate::Readable for P8IV_SPEC {
    type Reader = R;
}
