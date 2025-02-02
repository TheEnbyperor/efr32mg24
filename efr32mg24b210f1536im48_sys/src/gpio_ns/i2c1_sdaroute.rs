#[doc = "Register `I2C1_SDAROUTE` reader"]
pub type R = crate::R<I2c1SdarouteSpec>;
#[doc = "Register `I2C1_SDAROUTE` writer"]
pub type W = crate::W<I2c1SdarouteSpec>;
#[doc = "Field `PORT` reader - SDA port select register"]
pub type PortR = crate::FieldReader;
#[doc = "Field `PORT` writer - SDA port select register"]
pub type PortW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PIN` reader - SDA pin select register"]
pub type PinR = crate::FieldReader;
#[doc = "Field `PIN` writer - SDA pin select register"]
pub type PinW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - SDA port select register"]
    #[inline(always)]
    pub fn port(&self) -> PortR {
        PortR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - SDA pin select register"]
    #[inline(always)]
    pub fn pin(&self) -> PinR {
        PinR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDA port select register"]
    #[inline(always)]
    #[must_use]
    pub fn port(&mut self) -> PortW<I2c1SdarouteSpec> {
        PortW::new(self, 0)
    }
    #[doc = "Bits 16:19 - SDA pin select register"]
    #[inline(always)]
    #[must_use]
    pub fn pin(&mut self) -> PinW<I2c1SdarouteSpec> {
        PinW::new(self, 16)
    }
}
#[doc = "SDA port/pin select\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c1_sdaroute::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c1_sdaroute::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c1SdarouteSpec;
impl crate::RegisterSpec for I2c1SdarouteSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c1_sdaroute::R`](R) reader structure"]
impl crate::Readable for I2c1SdarouteSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c1_sdaroute::W`](W) writer structure"]
impl crate::Writable for I2c1SdarouteSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C1_SDAROUTE to value 0"]
impl crate::Resettable for I2c1SdarouteSpec {
    const RESET_VALUE: u32 = 0;
}
