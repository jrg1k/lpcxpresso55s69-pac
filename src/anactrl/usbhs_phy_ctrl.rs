#[doc = "Register `USBHS_PHY_CTRL` reader"]
pub type R = crate::R<UsbhsPhyCtrlSpec>;
#[doc = "Register `USBHS_PHY_CTRL` writer"]
pub type W = crate::W<UsbhsPhyCtrlSpec>;
#[doc = "Field `usb_vbusvalid_ext` reader - Override value for Vbus if using external detectors."]
pub type UsbVbusvalidExtR = crate::BitReader;
#[doc = "Field `usb_vbusvalid_ext` writer - Override value for Vbus if using external detectors."]
pub type UsbVbusvalidExtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `usb_id_ext` reader - Override value for ID if using external detectors."]
pub type UsbIdExtR = crate::BitReader;
#[doc = "Field `usb_id_ext` writer - Override value for ID if using external detectors."]
pub type UsbIdExtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&self) -> UsbVbusvalidExtR {
        UsbVbusvalidExtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&self) -> UsbIdExtR {
        UsbIdExtR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override value for Vbus if using external detectors."]
    #[inline(always)]
    pub fn usb_vbusvalid_ext(&mut self) -> UsbVbusvalidExtW<UsbhsPhyCtrlSpec> {
        UsbVbusvalidExtW::new(self, 0)
    }
    #[doc = "Bit 1 - Override value for ID if using external detectors."]
    #[inline(always)]
    pub fn usb_id_ext(&mut self) -> UsbIdExtW<UsbhsPhyCtrlSpec> {
        UsbIdExtW::new(self, 1)
    }
}
#[doc = "USB High Speed Phy Control\n\nYou can [`read`](crate::Reg::read) this register and get [`usbhs_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbhs_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbhsPhyCtrlSpec;
impl crate::RegisterSpec for UsbhsPhyCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbhs_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for UsbhsPhyCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`usbhs_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for UsbhsPhyCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBHS_PHY_CTRL to value 0x08"]
impl crate::Resettable for UsbhsPhyCtrlSpec {
    const RESET_VALUE: u32 = 0x08;
}
