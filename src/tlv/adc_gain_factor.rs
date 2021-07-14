#[doc = "Register `ADC_GAIN_FACTOR` reader"]
pub struct R(crate::R<ADC_GAIN_FACTOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_GAIN_FACTOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_GAIN_FACTOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_GAIN_FACTOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ADC Gain Factor\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_gain_factor](index.html) module"]
pub struct ADC_GAIN_FACTOR_SPEC;
impl crate::RegisterSpec for ADC_GAIN_FACTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_gain_factor::R](R) reader structure"]
impl crate::Readable for ADC_GAIN_FACTOR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC_GAIN_FACTOR to value 0"]
impl crate::Resettable for ADC_GAIN_FACTOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
