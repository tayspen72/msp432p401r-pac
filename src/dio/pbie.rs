#[doc = "Register `PBIE` reader"]
pub struct R(crate::R<PBIE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBIE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBIE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBIE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBIE` writer"]
pub struct W(crate::W<PBIE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBIE_SPEC>;
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
impl From<crate::W<PBIE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBIE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3IE` reader - Port 3 Interrupt Enable"]
pub struct P3IE_R(crate::FieldReader<u8, u8>);
impl P3IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3IE` writer - Port 3 Interrupt Enable"]
pub struct P3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P3IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4IE` reader - Port 4 Interrupt Enable"]
pub struct P4IE_R(crate::FieldReader<u8, u8>);
impl P4IE_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4IE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4IE` writer - Port 4 Interrupt Enable"]
pub struct P4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> P4IE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&self) -> P3IE_R {
        P3IE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&self) -> P4IE_R {
        P4IE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Interrupt Enable"]
    #[inline(always)]
    pub fn p3ie(&mut self) -> P3IE_W {
        P3IE_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Interrupt Enable"]
    #[inline(always)]
    pub fn p4ie(&mut self) -> P4IE_W {
        P4IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbie](index.html) module"]
pub struct PBIE_SPEC;
impl crate::RegisterSpec for PBIE_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbie::R](R) reader structure"]
impl crate::Readable for PBIE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbie::W](W) writer structure"]
impl crate::Writable for PBIE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBIE to value 0"]
impl crate::Resettable for PBIE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
