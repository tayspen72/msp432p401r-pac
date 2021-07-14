#[doc = "Register `AESAXIN` writer"]
pub struct W(crate::W<AESAXIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AESAXIN_SPEC>;
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
impl From<crate::W<AESAXIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AESAXIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AESXIN0x` writer - AES data in byte n when AESAXIN is written as half-word"]
pub struct AESXIN0X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN0X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u16 & 0xff);
        self.w
    }
}
#[doc = "Field `AESXIN1x` writer - AES data in byte n+1 when AESAXIN is written as half-word"]
pub struct AESXIN1X_W<'a> {
    w: &'a mut W,
}
impl<'a> AESXIN1X_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u16 & 0xff) << 8);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - AES data in byte n when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin0x(&mut self) -> AESXIN0X_W {
        AESXIN0X_W { w: self }
    }
    #[doc = "Bits 8:15 - AES data in byte n+1 when AESAXIN is written as half-word"]
    #[inline(always)]
    pub fn aesxin1x(&mut self) -> AESXIN1X_W {
        AESXIN1X_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AES Accelerator XORed Data In Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aesaxin](index.html) module"]
pub struct AESAXIN_SPEC;
impl crate::RegisterSpec for AESAXIN_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [aesaxin::W](W) writer structure"]
impl crate::Writable for AESAXIN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AESAXIN to value 0"]
impl crate::Resettable for AESAXIN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
