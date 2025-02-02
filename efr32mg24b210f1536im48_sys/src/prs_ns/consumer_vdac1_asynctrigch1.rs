#[doc = "Register `CONSUMER_VDAC1_ASYNCTRIGCH1` reader"]
pub type R = crate::R<ConsumerVdac1Asynctrigch1Spec>;
#[doc = "Register `CONSUMER_VDAC1_ASYNCTRIGCH1` writer"]
pub type W = crate::W<ConsumerVdac1Asynctrigch1Spec>;
#[doc = "Field `PRSSEL` reader - ASYNCTRIG async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - ASYNCTRIG async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - ASYNCTRIG async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ASYNCTRIG async channel select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<ConsumerVdac1Asynctrigch1Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "ASYNCTRIG Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_vdac1_asynctrigch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_vdac1_asynctrigch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerVdac1Asynctrigch1Spec;
impl crate::RegisterSpec for ConsumerVdac1Asynctrigch1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_vdac1_asynctrigch1::R`](R) reader structure"]
impl crate::Readable for ConsumerVdac1Asynctrigch1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_vdac1_asynctrigch1::W`](W) writer structure"]
impl crate::Writable for ConsumerVdac1Asynctrigch1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSUMER_VDAC1_ASYNCTRIGCH1 to value 0"]
impl crate::Resettable for ConsumerVdac1Asynctrigch1Spec {
    const RESET_VALUE: u32 = 0;
}
