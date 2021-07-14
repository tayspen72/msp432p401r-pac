#[doc = "Register `T32INTCLR1` writer"]
pub struct W(crate::W<T32INTCLR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T32INTCLR1_SPEC>;
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
impl From<crate::W<T32INTCLR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T32INTCLR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTCLR` writer - Write clears interrupt output"]
pub struct INTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> INTCLR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Write clears interrupt output"]
    #[inline(always)]
    pub fn intclr(&mut self) -> INTCLR_W {
        INTCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer 1 Interrupt Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t32intclr1](index.html) module"]
pub struct T32INTCLR1_SPEC;
impl crate::RegisterSpec for T32INTCLR1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t32intclr1::W](W) writer structure"]
impl crate::Writable for T32INTCLR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T32INTCLR1 to value 0"]
impl crate::Resettable for T32INTCLR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
