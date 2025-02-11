#[doc = "Register `DATABUFSTART` reader"]
pub type R = crate::R<DatabufstartSpec>;
#[doc = "Register `DATABUFSTART` writer"]
pub type W = crate::W<DatabufstartSpec>;
#[doc = "Field `DA_BUF` reader - Start address of the memory page where all endpoint data buffers are located."]
pub type DaBufR = crate::FieldReader<u32>;
#[doc = "Field `DA_BUF` writer - Start address of the memory page where all endpoint data buffers are located."]
pub type DaBufW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&self) -> DaBufR {
        DaBufR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address of the memory page where all endpoint data buffers are located."]
    #[inline(always)]
    pub fn da_buf(&mut self) -> DaBufW<DatabufstartSpec> {
        DaBufW::new(self, 0)
    }
}
#[doc = "USB Data buffer start address\n\nYou can [`read`](crate::Reg::read) this register and get [`databufstart::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`databufstart::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatabufstartSpec;
impl crate::RegisterSpec for DatabufstartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`databufstart::R`](R) reader structure"]
impl crate::Readable for DatabufstartSpec {}
#[doc = "`write(|w| ..)` method takes [`databufstart::W`](W) writer structure"]
impl crate::Writable for DatabufstartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATABUFSTART to value 0x4100_0000"]
impl crate::Resettable for DatabufstartSpec {
    const RESET_VALUE: u32 = 0x4100_0000;
}
