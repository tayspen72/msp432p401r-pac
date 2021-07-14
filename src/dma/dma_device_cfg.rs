#[doc = "Register `DMA_DEVICE_CFG` reader"]
pub struct R(crate::R<DMA_DEVICE_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_DEVICE_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_DEVICE_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_DEVICE_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUM_DMA_CHANNELS` reader - Number of DMA channels available"]
pub struct NUM_DMA_CHANNELS_R(crate::FieldReader<u8, u8>);
impl NUM_DMA_CHANNELS_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_DMA_CHANNELS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_DMA_CHANNELS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NUM_SRC_PER_CHANNEL` reader - Number of DMA sources per channel"]
pub struct NUM_SRC_PER_CHANNEL_R(crate::FieldReader<u8, u8>);
impl NUM_SRC_PER_CHANNEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NUM_SRC_PER_CHANNEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NUM_SRC_PER_CHANNEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of DMA channels available"]
    #[inline(always)]
    pub fn num_dma_channels(&self) -> NUM_DMA_CHANNELS_R {
        NUM_DMA_CHANNELS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Number of DMA sources per channel"]
    #[inline(always)]
    pub fn num_src_per_channel(&self) -> NUM_SRC_PER_CHANNEL_R {
        NUM_SRC_PER_CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Device Configuration Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_device_cfg](index.html) module"]
pub struct DMA_DEVICE_CFG_SPEC;
impl crate::RegisterSpec for DMA_DEVICE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_device_cfg::R](R) reader structure"]
impl crate::Readable for DMA_DEVICE_CFG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_DEVICE_CFG to value 0"]
impl crate::Resettable for DMA_DEVICE_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
