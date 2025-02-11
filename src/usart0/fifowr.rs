#[doc = "Register `FIFOWR` writer"]
pub type W = crate::W<FifowrSpec>;
#[doc = "Field `TXDATA` writer - Transmit data to the FIFO."]
pub type TxdataW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl W {
    #[doc = "Bits 0:8 - Transmit data to the FIFO."]
    #[inline(always)]
    pub fn txdata(&mut self) -> TxdataW<FifowrSpec> {
        TxdataW::new(self, 0)
    }
}
#[doc = "FIFO write data.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifowr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifowrSpec;
impl crate::RegisterSpec for FifowrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fifowr::W`](W) writer structure"]
impl crate::Writable for FifowrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FIFOWR to value 0"]
impl crate::Resettable for FifowrSpec {
    const RESET_VALUE: u32 = 0;
}
