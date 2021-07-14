#[doc = "Register `FLCTL_PRGBRST_DATA2_0` reader"]
pub struct R(crate::R<FLCTL_PRGBRST_DATA2_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_PRGBRST_DATA2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_PRGBRST_DATA2_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_PRGBRST_DATA2_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLCTL_PRGBRST_DATA2_0` writer"]
pub struct W(crate::W<FLCTL_PRGBRST_DATA2_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLCTL_PRGBRST_DATA2_0_SPEC>;
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
impl From<crate::W<FLCTL_PRGBRST_DATA2_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLCTL_PRGBRST_DATA2_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAIN` reader - Program Burst 128 bit Data Word 2"]
pub struct DATAIN_R(crate::FieldReader<u32, u32>);
impl DATAIN_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATAIN_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATAIN` writer - Program Burst 128 bit Data Word 2"]
pub struct DATAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DATAIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Program Burst 128 bit Data Word 2"]
    #[inline(always)]
    pub fn datain(&self) -> DATAIN_R {
        DATAIN_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program Burst 128 bit Data Word 2"]
    #[inline(always)]
    pub fn datain(&mut self) -> DATAIN_W {
        DATAIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Burst Data2 Register0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_prgbrst_data2_0](index.html) module"]
pub struct FLCTL_PRGBRST_DATA2_0_SPEC;
impl crate::RegisterSpec for FLCTL_PRGBRST_DATA2_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_prgbrst_data2_0::R](R) reader structure"]
impl crate::Readable for FLCTL_PRGBRST_DATA2_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flctl_prgbrst_data2_0::W](W) writer structure"]
impl crate::Writable for FLCTL_PRGBRST_DATA2_0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLCTL_PRGBRST_DATA2_0 to value 0xffff_ffff"]
impl crate::Resettable for FLCTL_PRGBRST_DATA2_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
