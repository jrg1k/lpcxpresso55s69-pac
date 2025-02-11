#[doc = "Register `SEC_CTRL_RAM4_SLAVE_RULE` reader"]
pub type R = crate::R<SecCtrlRam4SlaveRuleSpec>;
#[doc = "Register `SEC_CTRL_RAM4_SLAVE_RULE` writer"]
pub type W = crate::W<SecCtrlRam4SlaveRuleSpec>;
#[doc = "Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ram4Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Ram4Rule> for u8 {
    #[inline(always)]
    fn from(variant: Ram4Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ram4Rule {
    type Ux = u8;
}
impl crate::IsEnum for Ram4Rule {}
#[doc = "Field `RAM4_RULE` reader - Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
pub type Ram4RuleR = crate::FieldReader<Ram4Rule>;
impl Ram4RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ram4Rule {
        match self.bits {
            0 => Ram4Rule::EnumNsNp,
            1 => Ram4Rule::EnumNsP,
            2 => Ram4Rule::EnumSNp,
            3 => Ram4Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Ram4Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Ram4Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Ram4Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Ram4Rule::EnumSP
    }
}
#[doc = "Field `RAM4_RULE` writer - Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
pub type Ram4RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ram4Rule, crate::Safe>;
impl<'a, REG> Ram4RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram4Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram4Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Ram4Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Ram4Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
    #[inline(always)]
    pub fn ram4_rule(&self) -> Ram4RuleR {
        Ram4RuleR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Security access rules for the whole RAM4 : 0x2004_0000 - 0x2004_3FFF"]
    #[inline(always)]
    pub fn ram4_rule(&mut self) -> Ram4RuleW<SecCtrlRam4SlaveRuleSpec> {
        Ram4RuleW::new(self, 0)
    }
}
#[doc = "Security access rules for RAM4 slaves.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ram4_slave_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ram4_slave_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlRam4SlaveRuleSpec;
impl crate::RegisterSpec for SecCtrlRam4SlaveRuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ram4_slave_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlRam4SlaveRuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ram4_slave_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlRam4SlaveRuleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_RAM4_SLAVE_RULE to value 0"]
impl crate::Resettable for SecCtrlRam4SlaveRuleSpec {
    const RESET_VALUE: u32 = 0;
}
