#[doc = "Register `PDDIR` reader"]
pub struct R(crate::R<PDDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDIR` writer"]
pub struct W(crate::W<PDDIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDIR_SPEC>;
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
impl From<crate::W<PDDIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7DIR` reader - Port 7 Direction"]
pub struct P7DIR_R(crate::FieldReader<u8, u8>);
impl P7DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DIR` writer - Port 7 Direction"]
pub struct P7DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P8DIR` reader - Port 8 Direction"]
pub struct P8DIR_R(crate::FieldReader<u8, u8>);
impl P8DIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DIR` writer - Port 8 Direction"]
pub struct P8DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&self) -> P7DIR_R {
        P7DIR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&self) -> P8DIR_R {
        P8DIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Direction"]
    #[inline(always)]
    pub fn p7dir(&mut self) -> P7DIR_W {
        P7DIR_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Direction"]
    #[inline(always)]
    pub fn p8dir(&mut self) -> P8DIR_W {
        P8DIR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Direction\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pddir](index.html) module"]
pub struct PDDIR_SPEC;
impl crate::RegisterSpec for PDDIR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pddir::R](R) reader structure"]
impl crate::Readable for PDDIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pddir::W](W) writer structure"]
impl crate::Writable for PDDIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDDIR to value 0"]
impl crate::Resettable for PDDIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
