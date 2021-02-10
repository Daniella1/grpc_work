import sys
sys.path.insert(1, 'schema/')

import json
import logging
import xml.etree.ElementTree as ET
from argparse import ArgumentParser
from pathlib import Path

# from command_server import CommandServicer
from adder import Adder

from concurrent import futures
import logging
import grpc

from generate_from_proto import generate_schema
import unifmu_fmi2_pb2_grpc
from command_server import CommandServicer

def serve(fmuInstance):
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    unifmu_fmi2_pb2_grpc.add_SendCommandServicer_to_server(CommandServicer(fmuInstance), server)
    server.add_insecure_port('[::]:50051') # TODO must connect to correct port (right now this is hardcoded)
    server.start()
    print("Started server")
    print("Waiting!")
    server.wait_for_termination()



if __name__ == "__main__":

    generate_schema("unifmu_fmi2.proto")

    reference_to_attr = {}
    path = Path.cwd().parent / "grpc_python/src/modelDescription.xml"
    with open(path) as f:
        for v in ET.parse(f).find("ModelVariables"):
            reference_to_attr[int(v.attrib["valueReference"])] = v.attrib["name"]

    slave = Adder(reference_to_attr)

    serve(slave)


