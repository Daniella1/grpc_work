import xml.etree.ElementTree as ET
from argparse import ArgumentParser
from pathlib import Path

# from command_server import CommandServicer
from adder import Adder

from concurrent import futures
import grpc

from generate_from_proto import generate_schema
from unifmu_fmi2_pb2_grpc import SendCommandServicer, add_SendCommandServicer_to_server
from unifmu_fmi2_pb2 import StatusReturn, GetRealReturn, GetIntegerReturn, GetBooleanReturn, GetStringReturn

from handshake_pb2_grpc import HandshakerStub
from handshake_pb2 import HandshakeInfo

class CommandServicer(SendCommandServicer):

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


if __name__ == "__main__":

    #generate_schema("unifmu_fmi2.proto")
    #generate_schema("handshake.proto")

    parser = ArgumentParser()
    parser.add_argument(
        "--handshake-endpoint",
        dest="handshake_endpoint",
        type=str,
        help="ip_address:port",
        required=True,
    )
    handshake_info = parser.parse_args().handshake_endpoint
    print(f"Connecting to ip and port: {handshake_info}")
    handshaker_channel = grpc.insecure_channel(handshake_info)


    reference_to_attr = {}
    path = Path.cwd().parent / "grpc_python/src/modelDescription.xml"
    with open(path) as f:
        for v in ET.parse(f).find("ModelVariables"):
            reference_to_attr[int(v.attrib["valueReference"])] = v.attrib["name"]

    slave = Adder(reference_to_attr)

    server = grpc.server(futures.ThreadPoolExecutor())
    add_SendCommandServicer_to_server(CommandServicer(slave), server)
    ip = "localhost"
    p = server.add_insecure_port(f"{ip}:0") # change port to 0, to bind to random port
    server.start()
    print(f"Started server on {p}")
    print("Waiting!")

    # Tell the unifmu wrapper which ip and port the fmu is connected to
    handshaker_client = HandshakerStub(handshaker_channel)
    handshake_message = HandshakeInfo(ip_address=ip, port=str(p))
    handshaker_client.PerformHandshake(handshake_message)
    #handshaker_channel.close()

    
    server.wait_for_termination()


