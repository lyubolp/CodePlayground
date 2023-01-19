from abc import ABC, abstractmethod

from src.language import Language
from src.program_output import ProgramOutput
from src.result import Ok, Error, Result

class CompiledLanguage(Language):
    @abstractmethod
    def compile(self, code_path: str) -> Result[ProgramOutput, str]:
        pass

    @abstractmethod
    def get_executalbe_path(self) -> str:
        pass
