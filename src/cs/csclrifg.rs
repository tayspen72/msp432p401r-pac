#[doc = "Register `CSCLRIFG` writer"]
pub struct W(crate::W<CSCLRIFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCLRIFG_SPEC>;
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
impl From<crate::W<CSCLRIFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCLRIFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clear LFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_LFXTIFG_AW {
    #[doc = "0: No effect"]
    CLR_LFXTIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_LFXTIFG_1 = 1,
}
impl From<CLR_LFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_LFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_LFXTIFG` writer - Clear LFXT oscillator fault interrupt flag"]
pub struct CLR_LFXTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_LFXTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_LFXTIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_lfxtifg_0(self) -> &'a mut W {
        self.variant(CLR_LFXTIFG_AW::CLR_LFXTIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg_1(self) -> &'a mut W {
        self.variant(CLR_LFXTIFG_AW::CLR_LFXTIFG_1)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Clear HFXT oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_HFXTIFG_AW {
    #[doc = "0: No effect"]
    CLR_HFXTIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_HFXTIFG_1 = 1,
}
impl From<CLR_HFXTIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_HFXTIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXTIFG` writer - Clear HFXT oscillator fault interrupt flag"]
pub struct CLR_HFXTIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_HFXTIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_HFXTIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxtifg_0(self) -> &'a mut W {
        self.variant(CLR_HFXTIFG_AW::CLR_HFXTIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg_1(self) -> &'a mut W {
        self.variant(CLR_HFXTIFG_AW::CLR_HFXTIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Clear HFXT2 oscillator fault interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_HFXT2IFG_AW {
    #[doc = "0: No effect"]
    CLR_HFXT2IFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_HFXT2IFG_1 = 1,
}
impl From<CLR_HFXT2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_HFXT2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_HFXT2IFG` writer - Clear HFXT2 oscillator fault interrupt flag"]
pub struct CLR_HFXT2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_HFXT2IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_HFXT2IFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_0(self) -> &'a mut W {
        self.variant(CLR_HFXT2IFG_AW::CLR_HFXT2IFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg_1(self) -> &'a mut W {
        self.variant(CLR_HFXT2IFG_AW::CLR_HFXT2IFG_1)
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
#[doc = "Clear DCO external resistor open circuit fault interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_DCOR_OPNIFG_AW {
    #[doc = "0: No effect"]
    CLR_DCOR_OPNIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_DCOR_OPNIFG_1 = 1,
}
impl From<CLR_DCOR_OPNIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_DCOR_OPNIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_DCOR_OPNIFG` writer - Clear DCO external resistor open circuit fault interrupt flag."]
pub struct CLR_DCOR_OPNIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_DCOR_OPNIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_DCOR_OPNIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_0(self) -> &'a mut W {
        self.variant(CLR_DCOR_OPNIFG_AW::CLR_DCOR_OPNIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_dcor_opnifg_1(self) -> &'a mut W {
        self.variant(CLR_DCOR_OPNIFG_AW::CLR_DCOR_OPNIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "REFCNT period counter clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_CALIFG_AW {
    #[doc = "0: No effect"]
    CLR_CALIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_CALIFG_1 = 1,
}
impl From<CLR_CALIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_CALIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_CALIFG` writer - REFCNT period counter clear interrupt flag"]
pub struct CLR_CALIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_CALIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_CALIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_califg_0(self) -> &'a mut W {
        self.variant(CLR_CALIFG_AW::CLR_CALIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_califg_1(self) -> &'a mut W {
        self.variant(CLR_CALIFG_AW::CLR_CALIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Start fault counter clear interrupt flag LFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTLFIFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTLFIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTLFIFG_1 = 1,
}
impl From<CLR_FCNTLFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTLFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTLFIFG` writer - Start fault counter clear interrupt flag LFXT"]
pub struct CLR_FCNTLFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_FCNTLFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_FCNTLFIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcntlfifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTLFIFG_AW::CLR_FCNTLFIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcntlfifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTLFIFG_AW::CLR_FCNTLFIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTHFIFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTHFIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTHFIFG_1 = 1,
}
impl From<CLR_FCNTHFIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTHFIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHFIFG` writer - Start fault counter clear interrupt flag HFXT"]
pub struct CLR_FCNTHFIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_FCNTHFIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_FCNTHFIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthfifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTHFIFG_AW::CLR_FCNTHFIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthfifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTHFIFG_AW::CLR_FCNTHFIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Start fault counter clear interrupt flag HFXT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_FCNTHF2IFG_AW {
    #[doc = "0: No effect"]
    CLR_FCNTHF2IFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_FCNTHF2IFG_1 = 1,
}
impl From<CLR_FCNTHF2IFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_FCNTHF2IFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_FCNTHF2IFG` writer - Start fault counter clear interrupt flag HFXT2"]
pub struct CLR_FCNTHF2IFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_FCNTHF2IFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_FCNTHF2IFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_0(self) -> &'a mut W {
        self.variant(CLR_FCNTHF2IFG_AW::CLR_FCNTHF2IFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg_1(self) -> &'a mut W {
        self.variant(CLR_FCNTHF2IFG_AW::CLR_FCNTHF2IFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "PLL out-of-lock clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLOOLIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLOOLIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLOOLIFG_1 = 1,
}
impl From<CLR_PLLOOLIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLOOLIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOOLIFG` writer - PLL out-of-lock clear interrupt flag"]
pub struct CLR_PLLOOLIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_PLLOOLIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_PLLOOLIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloolifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLOOLIFG_AW::CLR_PLLOOLIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLOOLIFG_AW::CLR_PLLOOLIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "PLL loss-of-signal clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLLOSIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLLOSIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLLOSIFG_1 = 1,
}
impl From<CLR_PLLLOSIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLLOSIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLLOSIFG` writer - PLL loss-of-signal clear interrupt flag"]
pub struct CLR_PLLLOSIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_PLLLOSIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_PLLLOSIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plllosifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLLOSIFG_AW::CLR_PLLLOSIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLLOSIFG_AW::CLR_PLLLOSIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "PLL out-of-range clear interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLR_PLLOORIFG_AW {
    #[doc = "0: No effect"]
    CLR_PLLOORIFG_0 = 0,
    #[doc = "1: Clear pending interrupt flag"]
    CLR_PLLOORIFG_1 = 1,
}
impl From<CLR_PLLOORIFG_AW> for bool {
    #[inline(always)]
    fn from(variant: CLR_PLLOORIFG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLR_PLLOORIFG` writer - PLL out-of-range clear interrupt flag"]
pub struct CLR_PLLOORIFG_W<'a> {
    w: &'a mut W,
}
impl<'a> CLR_PLLOORIFG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLR_PLLOORIFG_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn clr_plloorifg_0(self) -> &'a mut W {
        self.variant(CLR_PLLOORIFG_AW::CLR_PLLOORIFG_0)
    }
    #[doc = "Clear pending interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg_1(self) -> &'a mut W {
        self.variant(CLR_PLLOORIFG_AW::CLR_PLLOORIFG_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Clear LFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_lfxtifg(&mut self) -> CLR_LFXTIFG_W {
        CLR_LFXTIFG_W { w: self }
    }
    #[doc = "Bit 1 - Clear HFXT oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxtifg(&mut self) -> CLR_HFXTIFG_W {
        CLR_HFXTIFG_W { w: self }
    }
    #[doc = "Bit 2 - Clear HFXT2 oscillator fault interrupt flag"]
    #[inline(always)]
    pub fn clr_hfxt2ifg(&mut self) -> CLR_HFXT2IFG_W {
        CLR_HFXT2IFG_W { w: self }
    }
    #[doc = "Bit 6 - Clear DCO external resistor open circuit fault interrupt flag."]
    #[inline(always)]
    pub fn clr_dcor_opnifg(&mut self) -> CLR_DCOR_OPNIFG_W {
        CLR_DCOR_OPNIFG_W { w: self }
    }
    #[doc = "Bit 15 - REFCNT period counter clear interrupt flag"]
    #[inline(always)]
    pub fn clr_califg(&mut self) -> CLR_CALIFG_W {
        CLR_CALIFG_W { w: self }
    }
    #[doc = "Bit 8 - Start fault counter clear interrupt flag LFXT"]
    #[inline(always)]
    pub fn clr_fcntlfifg(&mut self) -> CLR_FCNTLFIFG_W {
        CLR_FCNTLFIFG_W { w: self }
    }
    #[doc = "Bit 9 - Start fault counter clear interrupt flag HFXT"]
    #[inline(always)]
    pub fn clr_fcnthfifg(&mut self) -> CLR_FCNTHFIFG_W {
        CLR_FCNTHFIFG_W { w: self }
    }
    #[doc = "Bit 10 - Start fault counter clear interrupt flag HFXT2"]
    #[inline(always)]
    pub fn clr_fcnthf2ifg(&mut self) -> CLR_FCNTHF2IFG_W {
        CLR_FCNTHF2IFG_W { w: self }
    }
    #[doc = "Bit 12 - PLL out-of-lock clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloolifg(&mut self) -> CLR_PLLOOLIFG_W {
        CLR_PLLOOLIFG_W { w: self }
    }
    #[doc = "Bit 13 - PLL loss-of-signal clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plllosifg(&mut self) -> CLR_PLLLOSIFG_W {
        CLR_PLLLOSIFG_W { w: self }
    }
    #[doc = "Bit 14 - PLL out-of-range clear interrupt flag"]
    #[inline(always)]
    pub fn clr_plloorifg(&mut self) -> CLR_PLLOORIFG_W {
        CLR_PLLOORIFG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Interrupt Flag Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csclrifg](index.html) module"]
pub struct CSCLRIFG_SPEC;
impl crate::RegisterSpec for CSCLRIFG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [csclrifg::W](W) writer structure"]
impl crate::Writable for CSCLRIFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCLRIFG to value 0"]
impl crate::Resettable for CSCLRIFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
