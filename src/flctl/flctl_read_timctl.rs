#[doc = "Register `FLCTL_READ_TIMCTL` reader"]
pub struct R(crate::R<FLCTL_READ_TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLCTL_READ_TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLCTL_READ_TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLCTL_READ_TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SETUP` reader - Configures the length of the Setup phase for this operation"]
pub struct SETUP_R(crate::FieldReader<u8, u8>);
impl SETUP_R {
    pub(crate) fn new(bits: u8) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREF_BOOST1` reader - Length of the IREF_BOOST1 signal of the IP"]
pub struct IREF_BOOST1_R(crate::FieldReader<u8, u8>);
impl IREF_BOOST1_R {
    pub(crate) fn new(bits: u8) -> Self {
        IREF_BOOST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IREF_BOOST1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETUP_LONG` reader - Length of the Setup time into read mode when the device is recovering from one of the following conditions: Moving from Power-down or Standby back to Active and device is not trimmed. Moving from standby to active state in low-frequency active mode. Recovering from the LDO Boost operation after a Mass Erase."]
pub struct SETUP_LONG_R(crate::FieldReader<u8, u8>);
impl SETUP_LONG_R {
    pub(crate) fn new(bits: u8) -> Self {
        SETUP_LONG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_LONG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Configures the length of the Setup phase for this operation"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Length of the IREF_BOOST1 signal of the IP"]
    #[inline(always)]
    pub fn iref_boost1(&self) -> IREF_BOOST1_R {
        IREF_BOOST1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Length of the Setup time into read mode when the device is recovering from one of the following conditions: Moving from Power-down or Standby back to Active and device is not trimmed. Moving from standby to active state in low-frequency active mode. Recovering from the LDO Boost operation after a Mass Erase."]
    #[inline(always)]
    pub fn setup_long(&self) -> SETUP_LONG_R {
        SETUP_LONG_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Read Timing Control Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flctl_read_timctl](index.html) module"]
pub struct FLCTL_READ_TIMCTL_SPEC;
impl crate::RegisterSpec for FLCTL_READ_TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flctl_read_timctl::R](R) reader structure"]
impl crate::Readable for FLCTL_READ_TIMCTL_SPEC {
    type Reader = R;
}
