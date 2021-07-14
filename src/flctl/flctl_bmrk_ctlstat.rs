#[doc = "Register `FLCTL_BMRK_CTLSTAT` reader"]
pub struct R(crate::R<FLCTL_BMRK_CTLSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BMRK_CTLSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BMRK_CTLSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BMRK_CTLSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BMRK_CTLSTAT` writer"]
pub struct W(crate::W<FLCTL_BMRK_CTLSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BMRK_CTLSTAT_SPEC>;
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
impl From<crate::W<FLCTL_BMRK_CTLSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BMRK_CTLSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `I_BMRK` reader - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub struct I_BMRK_R(crate::FieldReader<bool, bool>);
impl I_BMRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        I_BMRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I_BMRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I_BMRK` writer - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
pub struct I_BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> I_BMRK_W<'a> {
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
#[doc = "Field `D_BMRK` reader - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub struct D_BMRK_R(crate::FieldReader<bool, bool>);
impl D_BMRK_R {
    pub(crate) fn new(bits: bool) -> Self {
        D_BMRK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for D_BMRK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `D_BMRK` writer - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
pub struct D_BMRK_W<'a> {
    w: &'a mut W,
}
impl<'a> D_BMRK_W<'a> {
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
#[doc = "Field `CMP_EN` reader - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub struct CMP_EN_R(crate::FieldReader<bool, bool>);
impl CMP_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_EN` writer - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
pub struct CMP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Selects which benchmark register should be compared against the threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMP_SEL_A {
    #[doc = "0: Compares the Instruction Benchmark Register against the threshold value"]
    EN_1_0X0 = 0,
    #[doc = "1: Compares the Data Benchmark Register against the threshold value"]
    EN_2_0X1 = 1,
}
impl From<CMP_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMP_SEL` reader - Selects which benchmark register should be compared against the threshold"]
pub struct CMP_SEL_R(crate::FieldReader<bool, CMP_SEL_A>);
impl CMP_SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_SEL_A {
        match self.bits {
            false => CMP_SEL_A::EN_1_0X0,
            true => CMP_SEL_A::EN_2_0X1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_1_0X0`"]
    #[inline(always)]
    pub fn is_en_1_0x0(&self) -> bool {
        **self == CMP_SEL_A::EN_1_0X0
    }
    #[doc = "Checks if the value of the field is `EN_2_0X1`"]
    #[inline(always)]
    pub fn is_en_2_0x1(&self) -> bool {
        **self == CMP_SEL_A::EN_2_0X1
    }
}
impl core::ops::Deref for CMP_SEL_R {
    type Target = crate::FieldReader<bool, CMP_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP_SEL` writer - Selects which benchmark register should be compared against the threshold"]
pub struct CMP_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMP_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMP_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Compares the Instruction Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_1_0x0(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_1_0X0)
    }
    #[doc = "Compares the Data Benchmark Register against the threshold value"]
    #[inline(always)]
    pub fn en_2_0x1(self) -> &'a mut W {
        self.variant(CMP_SEL_A::EN_2_0X1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&self) -> I_BMRK_R {
        I_BMRK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&self) -> D_BMRK_R {
        D_BMRK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&self) -> CMP_EN_R {
        CMP_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&self) -> CMP_SEL_R {
        CMP_SEL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, increments the Instruction Benchmark count register on each instruction fetch to the Flash"]
    #[inline(always)]
    pub fn i_bmrk(&mut self) -> I_BMRK_W {
        I_BMRK_W { w: self }
    }
    #[doc = "Bit 1 - When 1, increments the Data Benchmark count register on each data read access to the Flash"]
    #[inline(always)]
    pub fn d_bmrk(&mut self) -> D_BMRK_W {
        D_BMRK_W { w: self }
    }
    #[doc = "Bit 2 - When 1, enables comparison of the Instruction or Data Benchmark Registers against the threshold value"]
    #[inline(always)]
    pub fn cmp_en(&mut self) -> CMP_EN_W {
        CMP_EN_W { w: self }
    }
    #[doc = "Bit 3 - Selects which benchmark register should be compared against the threshold"]
    #[inline(always)]
    pub fn cmp_sel(&mut self) -> CMP_SEL_W {
        CMP_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Benchmark Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_ctlstat](index.html) module"]
pub struct FLCTL_BMRK_CTLSTAT_SPEC;
impl crate::RegisterSpec for FLCTL_BMRK_CTLSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bmrk_ctlstat::R](R) reader structure"]
impl crate::Readable for FLCTL_BMRK_CTLSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_ctlstat::W](W) writer structure"]
impl crate::Writable for FLCTL_BMRK_CTLSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BMRK_CTLSTAT to value 0"]
impl crate::Resettable for FLCTL_BMRK_CTLSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
