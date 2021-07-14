#[doc = "Register `BSL_PORTIF_CFG_UART` reader"]
pub struct R(crate::R<BSL_PORTIF_CFG_UART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BSL_PORTIF_CFG_UART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BSL_PORTIF_CFG_UART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BSL_PORTIF_CFG_UART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "BSL Port Interface Configuration for UART\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsl_portif_cfg_uart](index.html) module"]
pub struct BSL_PORTIF_CFG_UART_SPEC;
impl crate::RegisterSpec for BSL_PORTIF_CFG_UART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bsl_portif_cfg_uart::R](R) reader structure"]
impl crate::Readable for BSL_PORTIF_CFG_UART_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BSL_PORTIF_CFG_UART to value 0"]
impl crate::Resettable for BSL_PORTIF_CFG_UART_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
