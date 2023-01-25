"""Result type for Python"""
from abc import ABC
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

    def is_ok(self) -> bool:
        """
        Returns true if the result is Ok.
        """
        return self._is_ok


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


class Error(BaseResult[T, E]):
    """Error result type"""
    def __init__(self, error: E) -> None:
        super().__init__(False)
        self.__error: E = error

    def error(self) -> E:
        """
        Get the error of the result.
        """
        return self.__error


Result = Union[Ok[T, E], Error[T, E]]
