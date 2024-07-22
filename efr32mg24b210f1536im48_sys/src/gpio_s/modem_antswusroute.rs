#[doc = "Register `MODEM_ANTSWUSROUTE` reader"]
pub type R = crate::R<ModemAntswusrouteSpec>;
#[doc = "Register `MODEM_ANTSWUSROUTE` writer"]
pub type W = crate::W<ModemAntswusrouteSpec>;
#[doc = "Field `PORT` reader - ANTSWUS port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - ANTSWUS port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - ANTSWUS pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - ANTSWUS pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - ANTSWUS port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - ANTSWUS pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ANTSWUS port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<ModemAntswusrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - ANTSWUS pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<ModemAntswusrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "ANTSWUS port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`modem_antswusroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`modem_antswusroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModemAntswusrouteSpec;
impl crate::RegisterSpec for ModemAntswusrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`modem_antswusroute::R`](R) reader structure"]
impl crate::Readable for ModemAntswusrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`modem_antswusroute::W`](W) writer structure"]
impl crate::Writable for ModemAntswusrouteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MODEM_ANTSWUSROUTE to value 0"]
impl crate::Resettable for ModemAntswusrouteSpec {
    const RESET_VALUE: u32 = 0;
}
