#[doc = "Register `USART0_TXROUTE` reader"]
pub type R = crate::R<Usart0TxrouteSpec>;
#[doc = "Register `USART0_TXROUTE` writer"]
pub type W = crate::W<Usart0TxrouteSpec>;
#[doc = "Field `PORT` reader - TX port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - TX port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - TX pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - TX pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<Usart0TxrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - TX pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<Usart0TxrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "TX port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`usart0_txroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usart0_txroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Usart0TxrouteSpec;
impl crate::RegisterSpec for Usart0TxrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usart0_txroute::R`](R) reader structure"]
impl crate::Readable for Usart0TxrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`usart0_txroute::W`](W) writer structure"]
impl crate::Writable for Usart0TxrouteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USART0_TXROUTE to value 0"]
impl crate::Resettable for Usart0TxrouteSpec {
    const RESET_VALUE: u32 = 0;
}
