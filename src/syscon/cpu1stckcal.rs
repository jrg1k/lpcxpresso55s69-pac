#[doc = "Register `CPU1STCKCAL` reader"]
pub type R = crate::R<Cpu1stckcalSpec>;
#[doc = "Register `CPU1STCKCAL` writer"]
pub type W = crate::W<Cpu1stckcalSpec>;
#[doc = "Field `TENMS` reader - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TenmsR = crate::FieldReader<u32>;
#[doc = "Field `TENMS` writer - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
pub type TenmsW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `SKEW` reader - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
pub type SkewR = crate::BitReader;
#[doc = "Field `SKEW` writer - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
pub type SkewW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOREF` reader - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub type NorefR = crate::BitReader;
#[doc = "Field `NOREF` writer - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
pub type NorefW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&self) -> TenmsR {
        TenmsR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn skew(&self) -> SkewR {
        SkewR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&self) -> NorefR {
        NorefR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - Reload value for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as zero, the calibration value is not known."]
    #[inline(always)]
    pub fn tenms(&mut self) -> TenmsW<Cpu1stckcalSpec> {
        TenmsW::new(self, 0)
    }
    #[doc = "Bit 24 - Indicates whether the TENMS value is exact: 0 = TENMS value is exact; 1 = TENMS value is inexact, or not given."]
    #[inline(always)]
    pub fn skew(&mut self) -> SkewW<Cpu1stckcalSpec> {
        SkewW::new(self, 24)
    }
    #[doc = "Bit 25 - Indicates whether the device provides a reference clock to the processor: 0 = reference clock provided; 1 = no reference clock provided."]
    #[inline(always)]
    pub fn noref(&mut self) -> NorefW<Cpu1stckcalSpec> {
        NorefW::new(self, 25)
    }
}
#[doc = "System tick calibration for CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1stckcal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1stckcal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1stckcalSpec;
impl crate::RegisterSpec for Cpu1stckcalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1stckcal::R`](R) reader structure"]
impl crate::Readable for Cpu1stckcalSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1stckcal::W`](W) writer structure"]
impl crate::Writable for Cpu1stckcalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU1STCKCAL to value 0"]
impl crate::Resettable for Cpu1stckcalSpec {
    const RESET_VALUE: u32 = 0;
}
