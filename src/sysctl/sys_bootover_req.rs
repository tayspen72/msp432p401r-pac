#[doc = "Register `SYS_BOOTOVER_REQ[%s]` reader"]
pub struct R(crate::R<SYS_BOOTOVER_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_BOOTOVER_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_BOOTOVER_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_BOOTOVER_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_BOOTOVER_REQ[%s]` writer"]
pub struct W(crate::W<SYS_BOOTOVER_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_BOOTOVER_REQ_SPEC>;
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
impl From<crate::W<SYS_BOOTOVER_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_BOOTOVER_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGVAL` reader - Value set by debugger, read and clear by the CPU"]
pub struct REGVAL_R(crate::FieldReader<u32, u32>);
impl REGVAL_R {
    pub(crate) fn new(bits: u32) -> Self {
        REGVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REGVAL_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REGVAL` writer - Value set by debugger, read and clear by the CPU"]
pub struct REGVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> REGVAL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Value set by debugger, read and clear by the CPU"]
    #[inline(always)]
    pub fn regval(&self) -> REGVAL_R {
        REGVAL_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Value set by debugger, read and clear by the CPU"]
    #[inline(always)]
    pub fn regval(&mut self) -> REGVAL_W {
        REGVAL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot Override Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_bootover_req](index.html) module"]
pub struct SYS_BOOTOVER_REQ_SPEC;
impl crate::RegisterSpec for SYS_BOOTOVER_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_bootover_req::R](R) reader structure"]
impl crate::Readable for SYS_BOOTOVER_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_bootover_req::W](W) writer structure"]
impl crate::Writable for SYS_BOOTOVER_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_BOOTOVER_REQ[%s]
to value 0"]
impl crate::Resettable for SYS_BOOTOVER_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
