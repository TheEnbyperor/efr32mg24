#[doc = "Register `CC2_ICF` reader"]
pub type R = crate::R<Cc2IcfSpec>;
#[doc = "Field `ICF` reader - Input Capture FIFO"]
pub type IcfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Input Capture FIFO"]
    #[inline(always)]
    pub fn icf(&self) -> IcfR {
        IcfR::new(self.bits)
    }
}
#[doc = "No Description\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_icf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2IcfSpec;
impl crate::RegisterSpec for Cc2IcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_icf::R`](R) reader structure"]
impl crate::Readable for Cc2IcfSpec {}
#[doc = "`reset()` method sets CC2_ICF to value 0"]
impl crate::Resettable for Cc2IcfSpec {
    const RESET_VALUE: u32 = 0;
}
