#[doc = "Register `IDSTS` reader"]
pub type R = crate::R<IdstsSpec>;
#[doc = "Register `IDSTS` writer"]
pub type W = crate::W<IdstsSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt."]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt."]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Interrupt."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Interrupt."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt."]
pub type DuR = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt."]
pub type DuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error Summary."]
pub type CesR = crate::BitReader;
#[doc = "Field `CES` writer - Card Error Summary."]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary."]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary."]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary."]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary."]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EB` reader - Error Bits."]
pub type EbR = crate::FieldReader;
#[doc = "Field `EB` writer - Error Bits."]
pub type EbW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FSM` reader - DMAC state machine present state."]
pub type FsmR = crate::FieldReader;
#[doc = "Field `FSM` writer - DMAC state machine present state."]
pub type FsmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&self) -> EbR {
        EbR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&self) -> FsmR {
        FsmR::new(((self.bits >> 13) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt."]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<IdstsSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt."]
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<IdstsSpec> {
        RiW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Interrupt."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FbeW<IdstsSpec> {
        FbeW::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<IdstsSpec> {
        DuW::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error Summary."]
    #[inline(always)]
    pub fn ces(&mut self) -> CesW<IdstsSpec> {
        CesW::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary."]
    #[inline(always)]
    pub fn nis(&mut self) -> NisW<IdstsSpec> {
        NisW::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary."]
    #[inline(always)]
    pub fn ais(&mut self) -> AisW<IdstsSpec> {
        AisW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Error Bits."]
    #[inline(always)]
    pub fn eb(&mut self) -> EbW<IdstsSpec> {
        EbW::new(self, 10)
    }
    #[doc = "Bits 13:16 - DMAC state machine present state."]
    #[inline(always)]
    pub fn fsm(&mut self) -> FsmW<IdstsSpec> {
        FsmW::new(self, 13)
    }
}
#[doc = "Internal DMAC Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`idsts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idsts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdstsSpec;
impl crate::RegisterSpec for IdstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idsts::R`](R) reader structure"]
impl crate::Readable for IdstsSpec {}
#[doc = "`write(|w| ..)` method takes [`idsts::W`](W) writer structure"]
impl crate::Writable for IdstsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDSTS to value 0"]
impl crate::Resettable for IdstsSpec {
    const RESET_VALUE: u32 = 0;
}
