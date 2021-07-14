#[doc = "Register `PBIFG` reader"]
pub struct R(crate::R<PBIFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBIFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBIFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBIFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBIFG` writer"]
pub struct W(crate::W<PBIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBIFG_SPEC>;
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
impl From<crate::W<PBIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IFG` reader - Port 3 Interrupt Flag"]
pub struct P3IFG_R(crate::FieldReader<u8, u8>);
impl P3IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IFG` writer - Port 3 Interrupt Flag"]
pub struct P3IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4IFG` reader - Port 4 Interrupt Flag"]
pub struct P4IFG_R(crate::FieldReader<u8, u8>);
impl P4IFG_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4IFG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IFG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IFG` writer - Port 4 Interrupt Flag"]
pub struct P4IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IFG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&self) -> P3IFG_R {
        P3IFG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&self) -> P4IFG_R {
        P4IFG_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Flag"]
    #[inline(always)]
    pub fn p3ifg(&mut self) -> P3IFG_W {
        P3IFG_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Flag"]
    #[inline(always)]
    pub fn p4ifg(&mut self) -> P4IFG_W {
        P4IFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Interrupt Flag\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbifg](index.html) module"]
pub struct PBIFG_SPEC;
impl crate::RegisterSpec for PBIFG_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbifg::R](R) reader structure"]
impl crate::Readable for PBIFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbifg::W](W) writer structure"]
impl crate::Writable for PBIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBIFG to value 0"]
impl crate::Resettable for PBIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
