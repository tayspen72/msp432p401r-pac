#[doc = "Register `PSSIFG` reader"]
pub struct R(crate::R<PSSIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PSSIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PSSIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PSSIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "High-side SVSM interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVSMHIFG_A {
    #[doc = "0: No interrupt pending"]
    SVSMHIFG_0 = 0,
    #[doc = "1: Interrupt due to SVSMH"]
    SVSMHIFG_1 = 1,
}
impl From<SVSMHIFG_A> for bool {
    #[inline(always)]
    fn from(variant: SVSMHIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SVSMHIFG` reader - High-side SVSM interrupt flag"]
pub struct SVSMHIFG_R(crate::FieldReader<bool, SVSMHIFG_A>);
impl SVSMHIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SVSMHIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVSMHIFG_A {
        match self.bits {
            false => SVSMHIFG_A::SVSMHIFG_0,
            true => SVSMHIFG_A::SVSMHIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_0`"]
    #[inline(always)]
    pub fn is_svsmhifg_0(&self) -> bool {
        **self == SVSMHIFG_A::SVSMHIFG_0
    }
    #[doc = "Checks if the value of the field is `SVSMHIFG_1`"]
    #[inline(always)]
    pub fn is_svsmhifg_1(&self) -> bool {
        **self == SVSMHIFG_A::SVSMHIFG_1
    }
}
impl core::ops::Deref for SVSMHIFG_R {
    type Target = crate::FieldReader<bool, SVSMHIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - High-side SVSM interrupt flag"]
    #[inline(always)]
    pub fn svsmhifg(&self) -> SVSMHIFG_R {
        SVSMHIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pssifg](index.html) module"]
pub struct PSSIFG_SPEC;
impl crate::RegisterSpec for PSSIFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pssifg::R](R) reader structure"]
impl crate::Readable for PSSIFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PSSIFG to value 0"]
impl crate::Resettable for PSSIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
