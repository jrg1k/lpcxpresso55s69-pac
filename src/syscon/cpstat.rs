#[doc = "Register `CPSTAT` reader"]
pub type R = crate::R<CpstatSpec>;
#[doc = "Register `CPSTAT` writer"]
pub type W = crate::W<CpstatSpec>;
#[doc = "The CPU0 sleeping state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0sleeping {
    #[doc = "0: the CPU is not sleeping."]
    Awake = 0,
    #[doc = "1: the CPU is sleeping."]
    Sleeping = 1,
}
impl From<Cpu0sleeping> for bool {
    #[inline(always)]
    fn from(variant: Cpu0sleeping) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0SLEEPING` reader - The CPU0 sleeping state."]
pub type Cpu0sleepingR = crate::BitReader<Cpu0sleeping>;
impl Cpu0sleepingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0sleeping {
        match self.bits {
            false => Cpu0sleeping::Awake,
            true => Cpu0sleeping::Sleeping,
        }
    }
    #[doc = "the CPU is not sleeping."]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == Cpu0sleeping::Awake
    }
    #[doc = "the CPU is sleeping."]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Cpu0sleeping::Sleeping
    }
}
#[doc = "The CPU1 sleeping state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1sleeping {
    #[doc = "0: the CPU is not sleeping."]
    Awake = 0,
    #[doc = "1: the CPU is sleeping."]
    Sleeping = 1,
}
impl From<Cpu1sleeping> for bool {
    #[inline(always)]
    fn from(variant: Cpu1sleeping) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1SLEEPING` reader - The CPU1 sleeping state."]
pub type Cpu1sleepingR = crate::BitReader<Cpu1sleeping>;
impl Cpu1sleepingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1sleeping {
        match self.bits {
            false => Cpu1sleeping::Awake,
            true => Cpu1sleeping::Sleeping,
        }
    }
    #[doc = "the CPU is not sleeping."]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == Cpu1sleeping::Awake
    }
    #[doc = "the CPU is sleeping."]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Cpu1sleeping::Sleeping
    }
}
#[doc = "The CPU0 lockup state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu0lockup {
    #[doc = "0: the CPU is not in lockup."]
    Awake = 0,
    #[doc = "1: the CPU is in lockup."]
    Sleeping = 1,
}
impl From<Cpu0lockup> for bool {
    #[inline(always)]
    fn from(variant: Cpu0lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU0LOCKUP` reader - The CPU0 lockup state."]
pub type Cpu0lockupR = crate::BitReader<Cpu0lockup>;
impl Cpu0lockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu0lockup {
        match self.bits {
            false => Cpu0lockup::Awake,
            true => Cpu0lockup::Sleeping,
        }
    }
    #[doc = "the CPU is not in lockup."]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == Cpu0lockup::Awake
    }
    #[doc = "the CPU is in lockup."]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Cpu0lockup::Sleeping
    }
}
#[doc = "The CPU1 lockup state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpu1lockup {
    #[doc = "0: the CPU is not in lockup."]
    Awake = 0,
    #[doc = "1: the CPU is in lockup."]
    Sleeping = 1,
}
impl From<Cpu1lockup> for bool {
    #[inline(always)]
    fn from(variant: Cpu1lockup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPU1LOCKUP` reader - The CPU1 lockup state."]
pub type Cpu1lockupR = crate::BitReader<Cpu1lockup>;
impl Cpu1lockupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpu1lockup {
        match self.bits {
            false => Cpu1lockup::Awake,
            true => Cpu1lockup::Sleeping,
        }
    }
    #[doc = "the CPU is not in lockup."]
    #[inline(always)]
    pub fn is_awake(&self) -> bool {
        *self == Cpu1lockup::Awake
    }
    #[doc = "the CPU is in lockup."]
    #[inline(always)]
    pub fn is_sleeping(&self) -> bool {
        *self == Cpu1lockup::Sleeping
    }
}
impl R {
    #[doc = "Bit 0 - The CPU0 sleeping state."]
    #[inline(always)]
    pub fn cpu0sleeping(&self) -> Cpu0sleepingR {
        Cpu0sleepingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The CPU1 sleeping state."]
    #[inline(always)]
    pub fn cpu1sleeping(&self) -> Cpu1sleepingR {
        Cpu1sleepingR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The CPU0 lockup state."]
    #[inline(always)]
    pub fn cpu0lockup(&self) -> Cpu0lockupR {
        Cpu0lockupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The CPU1 lockup state."]
    #[inline(always)]
    pub fn cpu1lockup(&self) -> Cpu1lockupR {
        Cpu1lockupR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {}
#[doc = "CPU Status\n\nYou can [`read`](crate::Reg::read) this register and get [`cpstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpstatSpec;
impl crate::RegisterSpec for CpstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpstat::R`](R) reader structure"]
impl crate::Readable for CpstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cpstat::W`](W) writer structure"]
impl crate::Writable for CpstatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPSTAT to value 0"]
impl crate::Resettable for CpstatSpec {
    const RESET_VALUE: u32 = 0;
}
