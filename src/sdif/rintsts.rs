#[doc = "Register `RINTSTS` reader"]
pub type R = crate::R<RintstsSpec>;
#[doc = "Register `RINTSTS` writer"]
pub type W = crate::W<RintstsSpec>;
#[doc = "Field `CDET` reader - Card detect."]
pub type CdetR = crate::BitReader;
#[doc = "Field `CDET` writer - Card detect."]
pub type CdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RE` reader - Response error."]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Response error."]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDONE` reader - Command done."]
pub type CdoneR = crate::BitReader;
#[doc = "Field `CDONE` writer - Command done."]
pub type CdoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTO` reader - Data transfer over."]
pub type DtoR = crate::BitReader;
#[doc = "Field `DTO` writer - Data transfer over."]
pub type DtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDR` reader - Transmit FIFO data request."]
pub type TxdrR = crate::BitReader;
#[doc = "Field `TXDR` writer - Transmit FIFO data request."]
pub type TxdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXDR` reader - Receive FIFO data request."]
pub type RxdrR = crate::BitReader;
#[doc = "Field `RXDR` writer - Receive FIFO data request."]
pub type RxdrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRC` reader - Response CRC error."]
pub type RcrcR = crate::BitReader;
#[doc = "Field `RCRC` writer - Response CRC error."]
pub type RcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRC` reader - Data CRC error."]
pub type DcrcR = crate::BitReader;
#[doc = "Field `DCRC` writer - Data CRC error."]
pub type DcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTO_BAR` reader - Response time-out (RTO)/Boot Ack Received (BAR)."]
pub type RtoBarR = crate::BitReader;
#[doc = "Field `RTO_BAR` writer - Response time-out (RTO)/Boot Ack Received (BAR)."]
pub type RtoBarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRTO_BDS` reader - Data read time-out (DRTO)/Boot Data Start (BDS)."]
pub type DrtoBdsR = crate::BitReader;
#[doc = "Field `DRTO_BDS` writer - Data read time-out (DRTO)/Boot Data Start (BDS)."]
pub type DrtoBdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTO` reader - Data starvation-by-host time-out (HTO)."]
pub type HtoR = crate::BitReader;
#[doc = "Field `HTO` writer - Data starvation-by-host time-out (HTO)."]
pub type HtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRUN` reader - FIFO underrun/overrun error."]
pub type FrunR = crate::BitReader;
#[doc = "Field `FRUN` writer - FIFO underrun/overrun error."]
pub type FrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HLE` reader - Hardware locked write error."]
pub type HleR = crate::BitReader;
#[doc = "Field `HLE` writer - Hardware locked write error."]
pub type HleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBE` reader - Start-bit error."]
pub type SbeR = crate::BitReader;
#[doc = "Field `SBE` writer - Start-bit error."]
pub type SbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACD` reader - Auto command done."]
pub type AcdR = crate::BitReader;
#[doc = "Field `ACD` writer - Auto command done."]
pub type AcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EBE` reader - End-bit error (read)/write no CRC."]
pub type EbeR = crate::BitReader;
#[doc = "Field `EBE` writer - End-bit error (read)/write no CRC."]
pub type EbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_INTERRUPT` reader - Interrupt from SDIO card."]
pub type SdioInterruptR = crate::BitReader;
#[doc = "Field `SDIO_INTERRUPT` writer - Interrupt from SDIO card."]
pub type SdioInterruptW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&self) -> CdetR {
        CdetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&self) -> CdoneR {
        CdoneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&self) -> DtoR {
        DtoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&self) -> TxdrR {
        TxdrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&self) -> RxdrR {
        RxdrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&self) -> RcrcR {
        RcrcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&self) -> DcrcR {
        DcrcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[inline(always)]
    pub fn rto_bar(&self) -> RtoBarR {
        RtoBarR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[inline(always)]
    pub fn drto_bds(&self) -> DrtoBdsR {
        DrtoBdsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&self) -> HtoR {
        HtoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&self) -> FrunR {
        FrunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&self) -> HleR {
        HleR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&self) -> SbeR {
        SbeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&self) -> AcdR {
        AcdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub fn ebe(&self) -> EbeR {
        EbeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt from SDIO card."]
    #[inline(always)]
    pub fn sdio_interrupt(&self) -> SdioInterruptR {
        SdioInterruptR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&mut self) -> CdetW<RintstsSpec> {
        CdetW::new(self, 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<RintstsSpec> {
        ReW::new(self, 1)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&mut self) -> CdoneW<RintstsSpec> {
        CdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&mut self) -> DtoW<RintstsSpec> {
        DtoW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&mut self) -> TxdrW<RintstsSpec> {
        TxdrW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RxdrW<RintstsSpec> {
        RxdrW::new(self, 5)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&mut self) -> RcrcW<RintstsSpec> {
        RcrcW::new(self, 6)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&mut self) -> DcrcW<RintstsSpec> {
        DcrcW::new(self, 7)
    }
    #[doc = "Bit 8 - Response time-out (RTO)/Boot Ack Received (BAR)."]
    #[inline(always)]
    pub fn rto_bar(&mut self) -> RtoBarW<RintstsSpec> {
        RtoBarW::new(self, 8)
    }
    #[doc = "Bit 9 - Data read time-out (DRTO)/Boot Data Start (BDS)."]
    #[inline(always)]
    pub fn drto_bds(&mut self) -> DrtoBdsW<RintstsSpec> {
        DrtoBdsW::new(self, 9)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&mut self) -> HtoW<RintstsSpec> {
        HtoW::new(self, 10)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&mut self) -> FrunW<RintstsSpec> {
        FrunW::new(self, 11)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&mut self) -> HleW<RintstsSpec> {
        HleW::new(self, 12)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> SbeW<RintstsSpec> {
        SbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&mut self) -> AcdW<RintstsSpec> {
        AcdW::new(self, 14)
    }
    #[doc = "Bit 15 - End-bit error (read)/write no CRC."]
    #[inline(always)]
    pub fn ebe(&mut self) -> EbeW<RintstsSpec> {
        EbeW::new(self, 15)
    }
    #[doc = "Bit 16 - Interrupt from SDIO card."]
    #[inline(always)]
    pub fn sdio_interrupt(&mut self) -> SdioInterruptW<RintstsSpec> {
        SdioInterruptW::new(self, 16)
    }
}
#[doc = "Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rintsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rintsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RintstsSpec;
impl crate::RegisterSpec for RintstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rintsts::R`](R) reader structure"]
impl crate::Readable for RintstsSpec {}
#[doc = "`write(|w| ..)` method takes [`rintsts::W`](W) writer structure"]
impl crate::Writable for RintstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RINTSTS to value 0"]
impl crate::Resettable for RintstsSpec {
    const RESET_VALUE: u32 = 0;
}
