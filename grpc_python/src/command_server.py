import sys
sys.path.insert(1, 'schema/')

from unifmu_fmi2_pb2 import *
import unifmu_fmi2_pb2_grpc
from fmi2 import Fmi2FMU


class CommandServicer(unifmu_fmi2_pb2_grpc.SendCommandServicer):

    def __init__(self, fmu):
        super().__init__()
        self.fmu = fmu

    ##### REAL #####
    def Fmi2SetReal(self, request, context):
        status = self.fmu.set_xxx(request.references, request.values)
        return StatusReturn(status=status)

    def Fmi2GetReal(self, request, context):
        status, values = self.fmu.get_xxx(request.references)
        return GetRealReturn(status=status, values=values)


    ##### INTEGER #####
    def Fmi2SetInteger(self, request, context):
        status = self.fmu.set_xxx(request.references, request.values)
        return StatusReturn(status=status)

    def Fmi2GetInteger(self, request, context):
        status, values = self.fmu.get_xxx(request.references)
        return GetIntegerReturn(status=status, values=values)


    ##### BOOLEAN #####
    def Fmi2SetBoolean(self, request, context):
        status = self.fmu.set_xxx(request.references, request.values)
        return StatusReturn(status=status)

    def Fmi2GetBoolean(self, request, context):
        status, values = self.fmu.get_xxx(request.references)
        return GetBooleanReturn(status=status, values=values)

    ##### STRING #####
    def Fmi2SetString(self, request, context):
        status = self.fmu.set_xxx(request.references, request.values)
        return StatusReturn(status=status)

    def Fmi2GetString(self, request, context):
        status, values = self.fmu.get_xxx(request.references)
        return GetStringReturn(status=status, values=values)

    #### Do step ####
    def Fmi2DoStep(self, request, context):
        status = self.fmu.do_step(request.current_time, request.step_size, request.no_step_prior)
        return StatusReturn(status=status)

    ##### Set Debug Logging ####
    def Fmi2SetDebugLogging(self, request, context):
        status = self.fmu.set_debug_logging(request.categories, request.logging_on)
        return StatusReturn(status=status)
