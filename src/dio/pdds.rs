#[doc = "Register `PDDS` reader"]
pub struct R(crate::R<PDDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDDS` writer"]
pub struct W(crate::W<PDDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDDS_SPEC>;
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
impl From<crate::W<PDDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P7DS` reader - Port 7 Drive Strength"]
pub struct P7DS_R(crate::FieldReader<u8, u8>);
impl P7DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P7DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P7DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P7DS` writer - Port 7 Drive Strength"]
pub struct P7DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P7DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P8DS` reader - Port 8 Drive Strength"]
pub struct P8DS_R(crate::FieldReader<u8, u8>);
impl P8DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P8DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P8DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P8DS` writer - Port 8 Drive Strength"]
pub struct P8DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P8DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&self) -> P7DS_R {
        P7DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&self) -> P8DS_R {
        P8DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 7 Drive Strength"]
    #[inline(always)]
    pub fn p7ds(&mut self) -> P7DS_W {
        P7DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 8 Drive Strength"]
    #[inline(always)]
    pub fn p8ds(&mut self) -> P8DS_W {
        P8DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port D Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdds](index.html) module"]
pub struct PDDS_SPEC;
impl crate::RegisterSpec for PDDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pdds::R](R) reader structure"]
impl crate::Readable for PDDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdds::W](W) writer structure"]
impl crate::Writable for PDDS_SPEC {
    type Writer = W;
}
