#[doc = "Register `PADS` reader"]
pub struct R(crate::R<PADS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADS` writer"]
pub struct W(crate::W<PADS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADS_SPEC>;
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
impl From<crate::W<PADS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DS` reader - Port 1 Drive Strength"]
pub struct P1DS_R(crate::FieldReader<u8, u8>);
impl P1DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P1DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P1DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P1DS` writer - Port 1 Drive Strength"]
pub struct P1DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P1DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P2DS` reader - Port 2 Drive Strength"]
pub struct P2DS_R(crate::FieldReader<u8, u8>);
impl P2DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P2DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2DS` writer - Port 2 Drive Strength"]
pub struct P2DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P2DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&self) -> P1DS_R {
        P1DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&self) -> P2DS_R {
        P2DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 1 Drive Strength"]
    #[inline(always)]
    pub fn p1ds(&mut self) -> P1DS_W {
        P1DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 2 Drive Strength"]
    #[inline(always)]
    pub fn p2ds(&mut self) -> P2DS_W {
        P2DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port A Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pads](index.html) module"]
pub struct PADS_SPEC;
impl crate::RegisterSpec for PADS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pads::R](R) reader structure"]
impl crate::Readable for PADS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pads::W](W) writer structure"]
impl crate::Writable for PADS_SPEC {
    type Writer = W;
}
