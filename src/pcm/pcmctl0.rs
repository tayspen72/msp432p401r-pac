#[doc = "Register `PCMCTL0` reader"]
pub struct R(crate::R<PCMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCMCTL0` writer"]
pub struct W(crate::W<PCMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCMCTL0_SPEC>;
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
impl From<crate::W<PCMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Active Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AMR_A {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    AMR_0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    AMR_1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    AMR_4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    AMR_5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    AMR_8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    AMR_9 = 9,
}
impl From<AMR_A> for u8 {
    #[inline(always)]
    fn from(variant: AMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `AMR` reader - Active Mode Request"]
pub struct AMR_R(crate::FieldReader<u8, AMR_A>);
impl AMR_R {
    pub(crate) fn new(bits: u8) -> Self {
        AMR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMR_A> {
        match self.bits {
            0 => Some(AMR_A::AMR_0),
            1 => Some(AMR_A::AMR_1),
            4 => Some(AMR_A::AMR_4),
            5 => Some(AMR_A::AMR_5),
            8 => Some(AMR_A::AMR_8),
            9 => Some(AMR_A::AMR_9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AMR_0`"]
    #[inline(always)]
    pub fn is_amr_0(&self) -> bool {
        **self == AMR_A::AMR_0
    }
    #[doc = "Checks if the value of the field is `AMR_1`"]
    #[inline(always)]
    pub fn is_amr_1(&self) -> bool {
        **self == AMR_A::AMR_1
    }
    #[doc = "Checks if the value of the field is `AMR_4`"]
    #[inline(always)]
    pub fn is_amr_4(&self) -> bool {
        **self == AMR_A::AMR_4
    }
    #[doc = "Checks if the value of the field is `AMR_5`"]
    #[inline(always)]
    pub fn is_amr_5(&self) -> bool {
        **self == AMR_A::AMR_5
    }
    #[doc = "Checks if the value of the field is `AMR_8`"]
    #[inline(always)]
    pub fn is_amr_8(&self) -> bool {
        **self == AMR_A::AMR_8
    }
    #[doc = "Checks if the value of the field is `AMR_9`"]
    #[inline(always)]
    pub fn is_amr_9(&self) -> bool {
        **self == AMR_A::AMR_9
    }
}
impl core::ops::Deref for AMR_R {
    type Target = crate::FieldReader<u8, AMR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMR` writer - Active Mode Request"]
pub struct AMR_W<'a> {
    w: &'a mut W,
}
impl<'a> AMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LDO based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_0(self) -> &'a mut W {
        self.variant(AMR_A::AMR_0)
    }
    #[doc = "LDO based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_1(self) -> &'a mut W {
        self.variant(AMR_A::AMR_1)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_4(self) -> &'a mut W {
        self.variant(AMR_A::AMR_4)
    }
    #[doc = "DC-DC based Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_5(self) -> &'a mut W {
        self.variant(AMR_A::AMR_5)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 0."]
    #[inline(always)]
    pub fn amr_8(self) -> &'a mut W {
        self.variant(AMR_A::AMR_8)
    }
    #[doc = "Low-Frequency Active Mode at Core voltage setting 1."]
    #[inline(always)]
    pub fn amr_9(self) -> &'a mut W {
        self.variant(AMR_A::AMR_9)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Low Power Mode Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPMR_A {
    #[doc = "0: LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    LPMR_0 = 0,
    #[doc = "10: LPM3.5. Core voltage setting 0."]
    LPMR_10 = 10,
    #[doc = "12: LPM4.5"]
    LPMR_12 = 12,
}
impl From<LPMR_A> for u8 {
    #[inline(always)]
    fn from(variant: LPMR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPMR` reader - Low Power Mode Request"]
pub struct LPMR_R(crate::FieldReader<u8, LPMR_A>);
impl LPMR_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPMR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPMR_A> {
        match self.bits {
            0 => Some(LPMR_A::LPMR_0),
            10 => Some(LPMR_A::LPMR_10),
            12 => Some(LPMR_A::LPMR_12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LPMR_0`"]
    #[inline(always)]
    pub fn is_lpmr_0(&self) -> bool {
        **self == LPMR_A::LPMR_0
    }
    #[doc = "Checks if the value of the field is `LPMR_10`"]
    #[inline(always)]
    pub fn is_lpmr_10(&self) -> bool {
        **self == LPMR_A::LPMR_10
    }
    #[doc = "Checks if the value of the field is `LPMR_12`"]
    #[inline(always)]
    pub fn is_lpmr_12(&self) -> bool {
        **self == LPMR_A::LPMR_12
    }
}
impl core::ops::Deref for LPMR_R {
    type Target = crate::FieldReader<u8, LPMR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPMR` writer - Low Power Mode Request"]
pub struct LPMR_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPMR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LPM3. Core voltage setting is similar to the mode from which LPM3 is entered."]
    #[inline(always)]
    pub fn lpmr_0(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_0)
    }
    #[doc = "LPM3.5. Core voltage setting 0."]
    #[inline(always)]
    pub fn lpmr_10(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_10)
    }
    #[doc = "LPM4.5"]
    #[inline(always)]
    pub fn lpmr_12(self) -> &'a mut W {
        self.variant(LPMR_A::LPMR_12)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Current Power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CPM_A {
    #[doc = "0: LDO based Active Mode at Core voltage setting 0."]
    CPM_0 = 0,
    #[doc = "1: LDO based Active Mode at Core voltage setting 1."]
    CPM_1 = 1,
    #[doc = "4: DC-DC based Active Mode at Core voltage setting 0."]
    CPM_4 = 4,
    #[doc = "5: DC-DC based Active Mode at Core voltage setting 1."]
    CPM_5 = 5,
    #[doc = "8: Low-Frequency Active Mode at Core voltage setting 0."]
    CPM_8 = 8,
    #[doc = "9: Low-Frequency Active Mode at Core voltage setting 1."]
    CPM_9 = 9,
    #[doc = "16: LDO based LPM0 at Core voltage setting 0."]
    CPM_16 = 16,
    #[doc = "17: LDO based LPM0 at Core voltage setting 1."]
    CPM_17 = 17,
    #[doc = "20: DC-DC based LPM0 at Core voltage setting 0."]
    CPM_20 = 20,
    #[doc = "21: DC-DC based LPM0 at Core voltage setting 1."]
    CPM_21 = 21,
    #[doc = "24: Low-Frequency LPM0 at Core voltage setting 0."]
    CPM_24 = 24,
    #[doc = "25: Low-Frequency LPM0 at Core voltage setting 1."]
    CPM_25 = 25,
    #[doc = "32: LPM3"]
    CPM_32 = 32,
}
impl From<CPM_A> for u8 {
    #[inline(always)]
    fn from(variant: CPM_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CPM` reader - Current Power Mode"]
pub struct CPM_R(crate::FieldReader<u8, CPM_A>);
impl CPM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CPM_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CPM_A> {
        match self.bits {
            0 => Some(CPM_A::CPM_0),
            1 => Some(CPM_A::CPM_1),
            4 => Some(CPM_A::CPM_4),
            5 => Some(CPM_A::CPM_5),
            8 => Some(CPM_A::CPM_8),
            9 => Some(CPM_A::CPM_9),
            16 => Some(CPM_A::CPM_16),
            17 => Some(CPM_A::CPM_17),
            20 => Some(CPM_A::CPM_20),
            21 => Some(CPM_A::CPM_21),
            24 => Some(CPM_A::CPM_24),
            25 => Some(CPM_A::CPM_25),
            32 => Some(CPM_A::CPM_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CPM_0`"]
    #[inline(always)]
    pub fn is_cpm_0(&self) -> bool {
        **self == CPM_A::CPM_0
    }
    #[doc = "Checks if the value of the field is `CPM_1`"]
    #[inline(always)]
    pub fn is_cpm_1(&self) -> bool {
        **self == CPM_A::CPM_1
    }
    #[doc = "Checks if the value of the field is `CPM_4`"]
    #[inline(always)]
    pub fn is_cpm_4(&self) -> bool {
        **self == CPM_A::CPM_4
    }
    #[doc = "Checks if the value of the field is `CPM_5`"]
    #[inline(always)]
    pub fn is_cpm_5(&self) -> bool {
        **self == CPM_A::CPM_5
    }
    #[doc = "Checks if the value of the field is `CPM_8`"]
    #[inline(always)]
    pub fn is_cpm_8(&self) -> bool {
        **self == CPM_A::CPM_8
    }
    #[doc = "Checks if the value of the field is `CPM_9`"]
    #[inline(always)]
    pub fn is_cpm_9(&self) -> bool {
        **self == CPM_A::CPM_9
    }
    #[doc = "Checks if the value of the field is `CPM_16`"]
    #[inline(always)]
    pub fn is_cpm_16(&self) -> bool {
        **self == CPM_A::CPM_16
    }
    #[doc = "Checks if the value of the field is `CPM_17`"]
    #[inline(always)]
    pub fn is_cpm_17(&self) -> bool {
        **self == CPM_A::CPM_17
    }
    #[doc = "Checks if the value of the field is `CPM_20`"]
    #[inline(always)]
    pub fn is_cpm_20(&self) -> bool {
        **self == CPM_A::CPM_20
    }
    #[doc = "Checks if the value of the field is `CPM_21`"]
    #[inline(always)]
    pub fn is_cpm_21(&self) -> bool {
        **self == CPM_A::CPM_21
    }
    #[doc = "Checks if the value of the field is `CPM_24`"]
    #[inline(always)]
    pub fn is_cpm_24(&self) -> bool {
        **self == CPM_A::CPM_24
    }
    #[doc = "Checks if the value of the field is `CPM_25`"]
    #[inline(always)]
    pub fn is_cpm_25(&self) -> bool {
        **self == CPM_A::CPM_25
    }
    #[doc = "Checks if the value of the field is `CPM_32`"]
    #[inline(always)]
    pub fn is_cpm_32(&self) -> bool {
        **self == CPM_A::CPM_32
    }
}
impl core::ops::Deref for CPM_R {
    type Target = crate::FieldReader<u8, CPM_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMKEY` reader - PCM key"]
pub struct PCMKEY_R(crate::FieldReader<u16, u16>);
impl PCMKEY_R {
    pub(crate) fn new(bits: u16) -> Self {
        PCMKEY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCMKEY_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCMKEY` writer - PCM key"]
pub struct PCMKEY_W<'a> {
    w: &'a mut W,
}
impl<'a> PCMKEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&self) -> AMR_R {
        AMR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&self) -> LPMR_R {
        LPMR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Current Power Mode"]
    #[inline(always)]
    pub fn cpm(&self) -> CPM_R {
        CPM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&self) -> PCMKEY_R {
        PCMKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active Mode Request"]
    #[inline(always)]
    pub fn amr(&mut self) -> AMR_W {
        AMR_W { w: self }
    }
    #[doc = "Bits 4:7 - Low Power Mode Request"]
    #[inline(always)]
    pub fn lpmr(&mut self) -> LPMR_W {
        LPMR_W { w: self }
    }
    #[doc = "Bits 16:31 - PCM key"]
    #[inline(always)]
    pub fn pcmkey(&mut self) -> PCMKEY_W {
        PCMKEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcmctl0](index.html) module"]
pub struct PCMCTL0_SPEC;
impl crate::RegisterSpec for PCMCTL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcmctl0::R](R) reader structure"]
impl crate::Readable for PCMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcmctl0::W](W) writer structure"]
impl crate::Writable for PCMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCMCTL0 to value 0xa596_0000"]
impl crate::Resettable for PCMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xa596_0000
    }
}
