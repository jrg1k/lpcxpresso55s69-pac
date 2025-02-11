#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `CONTROLLER_RESET` reader - Controller reset."]
pub type ControllerResetR = crate::BitReader;
#[doc = "Field `CONTROLLER_RESET` writer - Controller reset."]
pub type ControllerResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_RESET` reader - Fifo reset."]
pub type FifoResetR = crate::BitReader;
#[doc = "Field `FIFO_RESET` writer - Fifo reset."]
pub type FifoResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RESET` reader - DMA reset."]
pub type DmaResetR = crate::BitReader;
#[doc = "Field `DMA_RESET` writer - DMA reset."]
pub type DmaResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_ENABLE` reader - Global interrupt enable/disable bit."]
pub type IntEnableR = crate::BitReader;
#[doc = "Field `INT_ENABLE` writer - Global interrupt enable/disable bit."]
pub type IntEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READ_WAIT` reader - Read/wait."]
pub type ReadWaitR = crate::BitReader;
#[doc = "Field `READ_WAIT` writer - Read/wait."]
pub type ReadWaitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_IRQ_RESPONSE` reader - Send irq response."]
pub type SendIrqResponseR = crate::BitReader;
#[doc = "Field `SEND_IRQ_RESPONSE` writer - Send irq response."]
pub type SendIrqResponseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_READ_DATA` reader - Abort read data."]
pub type AbortReadDataR = crate::BitReader;
#[doc = "Field `ABORT_READ_DATA` writer - Abort read data."]
pub type AbortReadDataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_CCSD` reader - Send ccsd."]
pub type SendCcsdR = crate::BitReader;
#[doc = "Field `SEND_CCSD` writer - Send ccsd."]
pub type SendCcsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEND_AUTO_STOP_CCSD` reader - Send auto stop ccsd."]
pub type SendAutoStopCcsdR = crate::BitReader;
#[doc = "Field `SEND_AUTO_STOP_CCSD` writer - Send auto stop ccsd."]
pub type SendAutoStopCcsdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` reader - CEATA device interrupt status."]
pub type CeataDeviceInterruptStatusR = crate::BitReader;
#[doc = "Field `CEATA_DEVICE_INTERRUPT_STATUS` writer - CEATA device interrupt status."]
pub type CeataDeviceInterruptStatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_VOLTAGE_A0` reader - Controls the state of the SD_VOLT0 pin."]
pub type CardVoltageA0R = crate::BitReader;
#[doc = "Field `CARD_VOLTAGE_A0` writer - Controls the state of the SD_VOLT0 pin."]
pub type CardVoltageA0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_VOLTAGE_A1` reader - Controls the state of the SD_VOLT1 pin."]
pub type CardVoltageA1R = crate::BitReader;
#[doc = "Field `CARD_VOLTAGE_A1` writer - Controls the state of the SD_VOLT1 pin."]
pub type CardVoltageA1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CARD_VOLTAGE_A2` reader - Controls the state of the SD_VOLT2 pin."]
pub type CardVoltageA2R = crate::BitReader;
#[doc = "Field `CARD_VOLTAGE_A2` writer - Controls the state of the SD_VOLT2 pin."]
pub type CardVoltageA2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_INTERNAL_DMAC` reader - SD/MMC DMA use."]
pub type UseInternalDmacR = crate::BitReader;
#[doc = "Field `USE_INTERNAL_DMAC` writer - SD/MMC DMA use."]
pub type UseInternalDmacW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&self) -> ControllerResetR {
        ControllerResetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&self) -> FifoResetR {
        FifoResetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&self) -> DmaResetR {
        DmaResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&self) -> IntEnableR {
        IntEnableR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&self) -> ReadWaitR {
        ReadWaitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&self) -> SendIrqResponseR {
        SendIrqResponseR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&self) -> AbortReadDataR {
        AbortReadDataR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&self) -> SendCcsdR {
        SendCcsdR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&self) -> SendAutoStopCcsdR {
        SendAutoStopCcsdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&self) -> CeataDeviceInterruptStatusR {
        CeataDeviceInterruptStatusR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&self) -> CardVoltageA0R {
        CardVoltageA0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&self) -> CardVoltageA1R {
        CardVoltageA1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&self) -> CardVoltageA2R {
        CardVoltageA2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&self) -> UseInternalDmacR {
        UseInternalDmacR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controller reset."]
    #[inline(always)]
    pub fn controller_reset(&mut self) -> ControllerResetW<CtrlSpec> {
        ControllerResetW::new(self, 0)
    }
    #[doc = "Bit 1 - Fifo reset."]
    #[inline(always)]
    pub fn fifo_reset(&mut self) -> FifoResetW<CtrlSpec> {
        FifoResetW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA reset."]
    #[inline(always)]
    pub fn dma_reset(&mut self) -> DmaResetW<CtrlSpec> {
        DmaResetW::new(self, 2)
    }
    #[doc = "Bit 4 - Global interrupt enable/disable bit."]
    #[inline(always)]
    pub fn int_enable(&mut self) -> IntEnableW<CtrlSpec> {
        IntEnableW::new(self, 4)
    }
    #[doc = "Bit 6 - Read/wait."]
    #[inline(always)]
    pub fn read_wait(&mut self) -> ReadWaitW<CtrlSpec> {
        ReadWaitW::new(self, 6)
    }
    #[doc = "Bit 7 - Send irq response."]
    #[inline(always)]
    pub fn send_irq_response(&mut self) -> SendIrqResponseW<CtrlSpec> {
        SendIrqResponseW::new(self, 7)
    }
    #[doc = "Bit 8 - Abort read data."]
    #[inline(always)]
    pub fn abort_read_data(&mut self) -> AbortReadDataW<CtrlSpec> {
        AbortReadDataW::new(self, 8)
    }
    #[doc = "Bit 9 - Send ccsd."]
    #[inline(always)]
    pub fn send_ccsd(&mut self) -> SendCcsdW<CtrlSpec> {
        SendCcsdW::new(self, 9)
    }
    #[doc = "Bit 10 - Send auto stop ccsd."]
    #[inline(always)]
    pub fn send_auto_stop_ccsd(&mut self) -> SendAutoStopCcsdW<CtrlSpec> {
        SendAutoStopCcsdW::new(self, 10)
    }
    #[doc = "Bit 11 - CEATA device interrupt status."]
    #[inline(always)]
    pub fn ceata_device_interrupt_status(&mut self) -> CeataDeviceInterruptStatusW<CtrlSpec> {
        CeataDeviceInterruptStatusW::new(self, 11)
    }
    #[doc = "Bit 16 - Controls the state of the SD_VOLT0 pin."]
    #[inline(always)]
    pub fn card_voltage_a0(&mut self) -> CardVoltageA0W<CtrlSpec> {
        CardVoltageA0W::new(self, 16)
    }
    #[doc = "Bit 17 - Controls the state of the SD_VOLT1 pin."]
    #[inline(always)]
    pub fn card_voltage_a1(&mut self) -> CardVoltageA1W<CtrlSpec> {
        CardVoltageA1W::new(self, 17)
    }
    #[doc = "Bit 18 - Controls the state of the SD_VOLT2 pin."]
    #[inline(always)]
    pub fn card_voltage_a2(&mut self) -> CardVoltageA2W<CtrlSpec> {
        CardVoltageA2W::new(self, 18)
    }
    #[doc = "Bit 25 - SD/MMC DMA use."]
    #[inline(always)]
    pub fn use_internal_dmac(&mut self) -> UseInternalDmacW<CtrlSpec> {
        UseInternalDmacW::new(self, 25)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
