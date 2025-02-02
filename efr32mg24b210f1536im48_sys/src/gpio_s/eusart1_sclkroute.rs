#[doc = "Register `EUSART1_SCLKROUTE` reader"]
pub type R = crate::R<Eusart1SclkrouteSpec>;
#[doc = "Register `EUSART1_SCLKROUTE` writer"]
pub type W = crate::W<Eusart1SclkrouteSpec>;
#[doc = "Field `PORT` reader - SCLK port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SCLK port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SCLK pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SCLK pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCLK port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<Eusart1SclkrouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SCLK pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<Eusart1SclkrouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SCLK port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`eusart1_sclkroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eusart1_sclkroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eusart1SclkrouteSpec;
impl crate::RegisterSpec for Eusart1SclkrouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eusart1_sclkroute::R`](R) reader structure"]
impl crate::Readable for Eusart1SclkrouteSpec {}
#[doc = "`write(|w| ..)` method takes [`eusart1_sclkroute::W`](W) writer structure"]
impl crate::Writable for Eusart1SclkrouteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EUSART1_SCLKROUTE to value 0"]
impl crate::Resettable for Eusart1SclkrouteSpec {
    const RESET_VALUE: u32 = 0;
}
