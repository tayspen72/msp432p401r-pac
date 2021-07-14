#[doc = "Register `ADC14_REF1P2V_TS30C` reader"]
pub struct R(crate::R<ADC14_REF1P2V_TS30C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14_REF1P2V_TS30C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14_REF1P2V_TS30C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14_REF1P2V_TS30C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "ADC14 1.2V Reference Temp. Sensor 30C\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14_ref1p2v_ts30c](index.html) module"]
pub struct ADC14_REF1P2V_TS30C_SPEC;
impl crate::RegisterSpec for ADC14_REF1P2V_TS30C_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14_ref1p2v_ts30c::R](R) reader structure"]
impl crate::Readable for ADC14_REF1P2V_TS30C_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC14_REF1P2V_TS30C to value 0"]
impl crate::Resettable for ADC14_REF1P2V_TS30C_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
