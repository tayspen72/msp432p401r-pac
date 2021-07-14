#[doc = "Register `PESEL0` reader"]
pub struct R(crate::R<PESEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PESEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PESEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PESEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PESEL0` writer"]
pub struct W(crate::W<PESEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PESEL0_SPEC>;
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
impl From<crate::W<PESEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PESEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P9SEL0` reader - Port 9 Select 0"]
pub struct P9SEL0_R(crate::FieldReader<u8, u8>);
impl P9SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        P9SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P9SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P9SEL0` writer - Port 9 Select 0"]
pub struct P9SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P9SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P10SEL0` reader - Port 10 Select 0"]
pub struct P10SEL0_R(crate::FieldReader<u8, u8>);
impl P10SEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        P10SEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P10SEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P10SEL0` writer - Port 10 Select 0"]
pub struct P10SEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> P10SEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 9 Select 0"]
    #[inline(always)]
    pub fn p9sel0(&self) -> P9SEL0_R {
        P9SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 10 Select 0"]
    #[inline(always)]
    pub fn p10sel0(&self) -> P10SEL0_R {
        P10SEL0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 9 Select 0"]
    #[inline(always)]
    pub fn p9sel0(&mut self) -> P9SEL0_W {
        P9SEL0_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 10 Select 0"]
    #[inline(always)]
    pub fn p10sel0(&mut self) -> P10SEL0_W {
        P10SEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port E Select 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pesel0](index.html) module"]
pub struct PESEL0_SPEC;
impl crate::RegisterSpec for PESEL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pesel0::R](R) reader structure"]
impl crate::Readable for PESEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pesel0::W](W) writer structure"]
impl crate::Writable for PESEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PESEL0 to value 0"]
impl crate::Resettable for PESEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
