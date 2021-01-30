import sys
sys.path.insert(1, 'schema/')

from unifmu_fmi2_pb2 import Fmi2Status, GetRealReturn, StatusReturn
import unifmu_fmi2_pb2_grpc

class CommandServicer(unifmu_fmi2_pb2_grpc.SendCommandServicer):

    def __init__(self, fmu):
        super().__init__()
        self.fmu = fmu


    def Fmi2SetReal(self, request, context):
        print(f"Received Fmi2SetReal with references: {request.references} and values: {request.values}")
        try:
            print(f"setting {request.references} to {request.values}")
            attributes = [self.fmu.reference_to_attr[vref] for vref in request.references]
            for a, v in zip(attributes, request.values):
                setattr(self.fmu, a, v)
            status = Fmi2Status.Ok
        except Exception:
            status = Fmi2Status.Error
        status = Fmi2Status.Ok
        return StatusReturn(status=status)

    def Fmi2GetReal(self, request, context):
        print(f"Received Fmi2GetReal with references: {request.references}")
        try:
            attributes = [self.fmu.reference_to_attr[vref] for vref in request.references]
            values = [getattr(self.fmu, a) for a in attributes]
            print(f"read vref: {request.references} with value: {values}")
            status =  Fmi2Status.Ok
        except Exception:
            status =  Fmi2Status.Error
            values = None
        return GetRealReturn(status=status, values=values)
