#[doc = "Register `PDSEL1` reader"]
pub struct R(crate::R<PDSEL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDSEL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDSEL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDSEL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDSEL1` writer"]
pub struct W(crate::W<PDSEL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDSEL1_SPEC>;
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
impl From<crate::W<PDSEL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDSEL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7SEL1` reader - Port 7 Select 1"]
pub struct P7SEL1_R(crate::FieldReader<u8, u8>);
impl P7SEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7SEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7SEL1` writer - Port 7 Select 1"]
pub struct P7SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P7SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P8SEL1` reader - Port 8 Select 1"]
pub struct P8SEL1_R(crate::FieldReader<u8, u8>);
impl P8SEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8SEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8SEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8SEL1` writer - Port 8 Select 1"]
pub struct P8SEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> P8SEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&self) -> P7SEL1_R {
        P7SEL1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&self) -> P8SEL1_R {
        P8SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Select 1"]
    #[inline(always)]
    pub fn p7sel1(&mut self) -> P7SEL1_W {
        P7SEL1_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Select 1"]
    #[inline(always)]
    pub fn p8sel1(&mut self) -> P8SEL1_W {
        P8SEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Select 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdsel1](index.html) module"]
pub struct PDSEL1_SPEC;
impl crate::RegisterSpec for PDSEL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdsel1::R](R) reader structure"]
impl crate::Readable for PDSEL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdsel1::W](W) writer structure"]
impl crate::Writable for PDSEL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDSEL1 to value 0"]
impl crate::Resettable for PDSEL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
