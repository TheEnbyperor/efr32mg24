#[doc = "Register `CONSUMER_SETAMPER_TAMPERSRC27` reader"]
pub type R = crate::R<ConsumerSetamperTampersrc27Spec>;
#[doc = "Register `CONSUMER_SETAMPER_TAMPERSRC27` writer"]
pub type W = crate::W<ConsumerSetamperTampersrc27Spec>;
#[doc = "Field `PRSSEL` reader - TAMPERSRC27 async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - TAMPERSRC27 async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TAMPERSRC27 async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TAMPERSRC27 async channel select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<ConsumerSetamperTampersrc27Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "TAMPERSRC27 Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_setamper_tampersrc27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_setamper_tampersrc27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerSetamperTampersrc27Spec;
impl crate::RegisterSpec for ConsumerSetamperTampersrc27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_setamper_tampersrc27::R`](R) reader structure"]
impl crate::Readable for ConsumerSetamperTampersrc27Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_setamper_tampersrc27::W`](W) writer structure"]
impl crate::Writable for ConsumerSetamperTampersrc27Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSUMER_SETAMPER_TAMPERSRC27 to value 0"]
impl crate::Resettable for ConsumerSetamperTampersrc27Spec {
    const RESET_VALUE: u32 = 0;
}
