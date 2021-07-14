#[doc = "Register `PMAPCTL` reader"]
pub struct R(crate::R<PMAPCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMAPCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMAPCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMAPCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMAPCTL` writer"]
pub struct W(crate::W<PMAPCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMAPCTL_SPEC>;
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
impl From<crate::W<PMAPCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMAPCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port mapping lock bit\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAPLOCKED_A {
    #[doc = "0: Access to mapping registers is granted"]
    PMAPLOCKED_0 = 0,
    #[doc = "1: Access to mapping registers is locked"]
    PMAPLOCKED_1 = 1,
}
impl From<PMAPLOCKED_A> for bool {
    #[inline(always)]
    fn from(variant: PMAPLOCKED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAPLOCKED` reader - Port mapping lock bit"]
pub struct PMAPLOCKED_R(crate::FieldReader<bool, PMAPLOCKED_A>);
impl PMAPLOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMAPLOCKED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAPLOCKED_A {
        match self.bits {
            false => PMAPLOCKED_A::PMAPLOCKED_0,
            true => PMAPLOCKED_A::PMAPLOCKED_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMAPLOCKED_0`"]
    #[inline(always)]
    pub fn is_pmaplocked_0(&self) -> bool {
        **self == PMAPLOCKED_A::PMAPLOCKED_0
    }
    #[doc = "Checks if the value of the field is `PMAPLOCKED_1`"]
    #[inline(always)]
    pub fn is_pmaplocked_1(&self) -> bool {
        **self == PMAPLOCKED_A::PMAPLOCKED_1
    }
}
impl core::ops::Deref for PMAPLOCKED_R {
    type Target = crate::FieldReader<bool, PMAPLOCKED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Port mapping reconfiguration control bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PMAPRECFG_A {
    #[doc = "0: Configuration allowed only once"]
    PMAPRECFG_0 = 0,
    #[doc = "1: Allow reconfiguration of port mapping"]
    PMAPRECFG_1 = 1,
}
impl From<PMAPRECFG_A> for bool {
    #[inline(always)]
    fn from(variant: PMAPRECFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMAPRECFG` reader - Port mapping reconfiguration control bit"]
pub struct PMAPRECFG_R(crate::FieldReader<bool, PMAPRECFG_A>);
impl PMAPRECFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PMAPRECFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMAPRECFG_A {
        match self.bits {
            false => PMAPRECFG_A::PMAPRECFG_0,
            true => PMAPRECFG_A::PMAPRECFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMAPRECFG_0`"]
    #[inline(always)]
    pub fn is_pmaprecfg_0(&self) -> bool {
        **self == PMAPRECFG_A::PMAPRECFG_0
    }
    #[doc = "Checks if the value of the field is `PMAPRECFG_1`"]
    #[inline(always)]
    pub fn is_pmaprecfg_1(&self) -> bool {
        **self == PMAPRECFG_A::PMAPRECFG_1
    }
}
impl core::ops::Deref for PMAPRECFG_R {
    type Target = crate::FieldReader<bool, PMAPRECFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PMAPRECFG` writer - Port mapping reconfiguration control bit"]
pub struct PMAPRECFG_W<'a> {
    w: &'a mut W,
}
impl<'a> PMAPRECFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PMAPRECFG_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Configuration allowed only once"]
    #[inline(always)]
    pub fn pmaprecfg_0(self) -> &'a mut W {
        self.variant(PMAPRECFG_A::PMAPRECFG_0)
    }
    #[doc = "Allow reconfiguration of port mapping"]
    #[inline(always)]
    pub fn pmaprecfg_1(self) -> &'a mut W {
        self.variant(PMAPRECFG_A::PMAPRECFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u16 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Port mapping lock bit"]
    #[inline(always)]
    pub fn pmaplocked(&self) -> PMAPLOCKED_R {
        PMAPLOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&self) -> PMAPRECFG_R {
        PMAPRECFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Port mapping reconfiguration control bit"]
    #[inline(always)]
    pub fn pmaprecfg(&mut self) -> PMAPRECFG_W {
        PMAPRECFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Mapping Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmapctl](index.html) module"]
pub struct PMAPCTL_SPEC;
impl crate::RegisterSpec for PMAPCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmapctl::R](R) reader structure"]
impl crate::Readable for PMAPCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmapctl::W](W) writer structure"]
impl crate::Writable for PMAPCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMAPCTL to value 0x01"]
impl crate::Resettable for PMAPCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
