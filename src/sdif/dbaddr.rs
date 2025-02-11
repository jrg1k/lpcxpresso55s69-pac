#[doc = "Register `DBADDR` reader"]
pub type R = crate::R<DbaddrSpec>;
#[doc = "Register `DBADDR` writer"]
pub type W = crate::W<DbaddrSpec>;
#[doc = "Field `SDL` reader - Start of Descriptor List."]
pub type SdlR = crate::FieldReader<u32>;
#[doc = "Field `SDL` writer - Start of Descriptor List."]
pub type SdlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Descriptor List."]
    #[inline(always)]
    pub fn sdl(&self) -> SdlR {
        SdlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Descriptor List."]
    #[inline(always)]
    pub fn sdl(&mut self) -> SdlW<DbaddrSpec> {
        SdlW::new(self, 0)
    }
}
#[doc = "Descriptor List Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`dbaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DbaddrSpec;
impl crate::RegisterSpec for DbaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbaddr::R`](R) reader structure"]
impl crate::Readable for DbaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`dbaddr::W`](W) writer structure"]
impl crate::Writable for DbaddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBADDR to value 0"]
impl crate::Resettable for DbaddrSpec {
    const RESET_VALUE: u32 = 0;
}
