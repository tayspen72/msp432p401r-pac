#[doc = "Register `CRC16DIRB` reader"]
pub struct R(crate::R<CRC16DIRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC16DIRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC16DIRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC16DIRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC16DIRB` writer"]
pub struct W(crate::W<CRC16DIRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC16DIRB_SPEC>;
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
impl From<crate::W<CRC16DIRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC16DIRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC16DIRB` reader - CRC16 data in reverse byte"]
pub struct CRC16DIRB_R(crate::FieldReader<u16, u16>);
impl CRC16DIRB_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRC16DIRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16DIRB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16DIRB` writer - CRC16 data in reverse byte"]
pub struct CRC16DIRB_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16DIRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&self) -> CRC16DIRB_R {
        CRC16DIRB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in reverse byte"]
    #[inline(always)]
    pub fn crc16dirb(&mut self) -> CRC16DIRB_W {
        CRC16DIRB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC16 Data In Reverse\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16dirb](index.html) module"]
pub struct CRC16DIRB_SPEC;
impl crate::RegisterSpec for CRC16DIRB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc16dirb::R](R) reader structure"]
impl crate::Readable for CRC16DIRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc16dirb::W](W) writer structure"]
impl crate::Writable for CRC16DIRB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC16DIRB to value 0"]
impl crate::Resettable for CRC16DIRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
