#[doc = "Register `CExIV` reader"]
pub struct R(crate::R<CEXIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CEXIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CEXIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CEXIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Comparator interrupt vector word register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CEIV_A {
    #[doc = "0: No interrupt pending"]
    CEIV_0 = 0,
    #[doc = "2: Interrupt Source: CEOUT interrupt; Interrupt Flag: CEIFG; Interrupt Priority: Highest"]
    CEIV_2 = 2,
    #[doc = "4: Interrupt Source: CEOUT interrupt inverted polarity; Interrupt Flag: CEIIFG"]
    CEIV_4 = 4,
    #[doc = "10: Interrupt Source: Comparator ready interrupt; Interrupt Flag: CERDYIFG; Interrupt Priority: Lowest"]
    CEIV_10 = 10,
}
impl From<CEIV_A> for u16 {
    #[inline(always)]
    fn from(variant: CEIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CEIV` reader - Comparator interrupt vector word register"]
pub struct CEIV_R(crate::FieldReader<u16, CEIV_A>);
impl CEIV_R {
    pub(crate) fn new(bits: u16) -> Self {
        CEIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CEIV_A> {
        match self.bits {
            0 => Some(CEIV_A::CEIV_0),
            2 => Some(CEIV_A::CEIV_2),
            4 => Some(CEIV_A::CEIV_4),
            10 => Some(CEIV_A::CEIV_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CEIV_0`"]
    #[inline(always)]
    pub fn is_ceiv_0(&self) -> bool {
        **self == CEIV_A::CEIV_0
    }
    #[doc = "Checks if the value of the field is `CEIV_2`"]
    #[inline(always)]
    pub fn is_ceiv_2(&self) -> bool {
        **self == CEIV_A::CEIV_2
    }
    #[doc = "Checks if the value of the field is `CEIV_4`"]
    #[inline(always)]
    pub fn is_ceiv_4(&self) -> bool {
        **self == CEIV_A::CEIV_4
    }
    #[doc = "Checks if the value of the field is `CEIV_10`"]
    #[inline(always)]
    pub fn is_ceiv_10(&self) -> bool {
        **self == CEIV_A::CEIV_10
    }
}
impl core::ops::Deref for CEIV_R {
    type Target = crate::FieldReader<u16, CEIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Comparator interrupt vector word register"]
    #[inline(always)]
    pub fn ceiv(&self) -> CEIV_R {
        CEIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Comparator Interrupt Vector Word Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cex_iv](index.html) module"]
pub struct CEXIV_SPEC;
impl crate::RegisterSpec for CEXIV_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cex_iv::R](R) reader structure"]
impl crate::Readable for CEXIV_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CExIV to value 0"]
impl crate::Resettable for CEXIV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
