#[doc = "Register `CRC16DI` reader"]
pub struct R(crate::R<CRC16DI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC16DI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC16DI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC16DI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRC16DI` writer"]
pub struct W(crate::W<CRC16DI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC16DI_SPEC>;
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
impl From<crate::W<CRC16DI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC16DI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRC16DI` reader - CRC16 data in"]
pub struct CRC16DI_R(crate::FieldReader<u16, u16>);
impl CRC16DI_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRC16DI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16DI_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRC16DI` writer - CRC16 data in"]
pub struct CRC16DI_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16DI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u16 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&self) -> CRC16DI_R {
        CRC16DI_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CRC16 data in"]
    #[inline(always)]
    pub fn crc16di(&mut self) -> CRC16DI_W {
        CRC16DI_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Input for CRC16 computation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc16di](index.html) module"]
pub struct CRC16DI_SPEC;
impl crate::RegisterSpec for CRC16DI_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc16di::R](R) reader structure"]
impl crate::Readable for CRC16DI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc16di::W](W) writer structure"]
impl crate::Writable for CRC16DI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRC16DI to value 0"]
impl crate::Resettable for CRC16DI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
