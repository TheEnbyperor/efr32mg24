#[doc = "Register `CONSUMER_TIMER3_DTIFS1` reader"]
pub type R = crate::R<ConsumerTimer3Dtifs1Spec>;
#[doc = "Register `CONSUMER_TIMER3_DTIFS1` writer"]
pub type W = crate::W<ConsumerTimer3Dtifs1Spec>;
#[doc = "Field `PRSSEL` reader - DTI async channel select"]
pub type PrsselR = crate::FieldReader;
#[doc = "Field `PRSSEL` writer - DTI async channel select"]
pub type PrsselW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DTI async channel select"]
    #[inline(always)]
    pub fn prssel(&self) -> PrsselR {
        PrsselR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DTI async channel select"]
    #[inline(always)]
    #[must_use]
    pub fn prssel(&mut self) -> PrsselW<ConsumerTimer3Dtifs1Spec> {
        PrsselW::new(self, 0)
    }
}
#[doc = "DTI Consumer register\n\nYou can [`read`](crate::Reg::read) this register and get [`consumer_timer3_dtifs1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`consumer_timer3_dtifs1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsumerTimer3Dtifs1Spec;
impl crate::RegisterSpec for ConsumerTimer3Dtifs1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`consumer_timer3_dtifs1::R`](R) reader structure"]
impl crate::Readable for ConsumerTimer3Dtifs1Spec {}
#[doc = "`write(|w| ..)` method takes [`consumer_timer3_dtifs1::W`](W) writer structure"]
impl crate::Writable for ConsumerTimer3Dtifs1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONSUMER_TIMER3_DTIFS1 to value 0"]
impl crate::Resettable for ConsumerTimer3Dtifs1Spec {
    const RESET_VALUE: u32 = 0;
}
