#[doc = "Register `CPU1_LOCK_REG` reader"]
pub type R = crate::R<Cpu1LockRegSpec>;
#[doc = "Register `CPU1_LOCK_REG` writer"]
pub type W = crate::W<Cpu1LockRegSpec>;
#[doc = "micro-Cortex M33 (CPU1) VTOR_NS register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsVtor {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsVtor> for u8 {
    #[inline(always)]
    fn from(variant: LockNsVtor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsVtor {
    type Ux = u8;
}
impl crate::IsEnum for LockNsVtor {}
#[doc = "Field `LOCK_NS_VTOR` reader - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
pub type LockNsVtorR = crate::FieldReader<LockNsVtor>;
impl LockNsVtorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsVtor> {
        match self.bits {
            1 => Some(LockNsVtor::Blocked),
            2 => Some(LockNsVtor::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsVtor::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsVtor::Writable
    }
}
#[doc = "Field `LOCK_NS_VTOR` writer - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
pub type LockNsVtorW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsVtor>;
impl<'a, REG> LockNsVtorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsVtor::Writable)
    }
}
#[doc = "micro-Cortex M33 (CPU1) non-secure MPU register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LockNsMpu {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<LockNsMpu> for u8 {
    #[inline(always)]
    fn from(variant: LockNsMpu) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LockNsMpu {
    type Ux = u8;
}
impl crate::IsEnum for LockNsMpu {}
#[doc = "Field `LOCK_NS_MPU` reader - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
pub type LockNsMpuR = crate::FieldReader<LockNsMpu>;
impl LockNsMpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LockNsMpu> {
        match self.bits {
            1 => Some(LockNsMpu::Blocked),
            2 => Some(LockNsMpu::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == LockNsMpu::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == LockNsMpu::Writable
    }
}
#[doc = "Field `LOCK_NS_MPU` writer - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
pub type LockNsMpuW<'a, REG> = crate::FieldWriter<'a, REG, 2, LockNsMpu>;
impl<'a, REG> LockNsMpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(LockNsMpu::Writable)
    }
}
#[doc = "CPU1_LOCK_REG write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cpu1LockRegLock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<Cpu1LockRegLock> for u8 {
    #[inline(always)]
    fn from(variant: Cpu1LockRegLock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cpu1LockRegLock {
    type Ux = u8;
}
impl crate::IsEnum for Cpu1LockRegLock {}
#[doc = "Field `CPU1_LOCK_REG_LOCK` reader - CPU1_LOCK_REG write-lock."]
pub type Cpu1LockRegLockR = crate::FieldReader<Cpu1LockRegLock>;
impl Cpu1LockRegLockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cpu1LockRegLock> {
        match self.bits {
            1 => Some(Cpu1LockRegLock::Blocked),
            2 => Some(Cpu1LockRegLock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == Cpu1LockRegLock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == Cpu1LockRegLock::Writable
    }
}
#[doc = "Field `CPU1_LOCK_REG_LOCK` writer - CPU1_LOCK_REG write-lock."]
pub type Cpu1LockRegLockW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cpu1LockRegLock>;
impl<'a, REG> Cpu1LockRegLockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1LockRegLock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(Cpu1LockRegLock::Writable)
    }
}
impl R {
    #[doc = "Bits 0:1 - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&self) -> LockNsVtorR {
        LockNsVtorR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&self) -> LockNsMpuR {
        LockNsMpuR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 30:31 - CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu1_lock_reg_lock(&self) -> Cpu1LockRegLockR {
        Cpu1LockRegLockR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - micro-Cortex M33 (CPU1) VTOR_NS register write-lock."]
    #[inline(always)]
    pub fn lock_ns_vtor(&mut self) -> LockNsVtorW<Cpu1LockRegSpec> {
        LockNsVtorW::new(self, 0)
    }
    #[doc = "Bits 2:3 - micro-Cortex M33 (CPU1) non-secure MPU register write-lock."]
    #[inline(always)]
    pub fn lock_ns_mpu(&mut self) -> LockNsMpuW<Cpu1LockRegSpec> {
        LockNsMpuW::new(self, 2)
    }
    #[doc = "Bits 30:31 - CPU1_LOCK_REG write-lock."]
    #[inline(always)]
    pub fn cpu1_lock_reg_lock(&mut self) -> Cpu1LockRegLockW<Cpu1LockRegSpec> {
        Cpu1LockRegLockW::new(self, 30)
    }
}
#[doc = "Miscalleneous control signals for in micro-Cortex M33 (CPU1)\n\nYou can [`read`](crate::Reg::read) this register and get [`cpu1_lock_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpu1_lock_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpu1LockRegSpec;
impl crate::RegisterSpec for Cpu1LockRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpu1_lock_reg::R`](R) reader structure"]
impl crate::Readable for Cpu1LockRegSpec {}
#[doc = "`write(|w| ..)` method takes [`cpu1_lock_reg::W`](W) writer structure"]
impl crate::Writable for Cpu1LockRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CPU1_LOCK_REG to value 0x8000_000a"]
impl crate::Resettable for Cpu1LockRegSpec {
    const RESET_VALUE: u32 = 0x8000_000a;
}
