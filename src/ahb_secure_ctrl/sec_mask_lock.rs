#[doc = "Register `SEC_MASK_LOCK` reader"]
pub type R = crate::R<SecMaskLockSpec>;
#[doc = "Register `SEC_MASK_LOCK` writer"]
pub type W = crate::W<SecMaskLockSpec>;
#[doc = "SEC_GPIO_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask0Lock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask0Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask0Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask0Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask0Lock {}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` reader - SEC_GPIO_MASK0 register write-lock."]
pub type SecGpioMask0LockR = crate::FieldReader<SecGpioMask0Lock>;
impl SecGpioMask0LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask0Lock> {
        match self.bits {
            1 => Some(SecGpioMask0Lock::Blocked),
            2 => Some(SecGpioMask0Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask0Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask0Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK0_LOCK` writer - SEC_GPIO_MASK0 register write-lock."]
pub type SecGpioMask0LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask0Lock>;
impl<'a, REG> SecGpioMask0LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask0Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask0Lock::Writable)
    }
}
#[doc = "SEC_GPIO_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecGpioMask1Lock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecGpioMask1Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecGpioMask1Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecGpioMask1Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecGpioMask1Lock {}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` reader - SEC_GPIO_MASK1 register write-lock."]
pub type SecGpioMask1LockR = crate::FieldReader<SecGpioMask1Lock>;
impl SecGpioMask1LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecGpioMask1Lock> {
        match self.bits {
            1 => Some(SecGpioMask1Lock::Blocked),
            2 => Some(SecGpioMask1Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecGpioMask1Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecGpioMask1Lock::Writable
    }
}
#[doc = "Field `SEC_GPIO_MASK1_LOCK` writer - SEC_GPIO_MASK1 register write-lock."]
pub type SecGpioMask1LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecGpioMask1Lock>;
impl<'a, REG> SecGpioMask1LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask1Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecGpioMask1Lock::Writable)
    }
}
#[doc = "SEC_CPU_INT_MASK0 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecCpu1IntMask0Lock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecCpu1IntMask0Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecCpu1IntMask0Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecCpu1IntMask0Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecCpu1IntMask0Lock {}
#[doc = "Field `SEC_CPU1_INT_MASK0_LOCK` reader - SEC_CPU_INT_MASK0 register write-lock."]
pub type SecCpu1IntMask0LockR = crate::FieldReader<SecCpu1IntMask0Lock>;
impl SecCpu1IntMask0LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecCpu1IntMask0Lock> {
        match self.bits {
            1 => Some(SecCpu1IntMask0Lock::Blocked),
            2 => Some(SecCpu1IntMask0Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecCpu1IntMask0Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecCpu1IntMask0Lock::Writable
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK0_LOCK` writer - SEC_CPU_INT_MASK0 register write-lock."]
pub type SecCpu1IntMask0LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecCpu1IntMask0Lock>;
impl<'a, REG> SecCpu1IntMask0LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecCpu1IntMask0Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecCpu1IntMask0Lock::Writable)
    }
}
#[doc = "SEC_CPU_INT_MASK1 register write-lock.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SecCpu1IntMask1Lock {
    #[doc = "1: Restricted mode."]
    Blocked = 1,
    #[doc = "2: Writable."]
    Writable = 2,
}
impl From<SecCpu1IntMask1Lock> for u8 {
    #[inline(always)]
    fn from(variant: SecCpu1IntMask1Lock) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SecCpu1IntMask1Lock {
    type Ux = u8;
}
impl crate::IsEnum for SecCpu1IntMask1Lock {}
#[doc = "Field `SEC_CPU1_INT_MASK1_LOCK` reader - SEC_CPU_INT_MASK1 register write-lock."]
pub type SecCpu1IntMask1LockR = crate::FieldReader<SecCpu1IntMask1Lock>;
impl SecCpu1IntMask1LockR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SecCpu1IntMask1Lock> {
        match self.bits {
            1 => Some(SecCpu1IntMask1Lock::Blocked),
            2 => Some(SecCpu1IntMask1Lock::Writable),
            _ => None,
        }
    }
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn is_blocked(&self) -> bool {
        *self == SecCpu1IntMask1Lock::Blocked
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn is_writable(&self) -> bool {
        *self == SecCpu1IntMask1Lock::Writable
    }
}
#[doc = "Field `SEC_CPU1_INT_MASK1_LOCK` writer - SEC_CPU_INT_MASK1 register write-lock."]
pub type SecCpu1IntMask1LockW<'a, REG> = crate::FieldWriter<'a, REG, 2, SecCpu1IntMask1Lock>;
impl<'a, REG> SecCpu1IntMask1LockW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Restricted mode."]
    #[inline(always)]
    pub fn blocked(self) -> &'a mut crate::W<REG> {
        self.variant(SecCpu1IntMask1Lock::Blocked)
    }
    #[doc = "Writable."]
    #[inline(always)]
    pub fn writable(self) -> &'a mut crate::W<REG> {
        self.variant(SecCpu1IntMask1Lock::Writable)
    }
}
impl R {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&self) -> SecGpioMask0LockR {
        SecGpioMask0LockR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&self) -> SecGpioMask1LockR {
        SecGpioMask1LockR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&self) -> SecCpu1IntMask0LockR {
        SecCpu1IntMask0LockR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&self) -> SecCpu1IntMask1LockR {
        SecCpu1IntMask1LockR::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SEC_GPIO_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask0_lock(&mut self) -> SecGpioMask0LockW<SecMaskLockSpec> {
        SecGpioMask0LockW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SEC_GPIO_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_gpio_mask1_lock(&mut self) -> SecGpioMask1LockW<SecMaskLockSpec> {
        SecGpioMask1LockW::new(self, 2)
    }
    #[doc = "Bits 8:9 - SEC_CPU_INT_MASK0 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask0_lock(&mut self) -> SecCpu1IntMask0LockW<SecMaskLockSpec> {
        SecCpu1IntMask0LockW::new(self, 8)
    }
    #[doc = "Bits 10:11 - SEC_CPU_INT_MASK1 register write-lock."]
    #[inline(always)]
    pub fn sec_cpu1_int_mask1_lock(&mut self) -> SecCpu1IntMask1LockW<SecMaskLockSpec> {
        SecCpu1IntMask1LockW::new(self, 10)
    }
}
#[doc = "Security General Purpose register access control.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_mask_lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_mask_lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecMaskLockSpec;
impl crate::RegisterSpec for SecMaskLockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_mask_lock::R`](R) reader structure"]
impl crate::Readable for SecMaskLockSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_mask_lock::W`](W) writer structure"]
impl crate::Writable for SecMaskLockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_MASK_LOCK to value 0x0aaa"]
impl crate::Resettable for SecMaskLockSpec {
    const RESET_VALUE: u32 = 0x0aaa;
}
