#[doc = "Register `RSTCTL_SOFTRESET_STAT` reader"]
pub struct R(crate::R<RSTCTL_SOFTRESET_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCTL_SOFTRESET_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCTL_SOFTRESET_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCTL_SOFTRESET_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SRC0` reader - If 1, indicates that SRC0 was the source of the Soft Reset"]
pub struct SRC0_R(crate::FieldReader<bool, bool>);
impl SRC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC1` reader - If 1, indicates that SRC1 was the source of the Soft Reset"]
pub struct SRC1_R(crate::FieldReader<bool, bool>);
impl SRC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC2` reader - If 1, indicates that SRC2 was the source of the Soft Reset"]
pub struct SRC2_R(crate::FieldReader<bool, bool>);
impl SRC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC3` reader - If 1, indicates that SRC3 was the source of the Soft Reset"]
pub struct SRC3_R(crate::FieldReader<bool, bool>);
impl SRC3_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC4` reader - If 1, indicates that SRC4 was the source of the Soft Reset"]
pub struct SRC4_R(crate::FieldReader<bool, bool>);
impl SRC4_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC5` reader - If 1, indicates that SRC5 was the source of the Soft Reset"]
pub struct SRC5_R(crate::FieldReader<bool, bool>);
impl SRC5_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC6` reader - If 1, indicates that SRC6 was the source of the Soft Reset"]
pub struct SRC6_R(crate::FieldReader<bool, bool>);
impl SRC6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC7` reader - If 1, indicates that SRC7 was the source of the Soft Reset"]
pub struct SRC7_R(crate::FieldReader<bool, bool>);
impl SRC7_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC8` reader - If 1, indicates that SRC8 was the source of the Soft Reset"]
pub struct SRC8_R(crate::FieldReader<bool, bool>);
impl SRC8_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC9` reader - If 1, indicates that SRC9 was the source of the Soft Reset"]
pub struct SRC9_R(crate::FieldReader<bool, bool>);
impl SRC9_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC10` reader - If 1, indicates that SRC10 was the source of the Soft Reset"]
pub struct SRC10_R(crate::FieldReader<bool, bool>);
impl SRC10_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC11` reader - If 1, indicates that SRC11 was the source of the Soft Reset"]
pub struct SRC11_R(crate::FieldReader<bool, bool>);
impl SRC11_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC12` reader - If 1, indicates that SRC12 was the source of the Soft Reset"]
pub struct SRC12_R(crate::FieldReader<bool, bool>);
impl SRC12_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC13` reader - If 1, indicates that SRC13 was the source of the Soft Reset"]
pub struct SRC13_R(crate::FieldReader<bool, bool>);
impl SRC13_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC14` reader - If 1, indicates that SRC14 was the source of the Soft Reset"]
pub struct SRC14_R(crate::FieldReader<bool, bool>);
impl SRC14_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRC15` reader - If 1, indicates that SRC15 was the source of the Soft Reset"]
pub struct SRC15_R(crate::FieldReader<bool, bool>);
impl SRC15_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRC15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRC15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - If 1, indicates that SRC0 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src0(&self) -> SRC0_R {
        SRC0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If 1, indicates that SRC1 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src1(&self) -> SRC1_R {
        SRC1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - If 1, indicates that SRC2 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src2(&self) -> SRC2_R {
        SRC2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - If 1, indicates that SRC3 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src3(&self) -> SRC3_R {
        SRC3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - If 1, indicates that SRC4 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src4(&self) -> SRC4_R {
        SRC4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - If 1, indicates that SRC5 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src5(&self) -> SRC5_R {
        SRC5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - If 1, indicates that SRC6 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src6(&self) -> SRC6_R {
        SRC6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - If 1, indicates that SRC7 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src7(&self) -> SRC7_R {
        SRC7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - If 1, indicates that SRC8 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src8(&self) -> SRC8_R {
        SRC8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - If 1, indicates that SRC9 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src9(&self) -> SRC9_R {
        SRC9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - If 1, indicates that SRC10 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src10(&self) -> SRC10_R {
        SRC10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - If 1, indicates that SRC11 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src11(&self) -> SRC11_R {
        SRC11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - If 1, indicates that SRC12 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src12(&self) -> SRC12_R {
        SRC12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - If 1, indicates that SRC13 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src13(&self) -> SRC13_R {
        SRC13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - If 1, indicates that SRC14 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src14(&self) -> SRC14_R {
        SRC14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - If 1, indicates that SRC15 was the source of the Soft Reset"]
    #[inline(always)]
    pub fn src15(&self) -> SRC15_R {
        SRC15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Soft Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstctl_softreset_stat](index.html) module"]
pub struct RSTCTL_SOFTRESET_STAT_SPEC;
impl crate::RegisterSpec for RSTCTL_SOFTRESET_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstctl_softreset_stat::R](R) reader structure"]
impl crate::Readable for RSTCTL_SOFTRESET_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCTL_SOFTRESET_STAT to value 0"]
impl crate::Resettable for RSTCTL_SOFTRESET_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
