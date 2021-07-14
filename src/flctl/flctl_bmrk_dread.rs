#[doc = "Register `FLCTL_BMRK_DREAD` reader"]
pub struct R(crate::R<FLCTL_BMRK_DREAD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_BMRK_DREAD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_BMRK_DREAD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_BMRK_DREAD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_BMRK_DREAD` writer"]
pub struct W(crate::W<FLCTL_BMRK_DREAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_BMRK_DREAD_SPEC>;
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
impl From<crate::W<FLCTL_BMRK_DREAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_BMRK_DREAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
pub struct COUNT_R(crate::FieldReader<u32, u32>);
impl COUNT_R {
    pub(crate) fn new(bits: u32) -> Self {
        COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COUNT` writer - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reflects the number of Data Read operations to the Flash (increments by one on each read)"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Benchmark Data Read Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_bmrk_dread](index.html) module"]
pub struct FLCTL_BMRK_DREAD_SPEC;
impl crate::RegisterSpec for FLCTL_BMRK_DREAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_bmrk_dread::R](R) reader structure"]
impl crate::Readable for FLCTL_BMRK_DREAD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_bmrk_dread::W](W) writer structure"]
impl crate::Writable for FLCTL_BMRK_DREAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_BMRK_DREAD to value 0"]
impl crate::Resettable for FLCTL_BMRK_DREAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
