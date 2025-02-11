#[doc = "Register `SEC_CTRL_RAM3_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRam3SlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAM3_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRam3SlaveRuleSpec>;
#[doc = "Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ram3Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ram3Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ram3Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram3Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ram3Rule {}
#[doc = "Field `RAM3_RULE` reader - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
pub type Ram3RuleR = crate::FieldReader<Ram3Rule>;
impl Ram3RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram3Rule {
        match self.bits {
            0 => Ram3Rule::EnumNsNp,
            1 => Ram3Rule::EnumNsP,
            2 => Ram3Rule::EnumSNp,
            3 => Ram3Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ram3Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ram3Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ram3Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ram3Rule::EnumSP
    }
}
#[doc = "Field `RAM3_RULE` writer - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
pub type Ram3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ram3Rule, crate::Safe>;
impl<'a, REG> Ram3RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram3Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram3Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram3Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram3Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[inline(always)]
    pub fn ram3_rule(&self) -> Ram3RuleR {
        Ram3RuleR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM3: 0x2003_0000 - 0x2003_FFFF"]
    #[inline(always)]
    pub fn ram3_rule(&mut self) -> Ram3RuleW<SecCtrlRam3SlaveRuleSpec> {
        Ram3RuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAM3 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram3_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram3_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam3SlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRam3SlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram3_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam3SlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram3_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam3SlaveRuleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM3_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRam3SlaveRuleSpec {
    const RESET_VALUE: u32 = 0;
}
