#[doc = "Register `UCBxADDMASK` reader"]
pub struct R(crate::R<UCBXADDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UCBXADDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UCBXADDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UCBXADDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UCBxADDMASK` writer"]
pub struct W(crate::W<UCBXADDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UCBXADDMASK_SPEC>;
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
impl From<crate::W<UCBXADDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UCBXADDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDMASK` reader - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub struct ADDMASK_R(crate::FieldReader<u16, u16>);
impl ADDMASK_R {
    pub(crate) fn new(bits: u16) -> Self {
        ADDMASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDMASK_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDMASK` writer - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
pub struct ADDMASK_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDMASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u16 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&self) -> ADDMASK_R {
        ADDMASK_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Address Mask Register. By clearing the corresponding bit of the own address, this bit is a don't care when comparing the address on the bus to the own address. Using this method, it is possible to react on more than one slave address. When all bits of ADDMASKx are set, the address mask feature is deactivated. Modify only when UCSWRST = 1."]
    #[inline(always)]
    pub fn addmask(&mut self) -> ADDMASK_W {
        ADDMASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "eUSCI_Bx I2C Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ucbx_addmask](index.html) module"]
pub struct UCBXADDMASK_SPEC;
impl crate::RegisterSpec for UCBXADDMASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ucbx_addmask::R](R) reader structure"]
impl crate::Readable for UCBXADDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ucbx_addmask::W](W) writer structure"]
impl crate::Writable for UCBXADDMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UCBxADDMASK to value 0x03ff"]
impl crate::Resettable for UCBXADDMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03ff
    }
}
