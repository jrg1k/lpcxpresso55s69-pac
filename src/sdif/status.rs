#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `FIFO_RX_WATERMARK` reader - FIFO reached Receive watermark level; not qualified with data transfer."]
pub type FifoRxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_RX_WATERMARK` writer - FIFO reached Receive watermark level; not qualified with data transfer."]
pub type FifoRxWatermarkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_TX_WATERMARK` reader - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub type FifoTxWatermarkR = crate::BitReader;
#[doc = "Field `FIFO_TX_WATERMARK` writer - FIFO reached Transmit watermark level; not qualified with data transfer."]
pub type FifoTxWatermarkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_EMPTY` reader - FIFO is empty status."]
pub type FifoEmptyR = crate::BitReader;
#[doc = "Field `FIFO_EMPTY` writer - FIFO is empty status."]
pub type FifoEmptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FULL` reader - FIFO is full status."]
pub type FifoFullR = crate::BitReader;
#[doc = "Field `FIFO_FULL` writer - FIFO is full status."]
pub type FifoFullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDFSMSTATES` reader - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub type CmdfsmstatesR = crate::FieldReader;
#[doc = "Field `CMDFSMSTATES` writer - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
pub type CmdfsmstatesW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DATA_3_STATUS` reader - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub type Data3StatusR = crate::BitReader;
#[doc = "Field `DATA_3_STATUS` writer - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
pub type Data3StatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_BUSY` reader - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub type DataBusyR = crate::BitReader;
#[doc = "Field `DATA_BUSY` writer - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
pub type DataBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_STATE_MC_BUSY` reader - Data transmit or receive state-machine is busy."]
pub type DataStateMcBusyR = crate::BitReader;
#[doc = "Field `DATA_STATE_MC_BUSY` writer - Data transmit or receive state-machine is busy."]
pub type DataStateMcBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESPONSE_INDEX` reader - Index of previous response, including any auto-stop sent by core."]
pub type ResponseIndexR = crate::FieldReader;
#[doc = "Field `RESPONSE_INDEX` writer - Index of previous response, including any auto-stop sent by core."]
pub type ResponseIndexW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `FIFO_COUNT` reader - FIFO count - Number of filled locations in FIFO."]
pub type FifoCountR = crate::FieldReader<u16>;
#[doc = "Field `FIFO_COUNT` writer - FIFO count - Number of filled locations in FIFO."]
pub type FifoCountW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DMA_ACK` reader - DMA acknowledge signal state."]
pub type DmaAckR = crate::BitReader;
#[doc = "Field `DMA_ACK` writer - DMA acknowledge signal state."]
pub type DmaAckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_REQ` reader - DMA request signal state."]
pub type DmaReqR = crate::BitReader;
#[doc = "Field `DMA_REQ` writer - DMA request signal state."]
pub type DmaReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&self) -> FifoRxWatermarkR {
        FifoRxWatermarkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&self) -> FifoTxWatermarkR {
        FifoTxWatermarkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FifoEmptyR {
        FifoEmptyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FifoFullR {
        FifoFullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&self) -> CmdfsmstatesR {
        CmdfsmstatesR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&self) -> Data3StatusR {
        Data3StatusR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&self) -> DataBusyR {
        DataBusyR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&self) -> DataStateMcBusyR {
        DataStateMcBusyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&self) -> ResponseIndexR {
        ResponseIndexR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&self) -> FifoCountR {
        FifoCountR::new(((self.bits >> 17) & 0x1fff) as u16)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&self) -> DmaAckR {
        DmaAckR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&self) -> DmaReqR {
        DmaReqR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO reached Receive watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_rx_watermark(&mut self) -> FifoRxWatermarkW<StatusSpec> {
        FifoRxWatermarkW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO reached Transmit watermark level; not qualified with data transfer."]
    #[inline(always)]
    pub fn fifo_tx_watermark(&mut self) -> FifoTxWatermarkW<StatusSpec> {
        FifoTxWatermarkW::new(self, 1)
    }
    #[doc = "Bit 2 - FIFO is empty status."]
    #[inline(always)]
    pub fn fifo_empty(&mut self) -> FifoEmptyW<StatusSpec> {
        FifoEmptyW::new(self, 2)
    }
    #[doc = "Bit 3 - FIFO is full status."]
    #[inline(always)]
    pub fn fifo_full(&mut self) -> FifoFullW<StatusSpec> {
        FifoFullW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Command FSM states: 0 - Idle 1 - Send init sequence 2 - Tx cmd start bit 3 - Tx cmd tx bit 4 - Tx cmd index + arg 5 - Tx cmd crc7 6 - Tx cmd end bit 7 - Rx resp start bit 8 - Rx resp IRQ response 9 - Rx resp tx bit 10 - Rx resp cmd idx 11 - Rx resp data 12 - Rx resp crc7 13 - Rx resp end bit 14 - Cmd path wait NCC 15 - Wait; CMD-to-response turnaround NOTE: The command FSM state is represented using 19 bits."]
    #[inline(always)]
    pub fn cmdfsmstates(&mut self) -> CmdfsmstatesW<StatusSpec> {
        CmdfsmstatesW::new(self, 4)
    }
    #[doc = "Bit 8 - Raw selected card_data\\[3\\]; checks whether card is present 0 - card not present 1 - card present."]
    #[inline(always)]
    pub fn data_3_status(&mut self) -> Data3StatusW<StatusSpec> {
        Data3StatusW::new(self, 8)
    }
    #[doc = "Bit 9 - Inverted version of raw selected card_data\\[0\\]
0 - card data not busy 1 - card data busy."]
    #[inline(always)]
    pub fn data_busy(&mut self) -> DataBusyW<StatusSpec> {
        DataBusyW::new(self, 9)
    }
    #[doc = "Bit 10 - Data transmit or receive state-machine is busy."]
    #[inline(always)]
    pub fn data_state_mc_busy(&mut self) -> DataStateMcBusyW<StatusSpec> {
        DataStateMcBusyW::new(self, 10)
    }
    #[doc = "Bits 11:16 - Index of previous response, including any auto-stop sent by core."]
    #[inline(always)]
    pub fn response_index(&mut self) -> ResponseIndexW<StatusSpec> {
        ResponseIndexW::new(self, 11)
    }
    #[doc = "Bits 17:29 - FIFO count - Number of filled locations in FIFO."]
    #[inline(always)]
    pub fn fifo_count(&mut self) -> FifoCountW<StatusSpec> {
        FifoCountW::new(self, 17)
    }
    #[doc = "Bit 30 - DMA acknowledge signal state."]
    #[inline(always)]
    pub fn dma_ack(&mut self) -> DmaAckW<StatusSpec> {
        DmaAckW::new(self, 30)
    }
    #[doc = "Bit 31 - DMA request signal state."]
    #[inline(always)]
    pub fn dma_req(&mut self) -> DmaReqW<StatusSpec> {
        DmaReqW::new(self, 31)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATUS to value 0x0406"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x0406;
}
