"""
Main program code
"""

from src.config import Config
from src.executor import Executor
from src.languages.python import Python

if __name__ == "__main__":
    config = Config('config.json')

    python_executor = Executor(Python(config))
    python_res = python_executor.run('print(2+3)')

    print(python_res)
