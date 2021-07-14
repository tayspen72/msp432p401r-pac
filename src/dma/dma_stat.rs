#[doc = "Register `DMA_STAT` reader"]
pub struct R(crate::R<DMA_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Enable status of the controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MASTEN_A {
    #[doc = "0: Controller disabled"]
    MASTEN_0 = 0,
    #[doc = "1: Controller enabled"]
    MASTEN_1 = 1,
}
impl From<MASTEN_A> for bool {
    #[inline(always)]
    fn from(variant: MASTEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTEN` reader - Enable status of the controller"]
pub struct MASTEN_R(crate::FieldReader<bool, MASTEN_A>);
impl MASTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MASTEN_A {
        match self.bits {
            false => MASTEN_A::MASTEN_0,
            true => MASTEN_A::MASTEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `MASTEN_0`"]
    #[inline(always)]
    pub fn is_masten_0(&self) -> bool {
        **self == MASTEN_A::MASTEN_0
    }
    #[doc = "Checks if the value of the field is `MASTEN_1`"]
    #[inline(always)]
    pub fn is_masten_1(&self) -> bool {
        **self == MASTEN_A::MASTEN_1
    }
}
impl core::ops::Deref for MASTEN_R {
    type Target = crate::FieldReader<bool, MASTEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Current state of the control state machine. State can be one of the following:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STATE_A {
    #[doc = "0: idle"]
    STATE_0 = 0,
    #[doc = "1: reading channel controller data"]
    STATE_1 = 1,
    #[doc = "2: reading source data end pointer"]
    STATE_2 = 2,
    #[doc = "3: reading destination data end pointer"]
    STATE_3 = 3,
    #[doc = "4: reading source data"]
    STATE_4 = 4,
    #[doc = "5: writing destination data"]
    STATE_5 = 5,
    #[doc = "6: waiting for DMA request to clear"]
    STATE_6 = 6,
    #[doc = "7: writing channel controller data"]
    STATE_7 = 7,
    #[doc = "8: stalled"]
    STATE_8 = 8,
    #[doc = "9: done"]
    STATE_9 = 9,
    #[doc = "10: peripheral scatter-gather transition"]
    STATE_10 = 10,
}
impl From<STATE_A> for u8 {
    #[inline(always)]
    fn from(variant: STATE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `STATE` reader - Current state of the control state machine. State can be one of the following:"]
pub struct STATE_R(crate::FieldReader<u8, STATE_A>);
impl STATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        STATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STATE_A> {
        match self.bits {
            0 => Some(STATE_A::STATE_0),
            1 => Some(STATE_A::STATE_1),
            2 => Some(STATE_A::STATE_2),
            3 => Some(STATE_A::STATE_3),
            4 => Some(STATE_A::STATE_4),
            5 => Some(STATE_A::STATE_5),
            6 => Some(STATE_A::STATE_6),
            7 => Some(STATE_A::STATE_7),
            8 => Some(STATE_A::STATE_8),
            9 => Some(STATE_A::STATE_9),
            10 => Some(STATE_A::STATE_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STATE_0`"]
    #[inline(always)]
    pub fn is_state_0(&self) -> bool {
        **self == STATE_A::STATE_0
    }
    #[doc = "Checks if the value of the field is `STATE_1`"]
    #[inline(always)]
    pub fn is_state_1(&self) -> bool {
        **self == STATE_A::STATE_1
    }
    #[doc = "Checks if the value of the field is `STATE_2`"]
    #[inline(always)]
    pub fn is_state_2(&self) -> bool {
        **self == STATE_A::STATE_2
    }
    #[doc = "Checks if the value of the field is `STATE_3`"]
    #[inline(always)]
    pub fn is_state_3(&self) -> bool {
        **self == STATE_A::STATE_3
    }
    #[doc = "Checks if the value of the field is `STATE_4`"]
    #[inline(always)]
    pub fn is_state_4(&self) -> bool {
        **self == STATE_A::STATE_4
    }
    #[doc = "Checks if the value of the field is `STATE_5`"]
    #[inline(always)]
    pub fn is_state_5(&self) -> bool {
        **self == STATE_A::STATE_5
    }
    #[doc = "Checks if the value of the field is `STATE_6`"]
    #[inline(always)]
    pub fn is_state_6(&self) -> bool {
        **self == STATE_A::STATE_6
    }
    #[doc = "Checks if the value of the field is `STATE_7`"]
    #[inline(always)]
    pub fn is_state_7(&self) -> bool {
        **self == STATE_A::STATE_7
    }
    #[doc = "Checks if the value of the field is `STATE_8`"]
    #[inline(always)]
    pub fn is_state_8(&self) -> bool {
        **self == STATE_A::STATE_8
    }
    #[doc = "Checks if the value of the field is `STATE_9`"]
    #[inline(always)]
    pub fn is_state_9(&self) -> bool {
        **self == STATE_A::STATE_9
    }
    #[doc = "Checks if the value of the field is `STATE_10`"]
    #[inline(always)]
    pub fn is_state_10(&self) -> bool {
        **self == STATE_A::STATE_10
    }
}
impl core::ops::Deref for STATE_R {
    type Target = crate::FieldReader<u8, STATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Number of available DMA channels minus one.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DMACHANS_A {
    #[doc = "0: Controller configured to use 1 DMA channel"]
    DMACHANS_0 = 0,
    #[doc = "1: Controller configured to use 2 DMA channels"]
    DMACHANS_1 = 1,
    #[doc = "30: Controller configured to use 31 DMA channels"]
    DMACHANS_30 = 30,
    #[doc = "31: Controller configured to use 32 DMA channels"]
    DMACHANS_31 = 31,
}
impl From<DMACHANS_A> for u8 {
    #[inline(always)]
    fn from(variant: DMACHANS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DMACHANS` reader - Number of available DMA channels minus one."]
pub struct DMACHANS_R(crate::FieldReader<u8, DMACHANS_A>);
impl DMACHANS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMACHANS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DMACHANS_A> {
        match self.bits {
            0 => Some(DMACHANS_A::DMACHANS_0),
            1 => Some(DMACHANS_A::DMACHANS_1),
            30 => Some(DMACHANS_A::DMACHANS_30),
            31 => Some(DMACHANS_A::DMACHANS_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DMACHANS_0`"]
    #[inline(always)]
    pub fn is_dmachans_0(&self) -> bool {
        **self == DMACHANS_A::DMACHANS_0
    }
    #[doc = "Checks if the value of the field is `DMACHANS_1`"]
    #[inline(always)]
    pub fn is_dmachans_1(&self) -> bool {
        **self == DMACHANS_A::DMACHANS_1
    }
    #[doc = "Checks if the value of the field is `DMACHANS_30`"]
    #[inline(always)]
    pub fn is_dmachans_30(&self) -> bool {
        **self == DMACHANS_A::DMACHANS_30
    }
    #[doc = "Checks if the value of the field is `DMACHANS_31`"]
    #[inline(always)]
    pub fn is_dmachans_31(&self) -> bool {
        **self == DMACHANS_A::DMACHANS_31
    }
}
impl core::ops::Deref for DMACHANS_R {
    type Target = crate::FieldReader<u8, DMACHANS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TESTSTAT_A {
    #[doc = "0: Controller does not include the integration test logic"]
    TESTSTAT_0 = 0,
    #[doc = "1: Controller includes the integration test logic"]
    TESTSTAT_1 = 1,
}
impl From<TESTSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: TESTSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TESTSTAT` reader - To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved."]
pub struct TESTSTAT_R(crate::FieldReader<u8, TESTSTAT_A>);
impl TESTSTAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TESTSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TESTSTAT_A> {
        match self.bits {
            0 => Some(TESTSTAT_A::TESTSTAT_0),
            1 => Some(TESTSTAT_A::TESTSTAT_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TESTSTAT_0`"]
    #[inline(always)]
    pub fn is_teststat_0(&self) -> bool {
        **self == TESTSTAT_A::TESTSTAT_0
    }
    #[doc = "Checks if the value of the field is `TESTSTAT_1`"]
    #[inline(always)]
    pub fn is_teststat_1(&self) -> bool {
        **self == TESTSTAT_A::TESTSTAT_1
    }
}
impl core::ops::Deref for TESTSTAT_R {
    type Target = crate::FieldReader<u8, TESTSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enable status of the controller"]
    #[inline(always)]
    pub fn masten(&self) -> MASTEN_R {
        MASTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Current state of the control state machine. State can be one of the following:"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 16:20 - Number of available DMA channels minus one."]
    #[inline(always)]
    pub fn dmachans(&self) -> DMACHANS_R {
        DMACHANS_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 28:31 - To reduce the gate count the controller can be configured to exclude the integration test logic. The values 2h to Fh are Reserved."]
    #[inline(always)]
    pub fn teststat(&self) -> TESTSTAT_R {
        TESTSTAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_stat](index.html) module"]
pub struct DMA_STAT_SPEC;
impl crate::RegisterSpec for DMA_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_stat::R](R) reader structure"]
impl crate::Readable for DMA_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_STAT to value 0"]
impl crate::Resettable for DMA_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
