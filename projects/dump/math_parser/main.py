import enum
import re
import typing

class OpString(enum.Enum):
    GTE = ">="
    GT = ">"
    LTE = "<="
    LT = "<"

    ADD = "+"
    SUB = "-"

    MUL = "*"
    DIV = "/"
    INT_DIV = "//"
    EXP = "^"

    PAR = r"\(.*\)"

    AND = "and"
    OR = "or"

    PERSISTENCE = r"persist\(.*\, .*\)"

    CHANNEL = r"\|.*\|"

    DELAY_CHANNEL = fr"\|{CHANNEL}\|"

class Operation:
    """
    Some set of operations that takes inputs and creates some output.
    """

    def evaluate(self) -> typing.Any:
        return True

class AnyOperation:
    """
    Takes a set of operations and checks if any are True
    """

    def __init__(self, operations: typing.Iterable[Operation]):
        self.operations = operations

    def evaluate(self) -> bool:
        return any([operation.evaluate() for operation in self.operations])

class AllOperation:
    """
    Takes a set of operations and checks if all are True
    """

    def __init__(self, operations: typing.Iterable[Operation]):
        self.operations = operations

    def evaluate(self) -> bool:
        return all([operation.evaluate() for operation in self.operations])

# class AndOperation:
# class OrOperation:
# class AddOperation:
# class SubtractOperation:
# class DivideOperation:
# class MultiplyOperation:
# class FloorDivisionOperation:

class Condition:
    """
    Some operation that returns a boolean output.
    """

    def __init__(self, operation: Operation):
        self.operation = operation

    def evaluate(self) -> bool:
        return self.operation.evaluate()

class Value:
    def __init__(self, name, channel = None, value=None):
        self.name = name
        self.channel = channel
        self.value = value

TEST_2 = "bounds(|nice.cool|, lte={config.lower_bound}, gte={config.upper_bound})"
TEST_4 = "bounds(|nice.cool|, lt=3, gte=6)"
TEST_5 = "log(nice.cool)"
TEST_3 = "nice.cool "

if __name__ == "__main__":
    print(OpString.DELAY_CHANNEL.value)