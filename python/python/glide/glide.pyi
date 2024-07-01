from collections.abc import Callable
from enum import Enum
from typing import List, Optional

from glide.constants import TResult

DEFAULT_TIMEOUT_IN_MILLISECONDS: int = ...
MAX_REQUEST_ARGS_LEN: int = ...

class Level(Enum):
    Error = 0
    Warn = 1
    Info = 2
    Debug = 3
    Trace = 4

    def is_lower(self, level: Level) -> bool: ...

class Script:
    def __init__(self, code: str) -> None: ...
    def __init__(self, code: bytes) -> None: ...
    def get_hash(self) -> str: ...
    def __del__(self) -> None: ...

def start_socket_listener_external(init_callback: Callable) -> None: ...
def value_from_pointer(pointer: int) -> TResult: ...
def create_leaked_value(message: str) -> int: ...
def create_leaked_bytes_vec(args_vec: List[bytes]) -> int: ...
def py_init(level: Optional[Level], file_name: Optional[str]) -> Level: ...
def py_log(log_level: Level, log_identifier: str, message: str) -> None: ...
