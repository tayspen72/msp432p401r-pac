#[doc = "Register `PBDS` reader"]
pub struct R(crate::R<PBDS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PBDS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PBDS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PBDS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PBDS` writer"]
pub struct W(crate::W<PBDS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PBDS_SPEC>;
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
impl From<crate::W<PBDS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PBDS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P3DS` reader - Port 3 Drive Strength"]
pub struct P3DS_R(crate::FieldReader<u8, u8>);
impl P3DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P3DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P3DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P3DS` writer - Port 3 Drive Strength"]
pub struct P3DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P3DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `P4DS` reader - Port 4 Drive Strength"]
pub struct P4DS_R(crate::FieldReader<u8, u8>);
impl P4DS_R {
    pub(crate) fn new(bits: u8) -> Self {
        P4DS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P4DS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P4DS` writer - Port 4 Drive Strength"]
pub struct P4DS_W<'a> {
    w: &'a mut W,
}
impl<'a> P4DS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&self) -> P3DS_R {
        P3DS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&self) -> P4DS_R {
        P4DS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port 3 Drive Strength"]
    #[inline(always)]
    pub fn p3ds(&mut self) -> P3DS_W {
        P3DS_W { w: self }
    }
    #[doc = "Bits 8:15 - Port 4 Drive Strength"]
    #[inline(always)]
    pub fn p4ds(&mut self) -> P4DS_W {
        P4DS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port B Drive Strength\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pbds](index.html) module"]
pub struct PBDS_SPEC;
impl crate::RegisterSpec for PBDS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pbds::R](R) reader structure"]
impl crate::Readable for PBDS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pbds::W](W) writer structure"]
impl crate::Writable for PBDS_SPEC {
    type Writer = W;
}
