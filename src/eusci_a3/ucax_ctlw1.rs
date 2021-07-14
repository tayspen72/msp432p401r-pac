#[doc = "Register `UCAxCTLW1` reader"]
pub struct R(crate::R<UCAXCTLW1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCAXCTLW1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCAXCTLW1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCAXCTLW1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCAxCTLW1` writer"]
pub struct W(crate::W<UCAXCTLW1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCAXCTLW1_SPEC>;
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
impl From<crate::W<UCAXCTLW1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCAXCTLW1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Deglitch time\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UCGLIT_A {
    #[doc = "0: Approximately 2 ns (equivalent of 1 delay element)"]
    UCGLIT_0 = 0,
    #[doc = "1: Approximately 50 ns"]
    UCGLIT_1 = 1,
    #[doc = "2: Approximately 100 ns"]
    UCGLIT_2 = 2,
    #[doc = "3: Approximately 200 ns"]
    UCGLIT_3 = 3,
}
impl From<UCGLIT_A> for u8 {
    #[inline(always)]
    fn from(variant: UCGLIT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UCGLIT` reader - Deglitch time"]
pub struct UCGLIT_R(crate::FieldReader<u8, UCGLIT_A>);
impl UCGLIT_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCGLIT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UCGLIT_A {
        match self.bits {
            0 => UCGLIT_A::UCGLIT_0,
            1 => UCGLIT_A::UCGLIT_1,
            2 => UCGLIT_A::UCGLIT_2,
            3 => UCGLIT_A::UCGLIT_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `UCGLIT_0`"]
    #[inline(always)]
    pub fn is_ucglit_0(&self) -> bool {
        **self == UCGLIT_A::UCGLIT_0
    }
    #[doc = "Checks if the value of the field is `UCGLIT_1`"]
    #[inline(always)]
    pub fn is_ucglit_1(&self) -> bool {
        **self == UCGLIT_A::UCGLIT_1
    }
    #[doc = "Checks if the value of the field is `UCGLIT_2`"]
    #[inline(always)]
    pub fn is_ucglit_2(&self) -> bool {
        **self == UCGLIT_A::UCGLIT_2
    }
    #[doc = "Checks if the value of the field is `UCGLIT_3`"]
    #[inline(always)]
    pub fn is_ucglit_3(&self) -> bool {
        **self == UCGLIT_A::UCGLIT_3
    }
}
impl core::ops::Deref for UCGLIT_R {
    type Target = crate::FieldReader<u8, UCGLIT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCGLIT` writer - Deglitch time"]
pub struct UCGLIT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCGLIT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UCGLIT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Approximately 2 ns (equivalent of 1 delay element)"]
    #[inline(always)]
    pub fn ucglit_0(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_0)
    }
    #[doc = "Approximately 50 ns"]
    #[inline(always)]
    pub fn ucglit_1(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_1)
    }
    #[doc = "Approximately 100 ns"]
    #[inline(always)]
    pub fn ucglit_2(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_2)
    }
    #[doc = "Approximately 200 ns"]
    #[inline(always)]
    pub fn ucglit_3(self) -> &'a mut W {
        self.variant(UCGLIT_A::UCGLIT_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u16 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&self) -> UCGLIT_R {
        UCGLIT_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Deglitch time"]
    #[inline(always)]
    pub fn ucglit(&mut self) -> UCGLIT_W {
        UCGLIT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Ax Control Word Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucax_ctlw1](index.html) module"]
pub struct UCAXCTLW1_SPEC;
impl crate::RegisterSpec for UCAXCTLW1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucax_ctlw1::R](R) reader structure"]
impl crate::Readable for UCAXCTLW1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucax_ctlw1::W](W) writer structure"]
impl crate::Writable for UCAXCTLW1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCAxCTLW1 to value 0x03"]
impl crate::Resettable for UCAXCTLW1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
