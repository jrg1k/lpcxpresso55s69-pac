#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` reader"]
pub type R = crate::R<SecCtrlAhbPort8Slave1RuleSpec>;
#[doc = "Register `SEC_CTRL_AHB_PORT8_SLAVE1_RULE` writer"]
pub type W = crate::W<SecCtrlAhbPort8Slave1RuleSpec>;
#[doc = "Flexcomm interface 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm2Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm2Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm2Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm2Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm2Rule {}
#[doc = "Field `FLEXCOMM2_RULE` reader - Flexcomm interface 2"]
pub type Flexcomm2RuleR = crate::FieldReader<Flexcomm2Rule>;
impl Flexcomm2RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm2Rule {
        match self.bits {
            0 => Flexcomm2Rule::EnumNsNp,
            1 => Flexcomm2Rule::EnumNsP,
            2 => Flexcomm2Rule::EnumSNp,
            3 => Flexcomm2Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm2Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm2Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm2Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm2Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM2_RULE` writer - Flexcomm interface 2"]
pub type Flexcomm2RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm2Rule, crate::Safe>;
impl<'a, REG> Flexcomm2RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm2Rule::EnumSP)
    }
}
#[doc = "Flexcomm interface 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm3Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm3Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm3Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm3Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm3Rule {}
#[doc = "Field `FLEXCOMM3_RULE` reader - Flexcomm interface 3"]
pub type Flexcomm3RuleR = crate::FieldReader<Flexcomm3Rule>;
impl Flexcomm3RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm3Rule {
        match self.bits {
            0 => Flexcomm3Rule::EnumNsNp,
            1 => Flexcomm3Rule::EnumNsP,
            2 => Flexcomm3Rule::EnumSNp,
            3 => Flexcomm3Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm3Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm3Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm3Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm3Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM3_RULE` writer - Flexcomm interface 3"]
pub type Flexcomm3RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm3Rule, crate::Safe>;
impl<'a, REG> Flexcomm3RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm3Rule::EnumSP)
    }
}
#[doc = "Flexcomm interface 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flexcomm4Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Flexcomm4Rule> for u8 {
    #[inline(always)]
    fn from(variant: Flexcomm4Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flexcomm4Rule {
    type Ux = u8;
}
impl crate::IsEnum for Flexcomm4Rule {}
#[doc = "Field `FLEXCOMM4_RULE` reader - Flexcomm interface 4"]
pub type Flexcomm4RuleR = crate::FieldReader<Flexcomm4Rule>;
impl Flexcomm4RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Flexcomm4Rule {
        match self.bits {
            0 => Flexcomm4Rule::EnumNsNp,
            1 => Flexcomm4Rule::EnumNsP,
            2 => Flexcomm4Rule::EnumSNp,
            3 => Flexcomm4Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Flexcomm4Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Flexcomm4Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Flexcomm4Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Flexcomm4Rule::EnumSP
    }
}
#[doc = "Field `FLEXCOMM4_RULE` writer - Flexcomm interface 4"]
pub type Flexcomm4RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Flexcomm4Rule, crate::Safe>;
impl<'a, REG> Flexcomm4RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Flexcomm4Rule::EnumSP)
    }
}
#[doc = "Inter CPU communication Mailbox\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MailboxRule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<MailboxRule> for u8 {
    #[inline(always)]
    fn from(variant: MailboxRule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MailboxRule {
    type Ux = u8;
}
impl crate::IsEnum for MailboxRule {}
#[doc = "Field `MAILBOX_RULE` reader - Inter CPU communication Mailbox"]
pub type MailboxRuleR = crate::FieldReader<MailboxRule>;
impl MailboxRuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MailboxRule {
        match self.bits {
            0 => MailboxRule::EnumNsNp,
            1 => MailboxRule::EnumNsP,
            2 => MailboxRule::EnumSNp,
            3 => MailboxRule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == MailboxRule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == MailboxRule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == MailboxRule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == MailboxRule::EnumSP
    }
}
#[doc = "Field `MAILBOX_RULE` writer - Inter CPU communication Mailbox"]
pub type MailboxRuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, MailboxRule, crate::Safe>;
impl<'a, REG> MailboxRuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(MailboxRule::EnumSP)
    }
}
#[doc = "High Speed GPIO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio0Rule {
    #[doc = "0: Non-secure and Non-priviledge user access allowed."]
    EnumNsNp = 0,
    #[doc = "1: Non-secure and Privilege access allowed."]
    EnumNsP = 1,
    #[doc = "2: Secure and Non-priviledge user access allowed."]
    EnumSNp = 2,
    #[doc = "3: Secure and Priviledge user access allowed."]
    EnumSP = 3,
}
impl From<Gpio0Rule> for u8 {
    #[inline(always)]
    fn from(variant: Gpio0Rule) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio0Rule {
    type Ux = u8;
}
impl crate::IsEnum for Gpio0Rule {}
#[doc = "Field `GPIO0_RULE` reader - High Speed GPIO"]
pub type Gpio0RuleR = crate::FieldReader<Gpio0Rule>;
impl Gpio0RuleR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio0Rule {
        match self.bits {
            0 => Gpio0Rule::EnumNsNp,
            1 => Gpio0Rule::EnumNsP,
            2 => Gpio0Rule::EnumSNp,
            3 => Gpio0Rule::EnumSP,
            _ => unreachable!(),
        }
    }
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_np(&self) -> bool {
        *self == Gpio0Rule::EnumNsNp
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn is_enum_ns_p(&self) -> bool {
        *self == Gpio0Rule::EnumNsP
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_np(&self) -> bool {
        *self == Gpio0Rule::EnumSNp
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn is_enum_s_p(&self) -> bool {
        *self == Gpio0Rule::EnumSP
    }
}
#[doc = "Field `GPIO0_RULE` writer - High Speed GPIO"]
pub type Gpio0RuleW<'a, REG> = crate::FieldWriter<'a, REG, 2, Gpio0Rule, crate::Safe>;
impl<'a, REG> Gpio0RuleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_ns_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rule::EnumNsNp)
    }
    #[doc = "Non-secure and Privilege access allowed."]
    #[inline(always)]
    pub fn enum_ns_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rule::EnumNsP)
    }
    #[doc = "Secure and Non-priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_np(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rule::EnumSNp)
    }
    #[doc = "Secure and Priviledge user access allowed."]
    #[inline(always)]
    pub fn enum_s_p(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio0Rule::EnumSP)
    }
}
impl R {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    pub fn flexcomm2_rule(&self) -> Flexcomm2RuleR {
        Flexcomm2RuleR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    pub fn flexcomm3_rule(&self) -> Flexcomm3RuleR {
        Flexcomm3RuleR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    pub fn flexcomm4_rule(&self) -> Flexcomm4RuleR {
        Flexcomm4RuleR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    pub fn mailbox_rule(&self) -> MailboxRuleR {
        MailboxRuleR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    pub fn gpio0_rule(&self) -> Gpio0RuleR {
        Gpio0RuleR::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Flexcomm interface 2"]
    #[inline(always)]
    pub fn flexcomm2_rule(&mut self) -> Flexcomm2RuleW<SecCtrlAhbPort8Slave1RuleSpec> {
        Flexcomm2RuleW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Flexcomm interface 3"]
    #[inline(always)]
    pub fn flexcomm3_rule(&mut self) -> Flexcomm3RuleW<SecCtrlAhbPort8Slave1RuleSpec> {
        Flexcomm3RuleW::new(self, 4)
    }
    #[doc = "Bits 8:9 - Flexcomm interface 4"]
    #[inline(always)]
    pub fn flexcomm4_rule(&mut self) -> Flexcomm4RuleW<SecCtrlAhbPort8Slave1RuleSpec> {
        Flexcomm4RuleW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Inter CPU communication Mailbox"]
    #[inline(always)]
    pub fn mailbox_rule(&mut self) -> MailboxRuleW<SecCtrlAhbPort8Slave1RuleSpec> {
        MailboxRuleW::new(self, 12)
    }
    #[doc = "Bits 16:17 - High Speed GPIO"]
    #[inline(always)]
    pub fn gpio0_rule(&mut self) -> Gpio0RuleW<SecCtrlAhbPort8Slave1RuleSpec> {
        Gpio0RuleW::new(self, 16)
    }
}
#[doc = "Security access rules for AHB peripherals.\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_ctrl_ahb_port8_slave1_rule::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_ctrl_ahb_port8_slave1_rule::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecCtrlAhbPort8Slave1RuleSpec;
impl crate::RegisterSpec for SecCtrlAhbPort8Slave1RuleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_ctrl_ahb_port8_slave1_rule::R`](R) reader structure"]
impl crate::Readable for SecCtrlAhbPort8Slave1RuleSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_ctrl_ahb_port8_slave1_rule::W`](W) writer structure"]
impl crate::Writable for SecCtrlAhbPort8Slave1RuleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_CTRL_AHB_PORT8_SLAVE1_RULE to value 0"]
impl crate::Resettable for SecCtrlAhbPort8Slave1RuleSpec {
    const RESET_VALUE: u32 = 0;
}
