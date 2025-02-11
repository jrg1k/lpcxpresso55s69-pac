#[doc = "Register `FIFOTH` reader"]
pub type R = crate::R<FifothSpec>;
#[doc = "Register `FIFOTH` writer"]
pub type W = crate::W<FifothSpec>;
#[doc = "Field `TX_WMARK` reader - FIFO threshold watermark level when transmitting data to card."]
pub type TxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `TX_WMARK` writer - FIFO threshold watermark level when transmitting data to card."]
pub type TxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `RX_WMARK` reader - FIFO threshold watermark level when receiving data to card."]
pub type RxWmarkR = crate::FieldReader<u16>;
#[doc = "Field `RX_WMARK` writer - FIFO threshold watermark level when receiving data to card."]
pub type RxWmarkW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMA_MTS` reader - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub type DmaMtsR = crate::FieldReader;
#[doc = "Field `DMA_MTS` writer - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
pub type DmaMtsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&self) -> TxWmarkR {
        TxWmarkR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&self) -> RxWmarkR {
        RxWmarkR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&self) -> DmaMtsR {
        DmaMtsR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - FIFO threshold watermark level when transmitting data to card."]
    #[inline(always)]
    pub fn tx_wmark(&mut self) -> TxWmarkW<FifothSpec> {
        TxWmarkW::new(self, 0)
    }
    #[doc = "Bits 16:27 - FIFO threshold watermark level when receiving data to card."]
    #[inline(always)]
    pub fn rx_wmark(&mut self) -> RxWmarkW<FifothSpec> {
        RxWmarkW::new(self, 16)
    }
    #[doc = "Bits 28:30 - Burst size of multiple transaction; should be programmed same as DW-DMA controller multiple-transaction-size SRC/DEST_MSIZE."]
    #[inline(always)]
    pub fn dma_mts(&mut self) -> DmaMtsW<FifothSpec> {
        DmaMtsW::new(self, 28)
    }
}
#[doc = "FIFO Threshold Watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`fifoth::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifoth::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifothSpec;
impl crate::RegisterSpec for FifothSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifoth::R`](R) reader structure"]
impl crate::Readable for FifothSpec {}
#[doc = "`write(|w| ..)` method takes [`fifoth::W`](W) writer structure"]
impl crate::Writable for FifothSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOTH to value 0x001f_0000"]
impl crate::Resettable for FifothSpec {
    const RESET_VALUE: u32 = 0x001f_0000;
}
