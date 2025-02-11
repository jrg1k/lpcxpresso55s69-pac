#[doc = "Register `BMOD` reader"]
pub type R = crate::R<BmodSpec>;
#[doc = "Register `BMOD` writer"]
pub type W = crate::W<BmodSpec>;
#[doc = "Field `SWR` reader - Software Reset."]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset."]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FB` reader - Fixed Burst."]
pub type FbR = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst."]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length."]
pub type DslR = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length."]
pub type DslW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DE` reader - SD/MMC DMA Enable."]
pub type DeR = crate::BitReader;
#[doc = "Field `DE` writer - SD/MMC DMA Enable."]
pub type DeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length."]
pub type PblR = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length."]
pub type PblW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Software Reset."]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fixed Burst."]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length."]
    #[inline(always)]
    pub fn dsl(&self) -> DslR {
        DslR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable."]
    #[inline(always)]
    pub fn de(&self) -> DeR {
        DeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length."]
    #[inline(always)]
    pub fn pbl(&self) -> PblR {
        PblR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset."]
    #[inline(always)]
    pub fn swr(&mut self) -> SwrW<BmodSpec> {
        SwrW::new(self, 0)
    }
    #[doc = "Bit 1 - Fixed Burst."]
    #[inline(always)]
    pub fn fb(&mut self) -> FbW<BmodSpec> {
        FbW::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length."]
    #[inline(always)]
    pub fn dsl(&mut self) -> DslW<BmodSpec> {
        DslW::new(self, 2)
    }
    #[doc = "Bit 7 - SD/MMC DMA Enable."]
    #[inline(always)]
    pub fn de(&mut self) -> DeW<BmodSpec> {
        DeW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Programmable Burst Length."]
    #[inline(always)]
    pub fn pbl(&mut self) -> PblW<BmodSpec> {
        PblW::new(self, 8)
    }
}
#[doc = "Bus Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmodSpec;
impl crate::RegisterSpec for BmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmod::R`](R) reader structure"]
impl crate::Readable for BmodSpec {}
#[doc = "`write(|w| ..)` method takes [`bmod::W`](W) writer structure"]
impl crate::Writable for BmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMOD to value 0"]
impl crate::Resettable for BmodSpec {
    const RESET_VALUE: u32 = 0;
}
