import pickle
from typing import Tuple


import sys
sys.path.insert(1, 'schema/')
from unifmu_fmi2_pb2 import FmiStatus


class Adder():
    def __init__(self, reference_to_attr) -> None:

        self.real_a = 0.0
        self.real_b = 0.0

        self.integer_a = 0
        self.integer_b = 0

        self.boolean_a = False
        self.boolean_b = False

        self.string_a = ""
        self.string_b = ""
        
        self.reference_to_attr = reference_to_attr

    def __repr__(self):
        return "Adder"

    def serialize(self):

        bytes = pickle.dumps(
            (
                self.real_a,
                self.real_b,
                self.integer_a,
                self.integer_b,
                self.boolean_a,
                self.boolean_b,
                self.string_a,
                self.string_b,
            )
        )
        return FmiStatus.Ok, bytes

    def deserialize(self, bytes) -> int:
        (
            real_a,
            real_b,
            integer_a,
            integer_b,
            boolean_a,
            boolean_b,
            string_a,
            string_b,
        ) = pickle.loads(bytes)
        self.real_a = real_a
        self.real_b = real_b
        self.integer_a = integer_a
        self.integer_b = integer_b
        self.boolean_a = boolean_a
        self.boolean_b = boolean_b
        self.string_a = string_a
        self.string_b = string_b

        return FmiStatus.ok

    @property
    def real_c(self):
        return self.real_a + self.real_b

    @property
    def integer_c(self):
        return self.integer_a + self.integer_b

    @property
    def boolean_c(self):
        return self.boolean_a and self.boolean_b

    @property
    def string_c(self):
        return self.string_a + self.string_b

