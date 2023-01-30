"""Result type for Python"""
from abc import ABC, abstractmethod
from typing import Generic, TypeVar, Union

T = TypeVar("T")
E = TypeVar("E")


class BaseResult(ABC, Generic[T, E]):
    """Base class for Result types"""
    def __init__(self, is_ok: bool) -> None:
        super().__init__()
        self._is_ok: bool = is_ok

    def __bool__(self) -> bool:
        return self._is_ok

    @abstractmethod
    def __eq__(self, other) -> bool:
        """
        Compare two results.
        """

    def is_ok(self) -> bool:
        """
        Returns true if the result is Ok.
        """
        return self._is_ok

    @abstractmethod
    def value(self) -> Union[T, E]:
        """
        Get the value of the result.
        """


class Ok(BaseResult[T, E]):
    """Ok result type"""
    def __init__(self, value) -> None:
        super().__init__(True)
        self.__value: T = value

    def value(self) -> T:
        """
        Get the value of the result.
        """
        return self.__value

    def __eq__(self, other) -> bool:
        return isinstance(other, Ok) and self.__value == other.value()


class Error(BaseResult[T, E]):
    """Error result type"""
    def __init__(self, error: E) -> None:
        super().__init__(False)
        self.__error: E = error

    def value(self) -> E:
        """
        Get the error of the result.
        """
        return self.__error

    def __eq__(self, other) -> bool:
        return isinstance(other, Error) and self.__error == other.value()


Result = Union[Ok[T, E], Error[T, E]]
