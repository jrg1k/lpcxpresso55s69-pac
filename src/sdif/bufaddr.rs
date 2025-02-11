#[doc = "Register `BUFADDR` reader"]
pub type R = crate::R<BufaddrSpec>;
#[doc = "Register `BUFADDR` writer"]
pub type W = crate::W<BufaddrSpec>;
#[doc = "Field `HBA` reader - Host Buffer Address Pointer."]
pub type HbaR = crate::FieldReader<u32>;
#[doc = "Field `HBA` writer - Host Buffer Address Pointer."]
pub type HbaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    pub fn hba(&self) -> HbaR {
        HbaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Host Buffer Address Pointer."]
    #[inline(always)]
    pub fn hba(&mut self) -> HbaW<BufaddrSpec> {
        HbaW::new(self, 0)
    }
}
#[doc = "Current Buffer Descriptor Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`bufaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bufaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BufaddrSpec;
impl crate::RegisterSpec for BufaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bufaddr::R`](R) reader structure"]
impl crate::Readable for BufaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`bufaddr::W`](W) writer structure"]
impl crate::Writable for BufaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUFADDR to value 0"]
impl crate::Resettable for BufaddrSpec {
    const RESET_VALUE: u32 = 0;
}
