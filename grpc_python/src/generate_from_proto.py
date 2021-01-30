import sys
sys.path.insert(1, 'schema/')
sys.path.insert(1, 'proto/')

import subprocess


def generate_schema(proto_filename="unifmu_fmi2.proto"):
    subprocess.call(["protoc", "--proto_path=proto", "--python_out=schema", "proto/" + proto_filename])
    subprocess.call(["python", "-m", "grpc_tools.protoc", "-I", "proto", "--python_out=schema", "--grpc_python_out=schema", "proto/" + proto_filename])
