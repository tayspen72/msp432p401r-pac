#[doc = "Register `PBOUT` reader"]
pub struct R(crate::R<PBOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBOUT` writer"]
pub struct W(crate::W<PBOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBOUT_SPEC>;
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
impl From<crate::W<PBOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3OUT` reader - Port 3 Output"]
pub struct P3OUT_R(crate::FieldReader<u8, u8>);
impl P3OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3OUT` writer - Port 3 Output"]
pub struct P3OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P3OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4OUT` reader - Port 4 Output"]
pub struct P4OUT_R(crate::FieldReader<u8, u8>);
impl P4OUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4OUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4OUT` writer - Port 4 Output"]
pub struct P4OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> P4OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&self) -> P3OUT_R {
        P3OUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&self) -> P4OUT_R {
        P4OUT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Output"]
    #[inline(always)]
    pub fn p3out(&mut self) -> P3OUT_W {
        P3OUT_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Output"]
    #[inline(always)]
    pub fn p4out(&mut self) -> P4OUT_W {
        P4OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Output\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbout](index.html) module"]
pub struct PBOUT_SPEC;
impl crate::RegisterSpec for PBOUT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbout::R](R) reader structure"]
impl crate::Readable for PBOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbout::W](W) writer structure"]
impl crate::Writable for PBOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBOUT to value 0"]
impl crate::Resettable for PBOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
