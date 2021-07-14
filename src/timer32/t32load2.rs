#[doc = "Register `T32LOAD2` reader"]
pub struct R(crate::R<T32LOAD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T32LOAD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T32LOAD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T32LOAD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T32LOAD2` writer"]
pub struct W(crate::W<T32LOAD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32LOAD2_SPEC>;
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
impl From<crate::W<T32LOAD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32LOAD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - The value from which the Timer 2 counter decrements"]
pub struct LOAD_R(crate::FieldReader<u32, u32>);
impl LOAD_R {
    pub(crate) fn new(bits: u32) -> Self {
        LOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOAD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOAD` writer - The value from which the Timer 2 counter decrements"]
pub struct LOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> LOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The value from which the Timer 2 counter decrements"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value from which the Timer 2 counter decrements"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 2 Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32load2](index.html) module"]
pub struct T32LOAD2_SPEC;
impl crate::RegisterSpec for T32LOAD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t32load2::R](R) reader structure"]
impl crate::Readable for T32LOAD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t32load2::W](W) writer structure"]
impl crate::Writable for T32LOAD2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32LOAD2 to value 0"]
impl crate::Resettable for T32LOAD2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
