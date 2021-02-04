#[doc = "GPIO status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_status](gpio_qspi_status) module"]
pub type GPIO_QSPI_STATUS = crate::Reg<u32, _GPIO_QSPI_STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_STATUS;
#[doc = "`read()` method returns [gpio_qspi_status::R](gpio_qspi_status::R) reader structure"]
impl crate::Readable for GPIO_QSPI_STATUS {}
#[doc = "GPIO status"]
pub mod gpio_qspi_status;
#[doc = "GPIO control including function select and overrides.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio_qspi_ctrl](gpio_qspi_ctrl) module"]
pub type GPIO_QSPI_CTRL = crate::Reg<u32, _GPIO_QSPI_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GPIO_QSPI_CTRL;
#[doc = "`read()` method returns [gpio_qspi_ctrl::R](gpio_qspi_ctrl::R) reader structure"]
impl crate::Readable for GPIO_QSPI_CTRL {}
#[doc = "`write(|w| ..)` method takes [gpio_qspi_ctrl::W](gpio_qspi_ctrl::W) writer structure"]
impl crate::Writable for GPIO_QSPI_CTRL {}
#[doc = "GPIO control including function select and overrides."]
pub mod gpio_qspi_ctrl;
