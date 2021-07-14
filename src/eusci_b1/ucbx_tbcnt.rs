#[doc = "Register `UCBxTBCNT` reader"]
pub struct R(crate::R<UCBXTBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXTBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXTBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXTBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxTBCNT` writer"]
pub struct W(crate::W<UCBXTBCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXTBCNT_SPEC>;
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
impl From<crate::W<UCBXTBCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXTBCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UCTBCNT` reader - Byte counter threshold value"]
pub struct UCTBCNT_R(crate::FieldReader<u8, u8>);
impl UCTBCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        UCTBCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UCTBCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UCTBCNT` writer - Byte counter threshold value"]
pub struct UCTBCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UCTBCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&self) -> UCTBCNT_R {
        UCTBCNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Byte counter threshold value"]
    #[inline(always)]
    pub fn uctbcnt(&mut self) -> UCTBCNT_W {
        UCTBCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx Byte Counter Threshold Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_tbcnt](index.html) module"]
pub struct UCBXTBCNT_SPEC;
impl crate::RegisterSpec for UCBXTBCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_tbcnt::R](R) reader structure"]
impl crate::Readable for UCBXTBCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_tbcnt::W](W) writer structure"]
impl crate::Writable for UCBXTBCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxTBCNT to value 0"]
impl crate::Resettable for UCBXTBCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
