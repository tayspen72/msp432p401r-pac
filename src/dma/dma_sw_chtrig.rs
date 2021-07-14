#[doc = "Register `DMA_SW_CHTRIG` reader"]
pub struct R(crate::R<DMA_SW_CHTRIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SW_CHTRIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SW_CHTRIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SW_CHTRIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_SW_CHTRIG` writer"]
pub struct W(crate::W<DMA_SW_CHTRIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SW_CHTRIG_SPEC>;
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
impl From<crate::W<DMA_SW_CHTRIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SW_CHTRIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0` reader - Write 1, triggers DMA_CHANNEL0"]
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
#[doc = "Field `CH0` writer - Write 1, triggers DMA_CHANNEL0"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CH1` reader - Write 1, triggers DMA_CHANNEL1"]
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
#[doc = "Field `CH1` writer - Write 1, triggers DMA_CHANNEL1"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH2` reader - Write 1, triggers DMA_CHANNEL2"]
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
#[doc = "Field `CH2` writer - Write 1, triggers DMA_CHANNEL2"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH3` reader - Write 1, triggers DMA_CHANNEL3"]
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
#[doc = "Field `CH3` writer - Write 1, triggers DMA_CHANNEL3"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH4` reader - Write 1, triggers DMA_CHANNEL4"]
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
#[doc = "Field `CH4` writer - Write 1, triggers DMA_CHANNEL4"]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH5` reader - Write 1, triggers DMA_CHANNEL5"]
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
#[doc = "Field `CH5` writer - Write 1, triggers DMA_CHANNEL5"]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `CH6` reader - Write 1, triggers DMA_CHANNEL6"]
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
#[doc = "Field `CH6` writer - Write 1, triggers DMA_CHANNEL6"]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `CH7` reader - Write 1, triggers DMA_CHANNEL7"]
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
#[doc = "Field `CH7` writer - Write 1, triggers DMA_CHANNEL7"]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CH8` reader - Write 1, triggers DMA_CHANNEL8"]
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
#[doc = "Field `CH8` writer - Write 1, triggers DMA_CHANNEL8"]
pub struct CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CH9` reader - Write 1, triggers DMA_CHANNEL9"]
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
#[doc = "Field `CH9` writer - Write 1, triggers DMA_CHANNEL9"]
pub struct CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CH10` reader - Write 1, triggers DMA_CHANNEL10"]
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
#[doc = "Field `CH10` writer - Write 1, triggers DMA_CHANNEL10"]
pub struct CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CH11` reader - Write 1, triggers DMA_CHANNEL11"]
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
#[doc = "Field `CH11` writer - Write 1, triggers DMA_CHANNEL11"]
pub struct CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CH12` reader - Write 1, triggers DMA_CHANNEL12"]
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
#[doc = "Field `CH12` writer - Write 1, triggers DMA_CHANNEL12"]
pub struct CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CH13` reader - Write 1, triggers DMA_CHANNEL13"]
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
#[doc = "Field `CH13` writer - Write 1, triggers DMA_CHANNEL13"]
pub struct CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `CH14` reader - Write 1, triggers DMA_CHANNEL14"]
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
#[doc = "Field `CH14` writer - Write 1, triggers DMA_CHANNEL14"]
pub struct CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `CH15` reader - Write 1, triggers DMA_CHANNEL15"]
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
#[doc = "Field `CH15` writer - Write 1, triggers DMA_CHANNEL15"]
pub struct CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `CH16` reader - Write 1, triggers DMA_CHANNEL16"]
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
#[doc = "Field `CH16` writer - Write 1, triggers DMA_CHANNEL16"]
pub struct CH16_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CH17` reader - Write 1, triggers DMA_CHANNEL17"]
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
#[doc = "Field `CH17` writer - Write 1, triggers DMA_CHANNEL17"]
pub struct CH17_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `CH18` reader - Write 1, triggers DMA_CHANNEL18"]
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
#[doc = "Field `CH18` writer - Write 1, triggers DMA_CHANNEL18"]
pub struct CH18_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `CH19` reader - Write 1, triggers DMA_CHANNEL19"]
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
#[doc = "Field `CH19` writer - Write 1, triggers DMA_CHANNEL19"]
pub struct CH19_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `CH20` reader - Write 1, triggers DMA_CHANNEL20"]
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
#[doc = "Field `CH20` writer - Write 1, triggers DMA_CHANNEL20"]
pub struct CH20_W<'a> {
    w: &'a mut W,
}
impl<'a> CH20_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `CH21` reader - Write 1, triggers DMA_CHANNEL21"]
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
#[doc = "Field `CH21` writer - Write 1, triggers DMA_CHANNEL21"]
pub struct CH21_W<'a> {
    w: &'a mut W,
}
impl<'a> CH21_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `CH22` reader - Write 1, triggers DMA_CHANNEL22"]
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
#[doc = "Field `CH22` writer - Write 1, triggers DMA_CHANNEL22"]
pub struct CH22_W<'a> {
    w: &'a mut W,
}
impl<'a> CH22_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CH23` reader - Write 1, triggers DMA_CHANNEL23"]
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
#[doc = "Field `CH23` writer - Write 1, triggers DMA_CHANNEL23"]
pub struct CH23_W<'a> {
    w: &'a mut W,
}
impl<'a> CH23_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CH24` reader - Write 1, triggers DMA_CHANNEL24"]
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
#[doc = "Field `CH24` writer - Write 1, triggers DMA_CHANNEL24"]
pub struct CH24_W<'a> {
    w: &'a mut W,
}
impl<'a> CH24_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CH25` reader - Write 1, triggers DMA_CHANNEL25"]
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
#[doc = "Field `CH25` writer - Write 1, triggers DMA_CHANNEL25"]
pub struct CH25_W<'a> {
    w: &'a mut W,
}
impl<'a> CH25_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CH26` reader - Write 1, triggers DMA_CHANNEL26"]
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
#[doc = "Field `CH26` writer - Write 1, triggers DMA_CHANNEL26"]
pub struct CH26_W<'a> {
    w: &'a mut W,
}
impl<'a> CH26_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CH27` reader - Write 1, triggers DMA_CHANNEL27"]
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
#[doc = "Field `CH27` writer - Write 1, triggers DMA_CHANNEL27"]
pub struct CH27_W<'a> {
    w: &'a mut W,
}
impl<'a> CH27_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CH28` reader - Write 1, triggers DMA_CHANNEL28"]
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
#[doc = "Field `CH28` writer - Write 1, triggers DMA_CHANNEL28"]
pub struct CH28_W<'a> {
    w: &'a mut W,
}
impl<'a> CH28_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CH29` reader - Write 1, triggers DMA_CHANNEL29"]
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
#[doc = "Field `CH29` writer - Write 1, triggers DMA_CHANNEL29"]
pub struct CH29_W<'a> {
    w: &'a mut W,
}
impl<'a> CH29_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `CH30` reader - Write 1, triggers DMA_CHANNEL30"]
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
#[doc = "Field `CH30` writer - Write 1, triggers DMA_CHANNEL30"]
pub struct CH30_W<'a> {
    w: &'a mut W,
}
impl<'a> CH30_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `CH31` reader - Write 1, triggers DMA_CHANNEL31"]
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
#[doc = "Field `CH31` writer - Write 1, triggers DMA_CHANNEL31"]
pub struct CH31_W<'a> {
    w: &'a mut W,
}
impl<'a> CH31_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Write 1, triggers DMA_CHANNEL0"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write 1, triggers DMA_CHANNEL1"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Write 1, triggers DMA_CHANNEL2"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Write 1, triggers DMA_CHANNEL3"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Write 1, triggers DMA_CHANNEL4"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Write 1, triggers DMA_CHANNEL5"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Write 1, triggers DMA_CHANNEL6"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Write 1, triggers DMA_CHANNEL7"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Write 1, triggers DMA_CHANNEL8"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Write 1, triggers DMA_CHANNEL9"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Write 1, triggers DMA_CHANNEL10"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Write 1, triggers DMA_CHANNEL11"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Write 1, triggers DMA_CHANNEL12"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Write 1, triggers DMA_CHANNEL13"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Write 1, triggers DMA_CHANNEL14"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Write 1, triggers DMA_CHANNEL15"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Write 1, triggers DMA_CHANNEL16"]
    #[inline(always)]
    pub fn ch16(&self) -> CH16_R {
        CH16_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Write 1, triggers DMA_CHANNEL17"]
    #[inline(always)]
    pub fn ch17(&self) -> CH17_R {
        CH17_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Write 1, triggers DMA_CHANNEL18"]
    #[inline(always)]
    pub fn ch18(&self) -> CH18_R {
        CH18_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Write 1, triggers DMA_CHANNEL19"]
    #[inline(always)]
    pub fn ch19(&self) -> CH19_R {
        CH19_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Write 1, triggers DMA_CHANNEL20"]
    #[inline(always)]
    pub fn ch20(&self) -> CH20_R {
        CH20_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Write 1, triggers DMA_CHANNEL21"]
    #[inline(always)]
    pub fn ch21(&self) -> CH21_R {
        CH21_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Write 1, triggers DMA_CHANNEL22"]
    #[inline(always)]
    pub fn ch22(&self) -> CH22_R {
        CH22_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write 1, triggers DMA_CHANNEL23"]
    #[inline(always)]
    pub fn ch23(&self) -> CH23_R {
        CH23_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Write 1, triggers DMA_CHANNEL24"]
    #[inline(always)]
    pub fn ch24(&self) -> CH24_R {
        CH24_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Write 1, triggers DMA_CHANNEL25"]
    #[inline(always)]
    pub fn ch25(&self) -> CH25_R {
        CH25_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Write 1, triggers DMA_CHANNEL26"]
    #[inline(always)]
    pub fn ch26(&self) -> CH26_R {
        CH26_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Write 1, triggers DMA_CHANNEL27"]
    #[inline(always)]
    pub fn ch27(&self) -> CH27_R {
        CH27_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Write 1, triggers DMA_CHANNEL28"]
    #[inline(always)]
    pub fn ch28(&self) -> CH28_R {
        CH28_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Write 1, triggers DMA_CHANNEL29"]
    #[inline(always)]
    pub fn ch29(&self) -> CH29_R {
        CH29_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Write 1, triggers DMA_CHANNEL30"]
    #[inline(always)]
    pub fn ch30(&self) -> CH30_R {
        CH30_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Write 1, triggers DMA_CHANNEL31"]
    #[inline(always)]
    pub fn ch31(&self) -> CH31_R {
        CH31_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write 1, triggers DMA_CHANNEL0"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bit 1 - Write 1, triggers DMA_CHANNEL1"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bit 2 - Write 1, triggers DMA_CHANNEL2"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bit 3 - Write 1, triggers DMA_CHANNEL3"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bit 4 - Write 1, triggers DMA_CHANNEL4"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bit 5 - Write 1, triggers DMA_CHANNEL5"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bit 6 - Write 1, triggers DMA_CHANNEL6"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bit 7 - Write 1, triggers DMA_CHANNEL7"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Bit 8 - Write 1, triggers DMA_CHANNEL8"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W { w: self }
    }
    #[doc = "Bit 9 - Write 1, triggers DMA_CHANNEL9"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W { w: self }
    }
    #[doc = "Bit 10 - Write 1, triggers DMA_CHANNEL10"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W { w: self }
    }
    #[doc = "Bit 11 - Write 1, triggers DMA_CHANNEL11"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W { w: self }
    }
    #[doc = "Bit 12 - Write 1, triggers DMA_CHANNEL12"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W { w: self }
    }
    #[doc = "Bit 13 - Write 1, triggers DMA_CHANNEL13"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W { w: self }
    }
    #[doc = "Bit 14 - Write 1, triggers DMA_CHANNEL14"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W { w: self }
    }
    #[doc = "Bit 15 - Write 1, triggers DMA_CHANNEL15"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W { w: self }
    }
    #[doc = "Bit 16 - Write 1, triggers DMA_CHANNEL16"]
    #[inline(always)]
    pub fn ch16(&mut self) -> CH16_W {
        CH16_W { w: self }
    }
    #[doc = "Bit 17 - Write 1, triggers DMA_CHANNEL17"]
    #[inline(always)]
    pub fn ch17(&mut self) -> CH17_W {
        CH17_W { w: self }
    }
    #[doc = "Bit 18 - Write 1, triggers DMA_CHANNEL18"]
    #[inline(always)]
    pub fn ch18(&mut self) -> CH18_W {
        CH18_W { w: self }
    }
    #[doc = "Bit 19 - Write 1, triggers DMA_CHANNEL19"]
    #[inline(always)]
    pub fn ch19(&mut self) -> CH19_W {
        CH19_W { w: self }
    }
    #[doc = "Bit 20 - Write 1, triggers DMA_CHANNEL20"]
    #[inline(always)]
    pub fn ch20(&mut self) -> CH20_W {
        CH20_W { w: self }
    }
    #[doc = "Bit 21 - Write 1, triggers DMA_CHANNEL21"]
    #[inline(always)]
    pub fn ch21(&mut self) -> CH21_W {
        CH21_W { w: self }
    }
    #[doc = "Bit 22 - Write 1, triggers DMA_CHANNEL22"]
    #[inline(always)]
    pub fn ch22(&mut self) -> CH22_W {
        CH22_W { w: self }
    }
    #[doc = "Bit 23 - Write 1, triggers DMA_CHANNEL23"]
    #[inline(always)]
    pub fn ch23(&mut self) -> CH23_W {
        CH23_W { w: self }
    }
    #[doc = "Bit 24 - Write 1, triggers DMA_CHANNEL24"]
    #[inline(always)]
    pub fn ch24(&mut self) -> CH24_W {
        CH24_W { w: self }
    }
    #[doc = "Bit 25 - Write 1, triggers DMA_CHANNEL25"]
    #[inline(always)]
    pub fn ch25(&mut self) -> CH25_W {
        CH25_W { w: self }
    }
    #[doc = "Bit 26 - Write 1, triggers DMA_CHANNEL26"]
    #[inline(always)]
    pub fn ch26(&mut self) -> CH26_W {
        CH26_W { w: self }
    }
    #[doc = "Bit 27 - Write 1, triggers DMA_CHANNEL27"]
    #[inline(always)]
    pub fn ch27(&mut self) -> CH27_W {
        CH27_W { w: self }
    }
    #[doc = "Bit 28 - Write 1, triggers DMA_CHANNEL28"]
    #[inline(always)]
    pub fn ch28(&mut self) -> CH28_W {
        CH28_W { w: self }
    }
    #[doc = "Bit 29 - Write 1, triggers DMA_CHANNEL29"]
    #[inline(always)]
    pub fn ch29(&mut self) -> CH29_W {
        CH29_W { w: self }
    }
    #[doc = "Bit 30 - Write 1, triggers DMA_CHANNEL30"]
    #[inline(always)]
    pub fn ch30(&mut self) -> CH30_W {
        CH30_W { w: self }
    }
    #[doc = "Bit 31 - Write 1, triggers DMA_CHANNEL31"]
    #[inline(always)]
    pub fn ch31(&mut self) -> CH31_W {
        CH31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Channel Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_sw_chtrig](index.html) module"]
pub struct DMA_SW_CHTRIG_SPEC;
impl crate::RegisterSpec for DMA_SW_CHTRIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_sw_chtrig::R](R) reader structure"]
impl crate::Readable for DMA_SW_CHTRIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_sw_chtrig::W](W) writer structure"]
impl crate::Writable for DMA_SW_CHTRIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_SW_CHTRIG to value 0"]
impl crate::Resettable for DMA_SW_CHTRIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
