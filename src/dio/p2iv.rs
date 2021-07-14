#[doc = "Register `P2IV` reader"]
pub struct R(crate::R<P2IV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Port 2 interrupt vector value"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum P2IV_A {
    #[doc = "0: No interrupt pending"]
    P2IV_0 = 0,
    #[doc = "2: Interrupt Source: Port 2.0 interrupt; Interrupt Flag: P2IFG0; Interrupt Priority: Highest"]
    P2IV_2 = 2,
    #[doc = "4: Interrupt Source: Port 2.1 interrupt; Interrupt Flag: P2IFG1"]
    P2IV_4 = 4,
    #[doc = "6: Interrupt Source: Port 2.2 interrupt; Interrupt Flag: P2IFG2"]
    P2IV_6 = 6,
    #[doc = "8: Interrupt Source: Port 2.3 interrupt; Interrupt Flag: P2IFG3"]
    P2IV_8 = 8,
    #[doc = "10: Interrupt Source: Port 2.4 interrupt; Interrupt Flag: P2IFG4"]
    P2IV_10 = 10,
    #[doc = "12: Interrupt Source: Port 2.5 interrupt; Interrupt Flag: P2IFG5"]
    P2IV_12 = 12,
    #[doc = "14: Interrupt Source: Port 2.6 interrupt; Interrupt Flag: P2IFG6"]
    P2IV_14 = 14,
    #[doc = "16: Interrupt Source: Port 2.7 interrupt; Interrupt Flag: P2IFG7; Interrupt Priority: Lowest"]
    P2IV_16 = 16,
}
impl From<P2IV_A> for u8 {
    #[inline(always)]
    fn from(variant: P2IV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `P2IV` reader - Port 2 interrupt vector value"]
pub struct P2IV_R(crate::FieldReader<u8, P2IV_A>);
impl P2IV_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2IV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<P2IV_A> {
        match self.bits {
            0 => Some(P2IV_A::P2IV_0),
            2 => Some(P2IV_A::P2IV_2),
            4 => Some(P2IV_A::P2IV_4),
            6 => Some(P2IV_A::P2IV_6),
            8 => Some(P2IV_A::P2IV_8),
            10 => Some(P2IV_A::P2IV_10),
            12 => Some(P2IV_A::P2IV_12),
            14 => Some(P2IV_A::P2IV_14),
            16 => Some(P2IV_A::P2IV_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `P2IV_0`"]
    #[inline(always)]
    pub fn is_p2iv_0(&self) -> bool {
        **self == P2IV_A::P2IV_0
    }
    #[doc = "Checks if the value of the field is `P2IV_2`"]
    #[inline(always)]
    pub fn is_p2iv_2(&self) -> bool {
        **self == P2IV_A::P2IV_2
    }
    #[doc = "Checks if the value of the field is `P2IV_4`"]
    #[inline(always)]
    pub fn is_p2iv_4(&self) -> bool {
        **self == P2IV_A::P2IV_4
    }
    #[doc = "Checks if the value of the field is `P2IV_6`"]
    #[inline(always)]
    pub fn is_p2iv_6(&self) -> bool {
        **self == P2IV_A::P2IV_6
    }
    #[doc = "Checks if the value of the field is `P2IV_8`"]
    #[inline(always)]
    pub fn is_p2iv_8(&self) -> bool {
        **self == P2IV_A::P2IV_8
    }
    #[doc = "Checks if the value of the field is `P2IV_10`"]
    #[inline(always)]
    pub fn is_p2iv_10(&self) -> bool {
        **self == P2IV_A::P2IV_10
    }
    #[doc = "Checks if the value of the field is `P2IV_12`"]
    #[inline(always)]
    pub fn is_p2iv_12(&self) -> bool {
        **self == P2IV_A::P2IV_12
    }
    #[doc = "Checks if the value of the field is `P2IV_14`"]
    #[inline(always)]
    pub fn is_p2iv_14(&self) -> bool {
        **self == P2IV_A::P2IV_14
    }
    #[doc = "Checks if the value of the field is `P2IV_16`"]
    #[inline(always)]
    pub fn is_p2iv_16(&self) -> bool {
        **self == P2IV_A::P2IV_16
    }
}
impl core::ops::Deref for P2IV_R {
    type Target = crate::FieldReader<u8, P2IV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:4 - Port 2 interrupt vector value"]
    #[inline(always)]
    pub fn p2iv(&self) -> P2IV_R {
        P2IV_R::new((self.bits & 0x1f) as u8)
    }
}
#[doc = "Port 2 Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2iv](index.html) module"]
pub struct P2IV_SPEC;
impl crate::RegisterSpec for P2IV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [p2iv::R](R) reader structure"]
impl crate::Readable for P2IV_SPEC {
    type Reader = R;
}
