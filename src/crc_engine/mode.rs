#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Field `CRC_POLY` reader - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
pub type CrcPolyR = crate::FieldReader;
#[doc = "Field `CRC_POLY` writer - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
pub type CrcPolyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_RVS_WR` reader - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
pub type BitRvsWrR = crate::BitReader;
#[doc = "Field `BIT_RVS_WR` writer - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
pub type BitRvsWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPL_WR` reader - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
pub type CmplWrR = crate::BitReader;
#[doc = "Field `CMPL_WR` writer - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
pub type CmplWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RVS_SUM` reader - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
pub type BitRvsSumR = crate::BitReader;
#[doc = "Field `BIT_RVS_SUM` writer - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
pub type BitRvsSumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPL_SUM` reader - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
pub type CmplSumR = crate::BitReader;
#[doc = "Field `CMPL_SUM` writer - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
pub type CmplSumW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CrcPolyR {
        CrcPolyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BitRvsWrR {
        BitRvsWrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CmplWrR {
        CmplWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BitRvsSumR {
        BitRvsSumR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CmplSumR {
        CmplSumR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CRC polynomial: 1X = CRC-32 polynomial 01 = CRC-16 polynomial 00 = CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> CrcPolyW<ModeSpec> {
        CrcPolyW::new(self, 0)
    }
    #[doc = "Bit 2 - Data bit order: 1 = Bit order reverse for CRC_WR_DATA (per byte) 0 = No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn bit_rvs_wr(&mut self) -> BitRvsWrW<ModeSpec> {
        BitRvsWrW::new(self, 2)
    }
    #[doc = "Bit 3 - Data complement: 1 = 1's complement for CRC_WR_DATA 0 = No 1's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&mut self) -> CmplWrW<ModeSpec> {
        CmplWrW::new(self, 3)
    }
    #[doc = "Bit 4 - CRC sum bit order: 1 = Bit order reverse for CRC_SUM 0 = No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> BitRvsSumW<ModeSpec> {
        BitRvsSumW::new(self, 4)
    }
    #[doc = "Bit 5 - CRC sum complement: 1 = 1's complement for CRC_SUM 0 = No 1's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> CmplSumW<ModeSpec> {
        CmplSumW::new(self, 5)
    }
}
#[doc = "CRC mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {
    const RESET_VALUE: u32 = 0;
}
