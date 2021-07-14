#[doc = "Register `ADC14IFGR1` reader"]
pub struct R(crate::R<ADC14IFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC14IFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC14IFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC14IFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Interrupt flag for ADC14MEMx within comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14INIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14INIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14INIFG_1 = 1,
}
impl From<ADC14INIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14INIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14INIFG` reader - Interrupt flag for ADC14MEMx within comparator window"]
pub struct ADC14INIFG_R(crate::FieldReader<bool, ADC14INIFG_A>);
impl ADC14INIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14INIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14INIFG_A {
        match self.bits {
            false => ADC14INIFG_A::ADC14INIFG_0,
            true => ADC14INIFG_A::ADC14INIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14INIFG_0`"]
    #[inline(always)]
    pub fn is_adc14inifg_0(&self) -> bool {
        **self == ADC14INIFG_A::ADC14INIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14INIFG_1`"]
    #[inline(always)]
    pub fn is_adc14inifg_1(&self) -> bool {
        **self == ADC14INIFG_A::ADC14INIFG_1
    }
}
impl core::ops::Deref for ADC14INIFG_R {
    type Target = crate::FieldReader<bool, ADC14INIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt flag for ADC14MEMx below comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14LOIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14LOIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14LOIFG_1 = 1,
}
impl From<ADC14LOIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14LOIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14LOIFG` reader - Interrupt flag for ADC14MEMx below comparator window"]
pub struct ADC14LOIFG_R(crate::FieldReader<bool, ADC14LOIFG_A>);
impl ADC14LOIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14LOIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14LOIFG_A {
        match self.bits {
            false => ADC14LOIFG_A::ADC14LOIFG_0,
            true => ADC14LOIFG_A::ADC14LOIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14LOIFG_0`"]
    #[inline(always)]
    pub fn is_adc14loifg_0(&self) -> bool {
        **self == ADC14LOIFG_A::ADC14LOIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14LOIFG_1`"]
    #[inline(always)]
    pub fn is_adc14loifg_1(&self) -> bool {
        **self == ADC14LOIFG_A::ADC14LOIFG_1
    }
}
impl core::ops::Deref for ADC14LOIFG_R {
    type Target = crate::FieldReader<bool, ADC14LOIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Interrupt flag for ADC14MEMx above comparator window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14HIIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14HIIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14HIIFG_1 = 1,
}
impl From<ADC14HIIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14HIIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14HIIFG` reader - Interrupt flag for ADC14MEMx above comparator window"]
pub struct ADC14HIIFG_R(crate::FieldReader<bool, ADC14HIIFG_A>);
impl ADC14HIIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14HIIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14HIIFG_A {
        match self.bits {
            false => ADC14HIIFG_A::ADC14HIIFG_0,
            true => ADC14HIIFG_A::ADC14HIIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14HIIFG_0`"]
    #[inline(always)]
    pub fn is_adc14hiifg_0(&self) -> bool {
        **self == ADC14HIIFG_A::ADC14HIIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14HIIFG_1`"]
    #[inline(always)]
    pub fn is_adc14hiifg_1(&self) -> bool {
        **self == ADC14HIIFG_A::ADC14HIIFG_1
    }
}
impl core::ops::Deref for ADC14HIIFG_R {
    type Target = crate::FieldReader<bool, ADC14HIIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC14MEMx overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14OVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14OVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14OVIFG_1 = 1,
}
impl From<ADC14OVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14OVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14OVIFG` reader - ADC14MEMx overflow interrupt flag"]
pub struct ADC14OVIFG_R(crate::FieldReader<bool, ADC14OVIFG_A>);
impl ADC14OVIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14OVIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14OVIFG_A {
        match self.bits {
            false => ADC14OVIFG_A::ADC14OVIFG_0,
            true => ADC14OVIFG_A::ADC14OVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14OVIFG_0`"]
    #[inline(always)]
    pub fn is_adc14ovifg_0(&self) -> bool {
        **self == ADC14OVIFG_A::ADC14OVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14OVIFG_1`"]
    #[inline(always)]
    pub fn is_adc14ovifg_1(&self) -> bool {
        **self == ADC14OVIFG_A::ADC14OVIFG_1
    }
}
impl core::ops::Deref for ADC14OVIFG_R {
    type Target = crate::FieldReader<bool, ADC14OVIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC14 conversion time overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14TOVIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14TOVIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14TOVIFG_1 = 1,
}
impl From<ADC14TOVIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14TOVIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14TOVIFG` reader - ADC14 conversion time overflow interrupt flag"]
pub struct ADC14TOVIFG_R(crate::FieldReader<bool, ADC14TOVIFG_A>);
impl ADC14TOVIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14TOVIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14TOVIFG_A {
        match self.bits {
            false => ADC14TOVIFG_A::ADC14TOVIFG_0,
            true => ADC14TOVIFG_A::ADC14TOVIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIFG_0`"]
    #[inline(always)]
    pub fn is_adc14tovifg_0(&self) -> bool {
        **self == ADC14TOVIFG_A::ADC14TOVIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14TOVIFG_1`"]
    #[inline(always)]
    pub fn is_adc14tovifg_1(&self) -> bool {
        **self == ADC14TOVIFG_A::ADC14TOVIFG_1
    }
}
impl core::ops::Deref for ADC14TOVIFG_R {
    type Target = crate::FieldReader<bool, ADC14TOVIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "ADC14 local buffered reference ready interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC14RDYIFG_A {
    #[doc = "0: No interrupt pending"]
    ADC14RDYIFG_0 = 0,
    #[doc = "1: Interrupt pending"]
    ADC14RDYIFG_1 = 1,
}
impl From<ADC14RDYIFG_A> for bool {
    #[inline(always)]
    fn from(variant: ADC14RDYIFG_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC14RDYIFG` reader - ADC14 local buffered reference ready interrupt flag"]
pub struct ADC14RDYIFG_R(crate::FieldReader<bool, ADC14RDYIFG_A>);
impl ADC14RDYIFG_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADC14RDYIFG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC14RDYIFG_A {
        match self.bits {
            false => ADC14RDYIFG_A::ADC14RDYIFG_0,
            true => ADC14RDYIFG_A::ADC14RDYIFG_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIFG_0`"]
    #[inline(always)]
    pub fn is_adc14rdyifg_0(&self) -> bool {
        **self == ADC14RDYIFG_A::ADC14RDYIFG_0
    }
    #[doc = "Checks if the value of the field is `ADC14RDYIFG_1`"]
    #[inline(always)]
    pub fn is_adc14rdyifg_1(&self) -> bool {
        **self == ADC14RDYIFG_A::ADC14RDYIFG_1
    }
}
impl core::ops::Deref for ADC14RDYIFG_R {
    type Target = crate::FieldReader<bool, ADC14RDYIFG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 1 - Interrupt flag for ADC14MEMx within comparator window"]
    #[inline(always)]
    pub fn adc14inifg(&self) -> ADC14INIFG_R {
        ADC14INIFG_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Interrupt flag for ADC14MEMx below comparator window"]
    #[inline(always)]
    pub fn adc14loifg(&self) -> ADC14LOIFG_R {
        ADC14LOIFG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Interrupt flag for ADC14MEMx above comparator window"]
    #[inline(always)]
    pub fn adc14hiifg(&self) -> ADC14HIIFG_R {
        ADC14HIIFG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ADC14MEMx overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14ovifg(&self) -> ADC14OVIFG_R {
        ADC14OVIFG_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ADC14 conversion time overflow interrupt flag"]
    #[inline(always)]
    pub fn adc14tovifg(&self) -> ADC14TOVIFG_R {
        ADC14TOVIFG_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - ADC14 local buffered reference ready interrupt flag"]
    #[inline(always)]
    pub fn adc14rdyifg(&self) -> ADC14RDYIFG_R {
        ADC14RDYIFG_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Interrupt Flag 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc14ifgr1](index.html) module"]
pub struct ADC14IFGR1_SPEC;
impl crate::RegisterSpec for ADC14IFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc14ifgr1::R](R) reader structure"]
impl crate::Readable for ADC14IFGR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADC14IFGR1 to value 0"]
impl crate::Resettable for ADC14IFGR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
