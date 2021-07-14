#[doc = "Register `RTCTIM1` reader"]
pub struct R(crate::R<RTCTIM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCTIM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCTIM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCTIM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCTIM1` writer"]
pub struct W(crate::W<RTCTIM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCTIM1_SPEC>;
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
impl From<crate::W<RTCTIM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCTIM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Hours` reader - Hours (0 to 23)"]
pub struct HOURS_R(crate::FieldReader<u8, u8>);
impl HOURS_R {
    pub(crate) fn new(bits: u8) -> Self {
        HOURS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HOURS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `Hours` writer - Hours (0 to 23)"]
pub struct HOURS_W<'a> {
    w: &'a mut W,
}
impl<'a> HOURS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u16 & 0x1f);
        self.w
    }
}
#[doc = "Field `DayofWeek` reader - Day of week (0 to 6)"]
pub struct DAYOFWEEK_R(crate::FieldReader<u8, u8>);
impl DAYOFWEEK_R {
    pub(crate) fn new(bits: u8) -> Self {
        DAYOFWEEK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DAYOFWEEK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DayofWeek` writer - Day of week (0 to 6)"]
pub struct DAYOFWEEK_W<'a> {
    w: &'a mut W,
}
impl<'a> DAYOFWEEK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u16 & 0x07) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&self) -> HOURS_R {
        HOURS_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&self) -> DAYOFWEEK_R {
        DAYOFWEEK_R::new(((self.bits >> 8) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours (0 to 23)"]
    #[inline(always)]
    pub fn hours(&mut self) -> HOURS_W {
        HOURS_W { w: self }
    }
    #[doc = "Bits 8:10 - Day of week (0 to 6)"]
    #[inline(always)]
    pub fn dayof_week(&mut self) -> DAYOFWEEK_W {
        DAYOFWEEK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Real-Time Clock Hour, Day of Week\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtctim1](index.html) module"]
pub struct RTCTIM1_SPEC;
impl crate::RegisterSpec for RTCTIM1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtctim1::R](R) reader structure"]
impl crate::Readable for RTCTIM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtctim1::W](W) writer structure"]
impl crate::Writable for RTCTIM1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCTIM1 to value 0"]
impl crate::Resettable for RTCTIM1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
