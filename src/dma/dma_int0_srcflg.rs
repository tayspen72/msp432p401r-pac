#[doc = "Register `DMA_INT0_SRCFLG` reader"]
pub struct R(crate::R<DMA_INT0_SRCFLG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT0_SRCFLG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT0_SRCFLG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT0_SRCFLG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0` reader - Channel 0 was the source of DMA_INT0"]
pub struct CH0_R(crate::FieldReader<bool, bool>);
impl CH0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1` reader - Channel 1 was the source of DMA_INT0"]
pub struct CH1_R(crate::FieldReader<bool, bool>);
impl CH1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2` reader - Channel 2 was the source of DMA_INT0"]
pub struct CH2_R(crate::FieldReader<bool, bool>);
impl CH2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3` reader - Channel 3 was the source of DMA_INT0"]
pub struct CH3_R(crate::FieldReader<bool, bool>);
impl CH3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4` reader - Channel 4 was the source of DMA_INT0"]
pub struct CH4_R(crate::FieldReader<bool, bool>);
impl CH4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5` reader - Channel 5 was the source of DMA_INT0"]
pub struct CH5_R(crate::FieldReader<bool, bool>);
impl CH5_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH6` reader - Channel 6 was the source of DMA_INT0"]
pub struct CH6_R(crate::FieldReader<bool, bool>);
impl CH6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH7` reader - Channel 7 was the source of DMA_INT0"]
pub struct CH7_R(crate::FieldReader<bool, bool>);
impl CH7_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH8` reader - Channel 8 was the source of DMA_INT0"]
pub struct CH8_R(crate::FieldReader<bool, bool>);
impl CH8_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH9` reader - Channel 9 was the source of DMA_INT0"]
pub struct CH9_R(crate::FieldReader<bool, bool>);
impl CH9_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH10` reader - Channel 10 was the source of DMA_INT0"]
pub struct CH10_R(crate::FieldReader<bool, bool>);
impl CH10_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH11` reader - Channel 11 was the source of DMA_INT0"]
pub struct CH11_R(crate::FieldReader<bool, bool>);
impl CH11_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH12` reader - Channel 12 was the source of DMA_INT0"]
pub struct CH12_R(crate::FieldReader<bool, bool>);
impl CH12_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH13` reader - Channel 13 was the source of DMA_INT0"]
pub struct CH13_R(crate::FieldReader<bool, bool>);
impl CH13_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH14` reader - Channel 14 was the source of DMA_INT0"]
pub struct CH14_R(crate::FieldReader<bool, bool>);
impl CH14_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH15` reader - Channel 15 was the source of DMA_INT0"]
pub struct CH15_R(crate::FieldReader<bool, bool>);
impl CH15_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH16` reader - Channel 16 was the source of DMA_INT0"]
pub struct CH16_R(crate::FieldReader<bool, bool>);
impl CH16_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH17` reader - Channel 17 was the source of DMA_INT0"]
pub struct CH17_R(crate::FieldReader<bool, bool>);
impl CH17_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH18` reader - Channel 18 was the source of DMA_INT0"]
pub struct CH18_R(crate::FieldReader<bool, bool>);
impl CH18_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH19` reader - Channel 19 was the source of DMA_INT0"]
pub struct CH19_R(crate::FieldReader<bool, bool>);
impl CH19_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH20` reader - Channel 20 was the source of DMA_INT0"]
pub struct CH20_R(crate::FieldReader<bool, bool>);
impl CH20_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH21` reader - Channel 21 was the source of DMA_INT0"]
pub struct CH21_R(crate::FieldReader<bool, bool>);
impl CH21_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH22` reader - Channel 22 was the source of DMA_INT0"]
pub struct CH22_R(crate::FieldReader<bool, bool>);
impl CH22_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH23` reader - Channel 23 was the source of DMA_INT0"]
pub struct CH23_R(crate::FieldReader<bool, bool>);
impl CH23_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH24` reader - Channel 24 was the source of DMA_INT0"]
pub struct CH24_R(crate::FieldReader<bool, bool>);
impl CH24_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH25` reader - Channel 25 was the source of DMA_INT0"]
pub struct CH25_R(crate::FieldReader<bool, bool>);
impl CH25_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH26` reader - Channel 26 was the source of DMA_INT0"]
pub struct CH26_R(crate::FieldReader<bool, bool>);
impl CH26_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH27` reader - Channel 27 was the source of DMA_INT0"]
pub struct CH27_R(crate::FieldReader<bool, bool>);
impl CH27_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH28` reader - Channel 28 was the source of DMA_INT0"]
pub struct CH28_R(crate::FieldReader<bool, bool>);
impl CH28_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH29` reader - Channel 29 was the source of DMA_INT0"]
pub struct CH29_R(crate::FieldReader<bool, bool>);
impl CH29_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH30` reader - Channel 30 was the source of DMA_INT0"]
pub struct CH30_R(crate::FieldReader<bool, bool>);
impl CH30_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH31` reader - Channel 31 was the source of DMA_INT0"]
pub struct CH31_R(crate::FieldReader<bool, bool>);
impl CH31_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel 6 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel 7 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Channel 8 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Channel 9 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Channel 10 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Channel 11 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Channel 12 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Channel 13 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel 14 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Channel 15 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Channel 16 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Channel 17 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Channel 18 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Channel 19 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Channel 20 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Channel 21 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Channel 22 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Channel 23 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Channel 24 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Channel 25 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Channel 26 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Channel 27 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Channel 28 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel 29 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Channel 30 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Channel 31 was the source of DMA_INT0"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "Interrupt 0 Source Channel Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int0_srcflg](index.html) module"]
pub struct DMA_INT0_SRCFLG_SPEC;
impl crate::RegisterSpec for DMA_INT0_SRCFLG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int0_srcflg::R](R) reader structure"]
impl crate::Readable for DMA_INT0_SRCFLG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT0_SRCFLG to value 0"]
impl crate::Resettable for DMA_INT0_SRCFLG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
