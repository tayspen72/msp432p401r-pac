#[doc = "Register `ADC14MEM[%s]` reader"]
pub struct R(crate::R<ADC14MEM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14MEM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14MEM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14MEM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14MEM[%s]` writer"]
pub struct W(crate::W<ADC14MEM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14MEM_SPEC>;
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
impl From<crate::W<ADC14MEM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14MEM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Conversion_Results` reader - Conversion Result"]
pub struct CONVERSION_RESULTS_R(crate::FieldReader<u16, u16>);
impl CONVERSION_RESULTS_R {
    pub(crate) fn new(bits: u16) -> Self {
        CONVERSION_RESULTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONVERSION_RESULTS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Conversion_Results` writer - Conversion Result"]
pub struct CONVERSION_RESULTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CONVERSION_RESULTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&self) -> CONVERSION_RESULTS_R {
        CONVERSION_RESULTS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Conversion Result"]
    #[inline(always)]
    pub fn conversion_results(&mut self) -> CONVERSION_RESULTS_W {
        CONVERSION_RESULTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Conversion Memory Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14mem](index.html) module"]
pub struct ADC14MEM_SPEC;
impl crate::RegisterSpec for ADC14MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14mem::R](R) reader structure"]
impl crate::Readable for ADC14MEM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14mem::W](W) writer structure"]
impl crate::Writable for ADC14MEM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14MEM[%s]
to value 0"]
impl crate::Resettable for ADC14MEM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
