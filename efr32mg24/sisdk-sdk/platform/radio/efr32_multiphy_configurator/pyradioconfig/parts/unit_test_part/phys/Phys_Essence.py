from pyradioconfig.calculator_model_framework.interfaces.iphy import IPhy

from py_2_and_3_compatibility import *

class PHYS_Essence(IPhy):
    """
    Init internal variables
    """
    def __init__(self):
        self._phy_name = "Essence Phys"
        self._major = 1
        self._minor = 0
        self._patch = 0


    def PHY_Essence_868M_38p4kbps(self, model):

        phy = self._makePhy(model, model.profiles.Base, 'PHY Essence 868M 38p4kbps')

        #modem_model.vars.shaping_filter.value_forced = model.vars.shaping_filter.var_enum.NONE
        phy.profile_inputs.shaping_filter.values = model.vars.shaping_filter.var_enum.NONE

        phy.profile_inputs.base_frequency_hz.values = long(868300000)

        return phy


    def PHY_Essence_868M_38p4kbps_long_preamble(self, modem_model):

        phy = self._makePhy(modem_model, modem_model.profiles.Base, 'PHY Essence 868M 38p4kbps long preamble')

        #modem_model.vars.shaping_filter.value_forced = model.vars.shaping_filter.var_enum.NONE
        phy.profile_inputs.shaping_filter.values = 1

        #modem_model.vars.base_frequency.value_forced = 868.3
        phy.profile_inputs.base_frequency_hz.values = long(868000003)

        phy.profile_inputs.preamble_pattern.values = 1


