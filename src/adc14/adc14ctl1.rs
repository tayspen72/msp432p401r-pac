#[doc = "Register `ADC14CTL1` reader"]
pub struct R(crate::R<ADC14CTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14CTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14CTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14CTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC14CTL1` writer"]
pub struct W(crate::W<ADC14CTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC14CTL1_SPEC>;
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
impl From<crate::W<ADC14CTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC14CTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC14 power modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14PWRMD_A {
    #[doc = "0: Regular power mode for use with any resolution setting. Sample rate can be up to 1 Msps."]
    ADC14PWRMD_0 = 0,
    #[doc = "2: Low-power mode for 12-bit, 10-bit, and 8-bit resolution settings. Sample rate must not exceed 200 ksps."]
    ADC14PWRMD_2 = 2,
}
impl From<ADC14PWRMD_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14PWRMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14PWRMD` reader - ADC14 power modes"]
pub struct ADC14PWRMD_R(crate::FieldReader<u8, ADC14PWRMD_A>);
impl ADC14PWRMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC14PWRMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC14PWRMD_A> {
        match self.bits {
            0 => Some(ADC14PWRMD_A::ADC14PWRMD_0),
            2 => Some(ADC14PWRMD_A::ADC14PWRMD_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14PWRMD_0`"]
    #[inline(always)]
    pub fn is_adc14pwrmd_0(&self) -> bool {
        **self == ADC14PWRMD_A::ADC14PWRMD_0
    }
    #[doc = "Checks if the value of the field is `ADC14PWRMD_2`"]
    #[inline(always)]
    pub fn is_adc14pwrmd_2(&self) -> bool {
        **self == ADC14PWRMD_A::ADC14PWRMD_2
    }
}
impl core::ops::Deref for ADC14PWRMD_R {
    type Target = crate::FieldReader<u8, ADC14PWRMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14PWRMD` writer - ADC14 power modes"]
pub struct ADC14PWRMD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14PWRMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14PWRMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Regular power mode for use with any resolution setting. Sample rate can be up to 1 Msps."]
    #[inline(always)]
    pub fn adc14pwrmd_0(self) -> &'a mut W {
        self.variant(ADC14PWRMD_A::ADC14PWRMD_0)
    }
    #[doc = "Low-power mode for 12-bit, 10-bit, and 8-bit resolution settings. Sample rate must not exceed 200 ksps."]
    #[inline(always)]
    pub fn adc14pwrmd_2(self) -> &'a mut W {
        self.variant(ADC14PWRMD_A::ADC14PWRMD_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "ADC14 reference buffer burst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14REFBURST_A {
    #[doc = "0: ADC reference buffer on continuously"]
    ADC14REFBURST_0 = 0,
    #[doc = "1: ADC reference buffer on only during sample-and-conversion"]
    ADC14REFBURST_1 = 1,
}
impl From<ADC14REFBURST_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14REFBURST_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14REFBURST` reader - ADC14 reference buffer burst"]
pub struct ADC14REFBURST_R(crate::FieldReader<bool, ADC14REFBURST_A>);
impl ADC14REFBURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14REFBURST_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14REFBURST_A {
        match self.bits {
            false => ADC14REFBURST_A::ADC14REFBURST_0,
            true => ADC14REFBURST_A::ADC14REFBURST_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14REFBURST_0`"]
    #[inline(always)]
    pub fn is_adc14refburst_0(&self) -> bool {
        **self == ADC14REFBURST_A::ADC14REFBURST_0
    }
    #[doc = "Checks if the value of the field is `ADC14REFBURST_1`"]
    #[inline(always)]
    pub fn is_adc14refburst_1(&self) -> bool {
        **self == ADC14REFBURST_A::ADC14REFBURST_1
    }
}
impl core::ops::Deref for ADC14REFBURST_R {
    type Target = crate::FieldReader<bool, ADC14REFBURST_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14REFBURST` writer - ADC14 reference buffer burst"]
pub struct ADC14REFBURST_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14REFBURST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14REFBURST_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC reference buffer on continuously"]
    #[inline(always)]
    pub fn adc14refburst_0(self) -> &'a mut W {
        self.variant(ADC14REFBURST_A::ADC14REFBURST_0)
    }
    #[doc = "ADC reference buffer on only during sample-and-conversion"]
    #[inline(always)]
    pub fn adc14refburst_1(self) -> &'a mut W {
        self.variant(ADC14REFBURST_A::ADC14REFBURST_1)
    }
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
#[doc = "ADC14 data read-back format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14DF_A {
    #[doc = "0: Binary unsigned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 0000h, and the analog input voltage + V(REF) results in 3FFFh"]
    ADC14DF_0 = 0,
    #[doc = "1: Signed binary (2s complement), left aligned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 8000h, and the analog input voltage + V(REF) results in 7FFCh"]
    ADC14DF_1 = 1,
}
impl From<ADC14DF_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14DF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14DF` reader - ADC14 data read-back format"]
pub struct ADC14DF_R(crate::FieldReader<bool, ADC14DF_A>);
impl ADC14DF_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14DF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14DF_A {
        match self.bits {
            false => ADC14DF_A::ADC14DF_0,
            true => ADC14DF_A::ADC14DF_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14DF_0`"]
    #[inline(always)]
    pub fn is_adc14df_0(&self) -> bool {
        **self == ADC14DF_A::ADC14DF_0
    }
    #[doc = "Checks if the value of the field is `ADC14DF_1`"]
    #[inline(always)]
    pub fn is_adc14df_1(&self) -> bool {
        **self == ADC14DF_A::ADC14DF_1
    }
}
impl core::ops::Deref for ADC14DF_R {
    type Target = crate::FieldReader<bool, ADC14DF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14DF` writer - ADC14 data read-back format"]
pub struct ADC14DF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14DF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14DF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Binary unsigned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 0000h, and the analog input voltage + V(REF) results in 3FFFh"]
    #[inline(always)]
    pub fn adc14df_0(self) -> &'a mut W {
        self.variant(ADC14DF_A::ADC14DF_0)
    }
    #[doc = "Signed binary (2s complement), left aligned. Theoretically, for ADC14DIF = 0 and 14-bit mode, the analog input voltage - V(REF) results in 8000h, and the analog input voltage + V(REF) results in 7FFCh"]
    #[inline(always)]
    pub fn adc14df_1(self) -> &'a mut W {
        self.variant(ADC14DF_A::ADC14DF_1)
    }
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
#[doc = "ADC14 resolution\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC14RES_A {
    #[doc = "0: 8 bit (9 clock cycle conversion time)"]
    ADC14RES_0 = 0,
    #[doc = "1: 10 bit (11 clock cycle conversion time)"]
    ADC14RES_1 = 1,
    #[doc = "2: 12 bit (14 clock cycle conversion time)"]
    ADC14RES_2 = 2,
    #[doc = "3: 14 bit (16 clock cycle conversion time)"]
    ADC14RES_3 = 3,
}
impl From<ADC14RES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC14RES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC14RES` reader - ADC14 resolution"]
pub struct ADC14RES_R(crate::FieldReader<u8, ADC14RES_A>);
impl ADC14RES_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC14RES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14RES_A {
        match self.bits {
            0 => ADC14RES_A::ADC14RES_0,
            1 => ADC14RES_A::ADC14RES_1,
            2 => ADC14RES_A::ADC14RES_2,
            3 => ADC14RES_A::ADC14RES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC14RES_0`"]
    #[inline(always)]
    pub fn is_adc14res_0(&self) -> bool {
        **self == ADC14RES_A::ADC14RES_0
    }
    #[doc = "Checks if the value of the field is `ADC14RES_1`"]
    #[inline(always)]
    pub fn is_adc14res_1(&self) -> bool {
        **self == ADC14RES_A::ADC14RES_1
    }
    #[doc = "Checks if the value of the field is `ADC14RES_2`"]
    #[inline(always)]
    pub fn is_adc14res_2(&self) -> bool {
        **self == ADC14RES_A::ADC14RES_2
    }
    #[doc = "Checks if the value of the field is `ADC14RES_3`"]
    #[inline(always)]
    pub fn is_adc14res_3(&self) -> bool {
        **self == ADC14RES_A::ADC14RES_3
    }
}
impl core::ops::Deref for ADC14RES_R {
    type Target = crate::FieldReader<u8, ADC14RES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14RES` writer - ADC14 resolution"]
pub struct ADC14RES_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14RES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14RES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "8 bit (9 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_0(self) -> &'a mut W {
        self.variant(ADC14RES_A::ADC14RES_0)
    }
    #[doc = "10 bit (11 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_1(self) -> &'a mut W {
        self.variant(ADC14RES_A::ADC14RES_1)
    }
    #[doc = "12 bit (14 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_2(self) -> &'a mut W {
        self.variant(ADC14RES_A::ADC14RES_2)
    }
    #[doc = "14 bit (16 clock cycle conversion time)"]
    #[inline(always)]
    pub fn adc14res_3(self) -> &'a mut W {
        self.variant(ADC14RES_A::ADC14RES_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `ADC14CSTARTADD` reader - ADC14 conversion start address"]
pub struct ADC14CSTARTADD_R(crate::FieldReader<u8, u8>);
impl ADC14CSTARTADD_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC14CSTARTADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC14CSTARTADD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14CSTARTADD` writer - ADC14 conversion start address"]
pub struct ADC14CSTARTADD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14CSTARTADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Controls 1/2 AVCC ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14BATMAP_A {
    #[doc = "0: ADC internal 1/2 x AVCC channel is not selected for ADC"]
    ADC14BATMAP_0 = 0,
    #[doc = "1: ADC internal 1/2 x AVCC channel is selected for ADC input channel MAX"]
    ADC14BATMAP_1 = 1,
}
impl From<ADC14BATMAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14BATMAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14BATMAP` reader - Controls 1/2 AVCC ADC input channel selection"]
pub struct ADC14BATMAP_R(crate::FieldReader<bool, ADC14BATMAP_A>);
impl ADC14BATMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14BATMAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14BATMAP_A {
        match self.bits {
            false => ADC14BATMAP_A::ADC14BATMAP_0,
            true => ADC14BATMAP_A::ADC14BATMAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14BATMAP_0`"]
    #[inline(always)]
    pub fn is_adc14batmap_0(&self) -> bool {
        **self == ADC14BATMAP_A::ADC14BATMAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14BATMAP_1`"]
    #[inline(always)]
    pub fn is_adc14batmap_1(&self) -> bool {
        **self == ADC14BATMAP_A::ADC14BATMAP_1
    }
}
impl core::ops::Deref for ADC14BATMAP_R {
    type Target = crate::FieldReader<bool, ADC14BATMAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14BATMAP` writer - Controls 1/2 AVCC ADC input channel selection"]
pub struct ADC14BATMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14BATMAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14BATMAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC internal 1/2 x AVCC channel is not selected for ADC"]
    #[inline(always)]
    pub fn adc14batmap_0(self) -> &'a mut W {
        self.variant(ADC14BATMAP_A::ADC14BATMAP_0)
    }
    #[doc = "ADC internal 1/2 x AVCC channel is selected for ADC input channel MAX"]
    #[inline(always)]
    pub fn adc14batmap_1(self) -> &'a mut W {
        self.variant(ADC14BATMAP_A::ADC14BATMAP_1)
    }
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
#[doc = "Controls temperature sensor ADC input channel selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14TCMAP_A {
    #[doc = "0: ADC internal temperature sensor channel is not selected for ADC"]
    ADC14TCMAP_0 = 0,
    #[doc = "1: ADC internal temperature sensor channel is selected for ADC input channel MAX-1"]
    ADC14TCMAP_1 = 1,
}
impl From<ADC14TCMAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14TCMAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TCMAP` reader - Controls temperature sensor ADC input channel selection"]
pub struct ADC14TCMAP_R(crate::FieldReader<bool, ADC14TCMAP_A>);
impl ADC14TCMAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14TCMAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14TCMAP_A {
        match self.bits {
            false => ADC14TCMAP_A::ADC14TCMAP_0,
            true => ADC14TCMAP_A::ADC14TCMAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14TCMAP_0`"]
    #[inline(always)]
    pub fn is_adc14tcmap_0(&self) -> bool {
        **self == ADC14TCMAP_A::ADC14TCMAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14TCMAP_1`"]
    #[inline(always)]
    pub fn is_adc14tcmap_1(&self) -> bool {
        **self == ADC14TCMAP_A::ADC14TCMAP_1
    }
}
impl core::ops::Deref for ADC14TCMAP_R {
    type Target = crate::FieldReader<bool, ADC14TCMAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14TCMAP` writer - Controls temperature sensor ADC input channel selection"]
pub struct ADC14TCMAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14TCMAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14TCMAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC internal temperature sensor channel is not selected for ADC"]
    #[inline(always)]
    pub fn adc14tcmap_0(self) -> &'a mut W {
        self.variant(ADC14TCMAP_A::ADC14TCMAP_0)
    }
    #[doc = "ADC internal temperature sensor channel is selected for ADC input channel MAX-1"]
    #[inline(always)]
    pub fn adc14tcmap_1(self) -> &'a mut W {
        self.variant(ADC14TCMAP_A::ADC14TCMAP_1)
    }
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
#[doc = "Controls internal channel 0 selection to ADC input channel MAX-2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14CH0MAP_A {
    #[doc = "0: ADC input channel internal 0 is not selected"]
    ADC14CH0MAP_0 = 0,
    #[doc = "1: ADC input channel internal 0 is selected for ADC input channel MAX-2"]
    ADC14CH0MAP_1 = 1,
}
impl From<ADC14CH0MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14CH0MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH0MAP` reader - Controls internal channel 0 selection to ADC input channel MAX-2"]
pub struct ADC14CH0MAP_R(crate::FieldReader<bool, ADC14CH0MAP_A>);
impl ADC14CH0MAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14CH0MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14CH0MAP_A {
        match self.bits {
            false => ADC14CH0MAP_A::ADC14CH0MAP_0,
            true => ADC14CH0MAP_A::ADC14CH0MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14CH0MAP_0`"]
    #[inline(always)]
    pub fn is_adc14ch0map_0(&self) -> bool {
        **self == ADC14CH0MAP_A::ADC14CH0MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14CH0MAP_1`"]
    #[inline(always)]
    pub fn is_adc14ch0map_1(&self) -> bool {
        **self == ADC14CH0MAP_A::ADC14CH0MAP_1
    }
}
impl core::ops::Deref for ADC14CH0MAP_R {
    type Target = crate::FieldReader<bool, ADC14CH0MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14CH0MAP` writer - Controls internal channel 0 selection to ADC input channel MAX-2"]
pub struct ADC14CH0MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14CH0MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14CH0MAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC input channel internal 0 is not selected"]
    #[inline(always)]
    pub fn adc14ch0map_0(self) -> &'a mut W {
        self.variant(ADC14CH0MAP_A::ADC14CH0MAP_0)
    }
    #[doc = "ADC input channel internal 0 is selected for ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map_1(self) -> &'a mut W {
        self.variant(ADC14CH0MAP_A::ADC14CH0MAP_1)
    }
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
#[doc = "Controls internal channel 1 selection to ADC input channel MAX-3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14CH1MAP_A {
    #[doc = "0: ADC input channel internal 1 is not selected"]
    ADC14CH1MAP_0 = 0,
    #[doc = "1: ADC input channel internal 1 is selected for ADC input channel MAX-3"]
    ADC14CH1MAP_1 = 1,
}
impl From<ADC14CH1MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14CH1MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH1MAP` reader - Controls internal channel 1 selection to ADC input channel MAX-3"]
pub struct ADC14CH1MAP_R(crate::FieldReader<bool, ADC14CH1MAP_A>);
impl ADC14CH1MAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14CH1MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14CH1MAP_A {
        match self.bits {
            false => ADC14CH1MAP_A::ADC14CH1MAP_0,
            true => ADC14CH1MAP_A::ADC14CH1MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14CH1MAP_0`"]
    #[inline(always)]
    pub fn is_adc14ch1map_0(&self) -> bool {
        **self == ADC14CH1MAP_A::ADC14CH1MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14CH1MAP_1`"]
    #[inline(always)]
    pub fn is_adc14ch1map_1(&self) -> bool {
        **self == ADC14CH1MAP_A::ADC14CH1MAP_1
    }
}
impl core::ops::Deref for ADC14CH1MAP_R {
    type Target = crate::FieldReader<bool, ADC14CH1MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14CH1MAP` writer - Controls internal channel 1 selection to ADC input channel MAX-3"]
pub struct ADC14CH1MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14CH1MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14CH1MAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC input channel internal 1 is not selected"]
    #[inline(always)]
    pub fn adc14ch1map_0(self) -> &'a mut W {
        self.variant(ADC14CH1MAP_A::ADC14CH1MAP_0)
    }
    #[doc = "ADC input channel internal 1 is selected for ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map_1(self) -> &'a mut W {
        self.variant(ADC14CH1MAP_A::ADC14CH1MAP_1)
    }
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
#[doc = "Controls internal channel 2 selection to ADC input channel MAX-4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14CH2MAP_A {
    #[doc = "0: ADC input channel internal 2 is not selected"]
    ADC14CH2MAP_0 = 0,
    #[doc = "1: ADC input channel internal 2 is selected for ADC input channel MAX-4"]
    ADC14CH2MAP_1 = 1,
}
impl From<ADC14CH2MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14CH2MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH2MAP` reader - Controls internal channel 2 selection to ADC input channel MAX-4"]
pub struct ADC14CH2MAP_R(crate::FieldReader<bool, ADC14CH2MAP_A>);
impl ADC14CH2MAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14CH2MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14CH2MAP_A {
        match self.bits {
            false => ADC14CH2MAP_A::ADC14CH2MAP_0,
            true => ADC14CH2MAP_A::ADC14CH2MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14CH2MAP_0`"]
    #[inline(always)]
    pub fn is_adc14ch2map_0(&self) -> bool {
        **self == ADC14CH2MAP_A::ADC14CH2MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14CH2MAP_1`"]
    #[inline(always)]
    pub fn is_adc14ch2map_1(&self) -> bool {
        **self == ADC14CH2MAP_A::ADC14CH2MAP_1
    }
}
impl core::ops::Deref for ADC14CH2MAP_R {
    type Target = crate::FieldReader<bool, ADC14CH2MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14CH2MAP` writer - Controls internal channel 2 selection to ADC input channel MAX-4"]
pub struct ADC14CH2MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14CH2MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14CH2MAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC input channel internal 2 is not selected"]
    #[inline(always)]
    pub fn adc14ch2map_0(self) -> &'a mut W {
        self.variant(ADC14CH2MAP_A::ADC14CH2MAP_0)
    }
    #[doc = "ADC input channel internal 2 is selected for ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map_1(self) -> &'a mut W {
        self.variant(ADC14CH2MAP_A::ADC14CH2MAP_1)
    }
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
#[doc = "Controls internal channel 3 selection to ADC input channel MAX-5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14CH3MAP_A {
    #[doc = "0: ADC input channel internal 3 is not selected"]
    ADC14CH3MAP_0 = 0,
    #[doc = "1: ADC input channel internal 3 is selected for ADC input channel MAX-5"]
    ADC14CH3MAP_1 = 1,
}
impl From<ADC14CH3MAP_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14CH3MAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14CH3MAP` reader - Controls internal channel 3 selection to ADC input channel MAX-5"]
pub struct ADC14CH3MAP_R(crate::FieldReader<bool, ADC14CH3MAP_A>);
impl ADC14CH3MAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14CH3MAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14CH3MAP_A {
        match self.bits {
            false => ADC14CH3MAP_A::ADC14CH3MAP_0,
            true => ADC14CH3MAP_A::ADC14CH3MAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14CH3MAP_0`"]
    #[inline(always)]
    pub fn is_adc14ch3map_0(&self) -> bool {
        **self == ADC14CH3MAP_A::ADC14CH3MAP_0
    }
    #[doc = "Checks if the value of the field is `ADC14CH3MAP_1`"]
    #[inline(always)]
    pub fn is_adc14ch3map_1(&self) -> bool {
        **self == ADC14CH3MAP_A::ADC14CH3MAP_1
    }
}
impl core::ops::Deref for ADC14CH3MAP_R {
    type Target = crate::FieldReader<bool, ADC14CH3MAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC14CH3MAP` writer - Controls internal channel 3 selection to ADC input channel MAX-5"]
pub struct ADC14CH3MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC14CH3MAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC14CH3MAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "ADC input channel internal 3 is not selected"]
    #[inline(always)]
    pub fn adc14ch3map_0(self) -> &'a mut W {
        self.variant(ADC14CH3MAP_A::ADC14CH3MAP_0)
    }
    #[doc = "ADC input channel internal 3 is selected for ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map_1(self) -> &'a mut W {
        self.variant(ADC14CH3MAP_A::ADC14CH3MAP_1)
    }
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
impl R {
    #[doc = "Bits 0:1 - ADC14 power modes"]
    #[inline(always)]
    pub fn adc14pwrmd(&self) -> ADC14PWRMD_R {
        ADC14PWRMD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - ADC14 reference buffer burst"]
    #[inline(always)]
    pub fn adc14refburst(&self) -> ADC14REFBURST_R {
        ADC14REFBURST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ADC14 data read-back format"]
    #[inline(always)]
    pub fn adc14df(&self) -> ADC14DF_R {
        ADC14DF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - ADC14 resolution"]
    #[inline(always)]
    pub fn adc14res(&self) -> ADC14RES_R {
        ADC14RES_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 16:20 - ADC14 conversion start address"]
    #[inline(always)]
    pub fn adc14cstartadd(&self) -> ADC14CSTARTADD_R {
        ADC14CSTARTADD_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Controls 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc14batmap(&self) -> ADC14BATMAP_R {
        ADC14BATMAP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Controls temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc14tcmap(&self) -> ADC14TCMAP_R {
        ADC14TCMAP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Controls internal channel 0 selection to ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map(&self) -> ADC14CH0MAP_R {
        ADC14CH0MAP_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Controls internal channel 1 selection to ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map(&self) -> ADC14CH1MAP_R {
        ADC14CH1MAP_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Controls internal channel 2 selection to ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map(&self) -> ADC14CH2MAP_R {
        ADC14CH2MAP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Controls internal channel 3 selection to ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map(&self) -> ADC14CH3MAP_R {
        ADC14CH3MAP_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC14 power modes"]
    #[inline(always)]
    pub fn adc14pwrmd(&mut self) -> ADC14PWRMD_W {
        ADC14PWRMD_W { w: self }
    }
    #[doc = "Bit 2 - ADC14 reference buffer burst"]
    #[inline(always)]
    pub fn adc14refburst(&mut self) -> ADC14REFBURST_W {
        ADC14REFBURST_W { w: self }
    }
    #[doc = "Bit 3 - ADC14 data read-back format"]
    #[inline(always)]
    pub fn adc14df(&mut self) -> ADC14DF_W {
        ADC14DF_W { w: self }
    }
    #[doc = "Bits 4:5 - ADC14 resolution"]
    #[inline(always)]
    pub fn adc14res(&mut self) -> ADC14RES_W {
        ADC14RES_W { w: self }
    }
    #[doc = "Bits 16:20 - ADC14 conversion start address"]
    #[inline(always)]
    pub fn adc14cstartadd(&mut self) -> ADC14CSTARTADD_W {
        ADC14CSTARTADD_W { w: self }
    }
    #[doc = "Bit 22 - Controls 1/2 AVCC ADC input channel selection"]
    #[inline(always)]
    pub fn adc14batmap(&mut self) -> ADC14BATMAP_W {
        ADC14BATMAP_W { w: self }
    }
    #[doc = "Bit 23 - Controls temperature sensor ADC input channel selection"]
    #[inline(always)]
    pub fn adc14tcmap(&mut self) -> ADC14TCMAP_W {
        ADC14TCMAP_W { w: self }
    }
    #[doc = "Bit 24 - Controls internal channel 0 selection to ADC input channel MAX-2"]
    #[inline(always)]
    pub fn adc14ch0map(&mut self) -> ADC14CH0MAP_W {
        ADC14CH0MAP_W { w: self }
    }
    #[doc = "Bit 25 - Controls internal channel 1 selection to ADC input channel MAX-3"]
    #[inline(always)]
    pub fn adc14ch1map(&mut self) -> ADC14CH1MAP_W {
        ADC14CH1MAP_W { w: self }
    }
    #[doc = "Bit 26 - Controls internal channel 2 selection to ADC input channel MAX-4"]
    #[inline(always)]
    pub fn adc14ch2map(&mut self) -> ADC14CH2MAP_W {
        ADC14CH2MAP_W { w: self }
    }
    #[doc = "Bit 27 - Controls internal channel 3 selection to ADC input channel MAX-5"]
    #[inline(always)]
    pub fn adc14ch3map(&mut self) -> ADC14CH3MAP_W {
        ADC14CH3MAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ctl1](index.html) module"]
pub struct ADC14CTL1_SPEC;
impl crate::RegisterSpec for ADC14CTL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14ctl1::R](R) reader structure"]
impl crate::Readable for ADC14CTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc14ctl1::W](W) writer structure"]
impl crate::Writable for ADC14CTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC14CTL1 to value 0x30"]
impl crate::Resettable for ADC14CTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x30
    }
}
