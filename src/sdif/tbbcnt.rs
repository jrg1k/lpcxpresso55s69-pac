#[doc = "Register `TBBCNT` reader"]
pub type R = crate::R<TbbcntSpec>;
#[doc = "Register `TBBCNT` writer"]
pub type W = crate::W<TbbcntSpec>;
#[doc = "Field `TRANS_FIFO_BYTE_COUNT` reader - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
pub type TransFifoByteCountR = crate::FieldReader<u32>;
#[doc = "Field `TRANS_FIFO_BYTE_COUNT` writer - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
pub type TransFifoByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn trans_fifo_byte_count(&self) -> TransFifoByteCountR {
        TransFifoByteCountR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of bytes transferred between Host/DMA memory and BIU FIFO."]
    #[inline(always)]
    pub fn trans_fifo_byte_count(&mut self) -> TransFifoByteCountW<TbbcntSpec> {
        TransFifoByteCountW::new(self, 0)
    }
}
#[doc = "Transferred Host to BIU-FIFO Byte Count register\n\nYou can [`read`](crate::Reg::read) this register and get [`tbbcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbbcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbbcntSpec;
impl crate::RegisterSpec for TbbcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tbbcnt::R`](R) reader structure"]
impl crate::Readable for TbbcntSpec {}
#[doc = "`write(|w| ..)` method takes [`tbbcnt::W`](W) writer structure"]
impl crate::Writable for TbbcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TBBCNT to value 0"]
impl crate::Resettable for TbbcntSpec {
    const RESET_VALUE: u32 = 0;
}
