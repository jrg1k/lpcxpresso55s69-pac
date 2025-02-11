#[doc = "Register `AHBMATPRIO` reader"]
pub type R = crate::R<AhbmatprioSpec>;
#[doc = "Register `AHBMATPRIO` writer"]
pub type W = crate::W<AhbmatprioSpec>;
#[doc = "Field `PRI_CPU0_CBUS` reader - CPU0 C-AHB bus."]
pub type PriCpu0CbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU0_CBUS` writer - CPU0 C-AHB bus."]
pub type PriCpu0CbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CPU0_SBUS` reader - CPU0 S-AHB bus."]
pub type PriCpu0SbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU0_SBUS` writer - CPU0 S-AHB bus."]
pub type PriCpu0SbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CPU1_CBUS` reader - CPU1 C-AHB bus."]
pub type PriCpu1CbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU1_CBUS` writer - CPU1 C-AHB bus."]
pub type PriCpu1CbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_CPU1_SBUS` reader - CPU1 S-AHB bus."]
pub type PriCpu1SbusR = crate::FieldReader;
#[doc = "Field `PRI_CPU1_SBUS` writer - CPU1 S-AHB bus."]
pub type PriCpu1SbusW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_USB_FS` reader - USB-FS.(USB0)"]
pub type PriUsbFsR = crate::FieldReader;
#[doc = "Field `PRI_USB_FS` writer - USB-FS.(USB0)"]
pub type PriUsbFsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SDMA0` reader - DMA0 controller priority."]
pub type PriSdma0R = crate::FieldReader;
#[doc = "Field `PRI_SDMA0` writer - DMA0 controller priority."]
pub type PriSdma0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SDIO` reader - SDIO."]
pub type PriSdioR = crate::FieldReader;
#[doc = "Field `PRI_SDIO` writer - SDIO."]
pub type PriSdioW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_PQ` reader - PQ (HW Accelerator)."]
pub type PriPqR = crate::FieldReader;
#[doc = "Field `PRI_PQ` writer - PQ (HW Accelerator)."]
pub type PriPqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_HASH_AES` reader - HASH_AES."]
pub type PriHashAesR = crate::FieldReader;
#[doc = "Field `PRI_HASH_AES` writer - HASH_AES."]
pub type PriHashAesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_USB_HS` reader - USB-HS.(USB1)"]
pub type PriUsbHsR = crate::FieldReader;
#[doc = "Field `PRI_USB_HS` writer - USB-HS.(USB1)"]
pub type PriUsbHsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SDMA1` reader - DMA1 controller priority."]
pub type PriSdma1R = crate::FieldReader;
#[doc = "Field `PRI_SDMA1` writer - DMA1 controller priority."]
pub type PriSdma1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&self) -> PriCpu0CbusR {
        PriCpu0CbusR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&self) -> PriCpu0SbusR {
        PriCpu0SbusR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CPU1 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_cbus(&self) -> PriCpu1CbusR {
        PriCpu1CbusR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - CPU1 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_sbus(&self) -> PriCpu1SbusR {
        PriCpu1SbusR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fs(&self) -> PriUsbFsR {
        PriUsbFsR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&self) -> PriSdma0R {
        PriSdma0R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&self) -> PriSdioR {
        PriSdioR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - PQ (HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&self) -> PriPqR {
        PriPqR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&self) -> PriHashAesR {
        PriHashAesR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline(always)]
    pub fn pri_usb_hs(&self) -> PriUsbHsR {
        PriUsbHsR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&self) -> PriSdma1R {
        PriSdma1R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CPU0 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_cbus(&mut self) -> PriCpu0CbusW<AhbmatprioSpec> {
        PriCpu0CbusW::new(self, 0)
    }
    #[doc = "Bits 2:3 - CPU0 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu0_sbus(&mut self) -> PriCpu0SbusW<AhbmatprioSpec> {
        PriCpu0SbusW::new(self, 2)
    }
    #[doc = "Bits 4:5 - CPU1 C-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_cbus(&mut self) -> PriCpu1CbusW<AhbmatprioSpec> {
        PriCpu1CbusW::new(self, 4)
    }
    #[doc = "Bits 6:7 - CPU1 S-AHB bus."]
    #[inline(always)]
    pub fn pri_cpu1_sbus(&mut self) -> PriCpu1SbusW<AhbmatprioSpec> {
        PriCpu1SbusW::new(self, 6)
    }
    #[doc = "Bits 8:9 - USB-FS.(USB0)"]
    #[inline(always)]
    pub fn pri_usb_fs(&mut self) -> PriUsbFsW<AhbmatprioSpec> {
        PriUsbFsW::new(self, 8)
    }
    #[doc = "Bits 10:11 - DMA0 controller priority."]
    #[inline(always)]
    pub fn pri_sdma0(&mut self) -> PriSdma0W<AhbmatprioSpec> {
        PriSdma0W::new(self, 10)
    }
    #[doc = "Bits 16:17 - SDIO."]
    #[inline(always)]
    pub fn pri_sdio(&mut self) -> PriSdioW<AhbmatprioSpec> {
        PriSdioW::new(self, 16)
    }
    #[doc = "Bits 18:19 - PQ (HW Accelerator)."]
    #[inline(always)]
    pub fn pri_pq(&mut self) -> PriPqW<AhbmatprioSpec> {
        PriPqW::new(self, 18)
    }
    #[doc = "Bits 20:21 - HASH_AES."]
    #[inline(always)]
    pub fn pri_hash_aes(&mut self) -> PriHashAesW<AhbmatprioSpec> {
        PriHashAesW::new(self, 20)
    }
    #[doc = "Bits 22:23 - USB-HS.(USB1)"]
    #[inline(always)]
    pub fn pri_usb_hs(&mut self) -> PriUsbHsW<AhbmatprioSpec> {
        PriUsbHsW::new(self, 22)
    }
    #[doc = "Bits 24:25 - DMA1 controller priority."]
    #[inline(always)]
    pub fn pri_sdma1(&mut self) -> PriSdma1W<AhbmatprioSpec> {
        PriSdma1W::new(self, 24)
    }
}
#[doc = "AHB Matrix priority control register Priority values are 3 = highest, 0 = lowest\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbmatprio::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbmatprio::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbmatprioSpec;
impl crate::RegisterSpec for AhbmatprioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbmatprio::R`](R) reader structure"]
impl crate::Readable for AhbmatprioSpec {}
#[doc = "`write(|w| ..)` method takes [`ahbmatprio::W`](W) writer structure"]
impl crate::Writable for AhbmatprioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AHBMATPRIO to value 0"]
impl crate::Resettable for AhbmatprioSpec {
    const RESET_VALUE: u32 = 0;
}
