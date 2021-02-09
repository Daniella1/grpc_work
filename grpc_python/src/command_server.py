import sys
sys.path.insert(1, 'schema/')

from unifmu_fmi2_pb2 import *
import unifmu_fmi2_pb2_grpc

class CommandServicer(unifmu_fmi2_pb2_grpc.SendCommandServicer):

    def __init__(self, fmu):
        super().__init__()
        self.fmu = fmu

    def SetXXX(self,request, context):
        try:
            attributes = [self.fmu.reference_to_attr[vref] for vref in request.references]
            for a, v in zip(attributes, request.values):
                setattr(self.fmu, a, v)
            status = FmiStatus.Ok
        except Exception:
            status = FmiStatus.Error
        status = FmiStatus.Ok
        return StatusReturn(status=status)

    def GetXXX(self,request,context):
        try:
            attributes = [self.fmu.reference_to_attr[vref] for vref in request.references]
            values = [getattr(self.fmu, a) for a in attributes]
            print(f"read vref: {request.references} with value: {values}")
            status =  FmiStatus.Ok
        except Exception:
            status =  FmiStatus.Error
            values = None
        return status, values

    ##### REAL #####
    def Fmi2SetReal(self, request, context):
        print(f"Received Fmi2SetReal with references: {request.references} and values: {request.values}")
        return self.SetXXX(request, context)

    def Fmi2GetReal(self, request, context):
        print(f"Received Fmi2GetReal with references: {request.references}")
        status, values = self.GetXXX(request, context)
        return GetRealReturn(status=status, values=values)


    ##### INTEGER #####
    def Fmi2SetInteger(self, request, context):
        return self.SetXXX(request, context)

    def Fmi2GetInteger(self, request, context):
        status, values = self.GetXXX(request,context)
        return GetIntegerReturn(status=status, values=values)


    ##### BOOLEAN #####
    def Fmi2SetBoolean(self, request, context):
        return self.SetXXX(request, context)

    def Fmi2GetBoolean(self, request, context):
        status, values = self.GetXXX(request,context)
        return GetBooleanReturn(status=status, values=values)

    ##### STRING #####
    def Fmi2SetString(self, request, context):
        return self.SetXXX(request, context)

    def Fmi2GetString(self, request, context):
        status, values = self.GetXXX(request,context)
        return GetStringReturn(status=status, values=values)


