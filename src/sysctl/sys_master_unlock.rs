#[doc = "Register `SYS_MASTER_UNLOCK` reader"]
pub struct R(crate::R<SYS_MASTER_UNLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_MASTER_UNLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_MASTER_UNLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_MASTER_UNLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_MASTER_UNLOCK` writer"]
pub struct W(crate::W<SYS_MASTER_UNLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_MASTER_UNLOCK_SPEC>;
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
impl From<crate::W<SYS_MASTER_UNLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_MASTER_UNLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UNLKEY` reader - Unlock Key"]
pub struct UNLKEY_R(crate::FieldReader<u16, u16>);
impl UNLKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        UNLKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLKEY` writer - Unlock Key"]
pub struct UNLKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Unlock Key"]
    #[inline(always)]
    pub fn unlkey(&self) -> UNLKEY_R {
        UNLKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlock Key"]
    #[inline(always)]
    pub fn unlkey(&mut self) -> UNLKEY_W {
        UNLKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Unlock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_master_unlock](index.html) module"]
pub struct SYS_MASTER_UNLOCK_SPEC;
impl crate::RegisterSpec for SYS_MASTER_UNLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_master_unlock::R](R) reader structure"]
impl crate::Readable for SYS_MASTER_UNLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_master_unlock::W](W) writer structure"]
impl crate::Writable for SYS_MASTER_UNLOCK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_MASTER_UNLOCK to value 0"]
impl crate::Resettable for SYS_MASTER_UNLOCK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
