from typing import Type

def repr_vars(cls: Type):
    def __repr__(self) -> str:
        return self.__str__()

    def __str__(self) -> str:
        return str(vars(self))

    cls.__repr__ = __repr__
    cls.__str__ = __str__

    return cls
