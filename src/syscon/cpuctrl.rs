#[doc = "Register `CPUCTRL` reader"]
pub type R = crate::R<CpuctrlSpec>;
#[doc = "Register `CPUCTRL` writer"]
pub type W = crate::W<CpuctrlSpec>;
#[doc = "CPU1 clock enable.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1clken {
    #[doc = "0: The CPU1 clock is not enabled."]
    Disable = 0,
    #[doc = "1: The CPU1 clock is enabled."]
    Enable = 1,
}
impl From<Cpu1clken> for bool {
    #[inline(always)]
    fn from(variant: Cpu1clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1CLKEN` reader - CPU1 clock enable."]
pub type Cpu1clkenR = crate::BitReader<Cpu1clken>;
impl Cpu1clkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1clken {
        match self.bits {
            false => Cpu1clken::Disable,
            true => Cpu1clken::Enable,
        }
    }
    #[doc = "The CPU1 clock is not enabled."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cpu1clken::Disable
    }
    #[doc = "The CPU1 clock is enabled."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Cpu1clken::Enable
    }
}
#[doc = "Field `CPU1CLKEN` writer - CPU1 clock enable."]
pub type Cpu1clkenW<'a, REG> = crate::BitWriter<'a, REG, Cpu1clken>;
impl<'a, REG> Cpu1clkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CPU1 clock is not enabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1clken::Disable)
    }
    #[doc = "The CPU1 clock is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1clken::Enable)
    }
}
#[doc = "CPU1 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1rsten {
    #[doc = "0: The CPU1 is not being reset."]
    Released = 0,
    #[doc = "1: The CPU1 is being reset."]
    Asserted = 1,
}
impl From<Cpu1rsten> for bool {
    #[inline(always)]
    fn from(variant: Cpu1rsten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1RSTEN` reader - CPU1 reset."]
pub type Cpu1rstenR = crate::BitReader<Cpu1rsten>;
impl Cpu1rstenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1rsten {
        match self.bits {
            false => Cpu1rsten::Released,
            true => Cpu1rsten::Asserted,
        }
    }
    #[doc = "The CPU1 is not being reset."]
    #[inline(always)]
    pub fn is_released(&self) -> bool {
        *self == Cpu1rsten::Released
    }
    #[doc = "The CPU1 is being reset."]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == Cpu1rsten::Asserted
    }
}
#[doc = "Field `CPU1RSTEN` writer - CPU1 reset."]
pub type Cpu1rstenW<'a, REG> = crate::BitWriter<'a, REG, Cpu1rsten>;
impl<'a, REG> Cpu1rstenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The CPU1 is not being reset."]
    #[inline(always)]
    pub fn released(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1rsten::Released)
    }
    #[doc = "The CPU1 is being reset."]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1rsten::Asserted)
    }
}
impl R {
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&self) -> Cpu1clkenR {
        Cpu1clkenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&self) -> Cpu1rstenR {
        Cpu1rstenR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - CPU1 clock enable."]
    #[inline(always)]
    pub fn cpu1clken(&mut self) -> Cpu1clkenW<CpuctrlSpec> {
        Cpu1clkenW::new(self, 3)
    }
    #[doc = "Bit 5 - CPU1 reset."]
    #[inline(always)]
    pub fn cpu1rsten(&mut self) -> Cpu1rstenW<CpuctrlSpec> {
        Cpu1rstenW::new(self, 5)
    }
}
#[doc = "CPU Control for multiple processors\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpuctrlSpec;
impl crate::RegisterSpec for CpuctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpuctrl::R`](R) reader structure"]
impl crate::Readable for CpuctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cpuctrl::W`](W) writer structure"]
impl crate::Writable for CpuctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPUCTRL to value 0x2c"]
impl crate::Resettable for CpuctrlSpec {
    const RESET_VALUE: u32 = 0x2c;
}
