#[doc = "Register `INTMASK` reader"]
pub type R = crate::R<IntmaskSpec>;
#[doc = "Register `INTMASK` writer"]
pub type W = crate::W<IntmaskSpec>;
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
#[doc = "Field `RTO` reader - Response time-out."]
pub type RtoR = crate::BitReader;
#[doc = "Field `RTO` writer - Response time-out."]
pub type RtoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRTO` reader - Data read time-out."]
pub type DrtoR = crate::BitReader;
#[doc = "Field `DRTO` writer - Data read time-out."]
pub type DrtoW<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `EBE` reader - End-bit error (read)/Write no CRC."]
pub type EbeR = crate::BitReader;
#[doc = "Field `EBE` writer - End-bit error (read)/Write no CRC."]
pub type EbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_INT_MASK` reader - Mask SDIO interrupt."]
pub type SdioIntMaskR = crate::BitReader;
#[doc = "Field `SDIO_INT_MASK` writer - Mask SDIO interrupt."]
pub type SdioIntMaskW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&self) -> RtoR {
        RtoR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&self) -> DrtoR {
        DrtoR::new(((self.bits >> 9) & 1) != 0)
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
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub fn ebe(&self) -> EbeR {
        EbeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&self) -> SdioIntMaskR {
        SdioIntMaskR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Card detect."]
    #[inline(always)]
    pub fn cdet(&mut self) -> CdetW<IntmaskSpec> {
        CdetW::new(self, 0)
    }
    #[doc = "Bit 1 - Response error."]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<IntmaskSpec> {
        ReW::new(self, 1)
    }
    #[doc = "Bit 2 - Command done."]
    #[inline(always)]
    pub fn cdone(&mut self) -> CdoneW<IntmaskSpec> {
        CdoneW::new(self, 2)
    }
    #[doc = "Bit 3 - Data transfer over."]
    #[inline(always)]
    pub fn dto(&mut self) -> DtoW<IntmaskSpec> {
        DtoW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit FIFO data request."]
    #[inline(always)]
    pub fn txdr(&mut self) -> TxdrW<IntmaskSpec> {
        TxdrW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO data request."]
    #[inline(always)]
    pub fn rxdr(&mut self) -> RxdrW<IntmaskSpec> {
        RxdrW::new(self, 5)
    }
    #[doc = "Bit 6 - Response CRC error."]
    #[inline(always)]
    pub fn rcrc(&mut self) -> RcrcW<IntmaskSpec> {
        RcrcW::new(self, 6)
    }
    #[doc = "Bit 7 - Data CRC error."]
    #[inline(always)]
    pub fn dcrc(&mut self) -> DcrcW<IntmaskSpec> {
        DcrcW::new(self, 7)
    }
    #[doc = "Bit 8 - Response time-out."]
    #[inline(always)]
    pub fn rto(&mut self) -> RtoW<IntmaskSpec> {
        RtoW::new(self, 8)
    }
    #[doc = "Bit 9 - Data read time-out."]
    #[inline(always)]
    pub fn drto(&mut self) -> DrtoW<IntmaskSpec> {
        DrtoW::new(self, 9)
    }
    #[doc = "Bit 10 - Data starvation-by-host time-out (HTO)."]
    #[inline(always)]
    pub fn hto(&mut self) -> HtoW<IntmaskSpec> {
        HtoW::new(self, 10)
    }
    #[doc = "Bit 11 - FIFO underrun/overrun error."]
    #[inline(always)]
    pub fn frun(&mut self) -> FrunW<IntmaskSpec> {
        FrunW::new(self, 11)
    }
    #[doc = "Bit 12 - Hardware locked write error."]
    #[inline(always)]
    pub fn hle(&mut self) -> HleW<IntmaskSpec> {
        HleW::new(self, 12)
    }
    #[doc = "Bit 13 - Start-bit error."]
    #[inline(always)]
    pub fn sbe(&mut self) -> SbeW<IntmaskSpec> {
        SbeW::new(self, 13)
    }
    #[doc = "Bit 14 - Auto command done."]
    #[inline(always)]
    pub fn acd(&mut self) -> AcdW<IntmaskSpec> {
        AcdW::new(self, 14)
    }
    #[doc = "Bit 15 - End-bit error (read)/Write no CRC."]
    #[inline(always)]
    pub fn ebe(&mut self) -> EbeW<IntmaskSpec> {
        EbeW::new(self, 15)
    }
    #[doc = "Bit 16 - Mask SDIO interrupt."]
    #[inline(always)]
    pub fn sdio_int_mask(&mut self) -> SdioIntMaskW<IntmaskSpec> {
        SdioIntMaskW::new(self, 16)
    }
}
#[doc = "Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmaskSpec;
impl crate::RegisterSpec for IntmaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmask::R`](R) reader structure"]
impl crate::Readable for IntmaskSpec {}
#[doc = "`write(|w| ..)` method takes [`intmask::W`](W) writer structure"]
impl crate::Writable for IntmaskSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTMASK to value 0"]
impl crate::Resettable for IntmaskSpec {
    const RESET_VALUE: u32 = 0;
}
