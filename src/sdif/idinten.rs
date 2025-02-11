#[doc = "Register `IDINTEN` reader"]
pub type R = crate::R<IdintenSpec>;
#[doc = "Register `IDINTEN` writer"]
pub type W = crate::W<IdintenSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt Enable."]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt Enable."]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt Enable."]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt Enable."]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error Enable."]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error Enable."]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DU` reader - Descriptor Unavailable Interrupt."]
pub type DuR = crate::BitReader;
#[doc = "Field `DU` writer - Descriptor Unavailable Interrupt."]
pub type DuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CES` reader - Card Error summary Interrupt Enable."]
pub type CesR = crate::BitReader;
#[doc = "Field `CES` writer - Card Error summary Interrupt Enable."]
pub type CesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary Enable."]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary Enable."]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary Enable."]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary Enable."]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&self) -> CesR {
        CesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable."]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<IdintenSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Interrupt Enable."]
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<IdintenSpec> {
        RiW::new(self, 1)
    }
    #[doc = "Bit 2 - Fatal Bus Error Enable."]
    #[inline(always)]
    pub fn fbe(&mut self) -> FbeW<IdintenSpec> {
        FbeW::new(self, 2)
    }
    #[doc = "Bit 4 - Descriptor Unavailable Interrupt."]
    #[inline(always)]
    pub fn du(&mut self) -> DuW<IdintenSpec> {
        DuW::new(self, 4)
    }
    #[doc = "Bit 5 - Card Error summary Interrupt Enable."]
    #[inline(always)]
    pub fn ces(&mut self) -> CesW<IdintenSpec> {
        CesW::new(self, 5)
    }
    #[doc = "Bit 8 - Normal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn nis(&mut self) -> NisW<IdintenSpec> {
        NisW::new(self, 8)
    }
    #[doc = "Bit 9 - Abnormal Interrupt Summary Enable."]
    #[inline(always)]
    pub fn ais(&mut self) -> AisW<IdintenSpec> {
        AisW::new(self, 9)
    }
}
#[doc = "Internal DMAC Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`idinten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idinten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdintenSpec;
impl crate::RegisterSpec for IdintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idinten::R`](R) reader structure"]
impl crate::Readable for IdintenSpec {}
#[doc = "`write(|w| ..)` method takes [`idinten::W`](W) writer structure"]
impl crate::Writable for IdintenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDINTEN to value 0"]
impl crate::Resettable for IdintenSpec {
    const RESET_VALUE: u32 = 0;
}
