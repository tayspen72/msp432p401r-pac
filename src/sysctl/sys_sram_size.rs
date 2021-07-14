#[doc = "Register `SYS_SRAM_SIZE` reader"]
pub struct R(crate::R<SYS_SRAM_SIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_SRAM_SIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_SRAM_SIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_SRAM_SIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SIZE` reader - Indicates the size of SRAM on the device"]
pub struct SIZE_R(crate::FieldReader<u32, u32>);
impl SIZE_R {
    pub(crate) fn new(bits: u32) -> Self {
        SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIZE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Indicates the size of SRAM on the device"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "SRAM Size Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_sram_size](index.html) module"]
pub struct SYS_SRAM_SIZE_SPEC;
impl crate::RegisterSpec for SYS_SRAM_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_sram_size::R](R) reader structure"]
impl crate::Readable for SYS_SRAM_SIZE_SPEC {
    type Reader = R;
}
