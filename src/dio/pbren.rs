#[doc = "Register `PBREN` reader"]
pub struct R(crate::R<PBREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBREN` writer"]
pub struct W(crate::W<PBREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBREN_SPEC>;
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
impl From<crate::W<PBREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3REN` reader - Port 3 Resistor Enable"]
pub struct P3REN_R(crate::FieldReader<u8, u8>);
impl P3REN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3REN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3REN` writer - Port 3 Resistor Enable"]
pub struct P3REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P3REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4REN` reader - Port 4 Resistor Enable"]
pub struct P4REN_R(crate::FieldReader<u8, u8>);
impl P4REN_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4REN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4REN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4REN` writer - Port 4 Resistor Enable"]
pub struct P4REN_W<'a> {
    w: &'a mut W,
}
impl<'a> P4REN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&self) -> P3REN_R {
        P3REN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&self) -> P4REN_R {
        P4REN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Resistor Enable"]
    #[inline(always)]
    pub fn p3ren(&mut self) -> P3REN_W {
        P3REN_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Resistor Enable"]
    #[inline(always)]
    pub fn p4ren(&mut self) -> P4REN_W {
        P4REN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Resistor Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbren](index.html) module"]
pub struct PBREN_SPEC;
impl crate::RegisterSpec for PBREN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbren::R](R) reader structure"]
impl crate::Readable for PBREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbren::W](W) writer structure"]
impl crate::Writable for PBREN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PBREN to value 0"]
impl crate::Resettable for PBREN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
