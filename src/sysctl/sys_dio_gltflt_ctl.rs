#[doc = "Register `SYS_DIO_GLTFLT_CTL` reader"]
pub struct R(crate::R<SYS_DIO_GLTFLT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_DIO_GLTFLT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_DIO_GLTFLT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_DIO_GLTFLT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_DIO_GLTFLT_CTL` writer"]
pub struct W(crate::W<SYS_DIO_GLTFLT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_DIO_GLTFLT_CTL_SPEC>;
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
impl From<crate::W<SYS_DIO_GLTFLT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_DIO_GLTFLT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Glitch filter enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GLTCH_EN_A {
    #[doc = "0: Disables glitch filter on the digital I/Os"]
    GLTCH_EN_0 = 0,
    #[doc = "1: Enables glitch filter on the digital I/Os"]
    GLTCH_EN_1 = 1,
}
impl From<GLTCH_EN_A> for bool {
    #[inline(always)]
    fn from(variant: GLTCH_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GLTCH_EN` reader - Glitch filter enable"]
pub struct GLTCH_EN_R(crate::FieldReader<bool, GLTCH_EN_A>);
impl GLTCH_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        GLTCH_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GLTCH_EN_A {
        match self.bits {
            false => GLTCH_EN_A::GLTCH_EN_0,
            true => GLTCH_EN_A::GLTCH_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GLTCH_EN_0`"]
    #[inline(always)]
    pub fn is_gltch_en_0(&self) -> bool {
        **self == GLTCH_EN_A::GLTCH_EN_0
    }
    #[doc = "Checks if the value of the field is `GLTCH_EN_1`"]
    #[inline(always)]
    pub fn is_gltch_en_1(&self) -> bool {
        **self == GLTCH_EN_A::GLTCH_EN_1
    }
}
impl core::ops::Deref for GLTCH_EN_R {
    type Target = crate::FieldReader<bool, GLTCH_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GLTCH_EN` writer - Glitch filter enable"]
pub struct GLTCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> GLTCH_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: GLTCH_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_0(self) -> &'a mut W {
        self.variant(GLTCH_EN_A::GLTCH_EN_0)
    }
    #[doc = "Enables glitch filter on the digital I/Os"]
    #[inline(always)]
    pub fn gltch_en_1(self) -> &'a mut W {
        self.variant(GLTCH_EN_A::GLTCH_EN_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&self) -> GLTCH_EN_R {
        GLTCH_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Glitch filter enable"]
    #[inline(always)]
    pub fn gltch_en(&mut self) -> GLTCH_EN_W {
        GLTCH_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital I/O Glitch Filter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_dio_gltflt_ctl](index.html) module"]
pub struct SYS_DIO_GLTFLT_CTL_SPEC;
impl crate::RegisterSpec for SYS_DIO_GLTFLT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_dio_gltflt_ctl::R](R) reader structure"]
impl crate::Readable for SYS_DIO_GLTFLT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_dio_gltflt_ctl::W](W) writer structure"]
impl crate::Writable for SYS_DIO_GLTFLT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_DIO_GLTFLT_CTL to value 0x01"]
impl crate::Resettable for SYS_DIO_GLTFLT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
