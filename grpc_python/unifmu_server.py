from concurrent import futures
import logging

import grpc

import unifmu_fmi2_pb2
import unifmu_fmi2_pb2_grpc


class CommandServicer(unifmu_fmi2_pb2_grpc.SendCommandServicer):

    def Fmi2SetReal(self, request, context):
        print(f"Received Fmi2SetReal with references: {request.references} and values: {request.values}")
        status = unifmu_fmi2_pb2.Fmi2Status.Ok
        return unifmu_fmi2_pb2.StatusReturn(status=status)


def serve():
    server = grpc.server(futures.ThreadPoolExecutor(max_workers=10))
    unifmu_fmi2_pb2_grpc.add_SendCommandServicer_to_server(CommandServicer(), server)
    server.add_insecure_port('[::]:50051')
    server.start()
    print("Started server")
    print("Waiting!")
    server.wait_for_termination()


if __name__ == '__main__':
    serve()