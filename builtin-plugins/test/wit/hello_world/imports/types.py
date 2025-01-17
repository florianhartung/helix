from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from ..types import Result, Ok, Err, Some


class RequestedWasiInterfaces(Flag):
    HTTP = auto()
    FOO = auto()
    BAR = auto()
    BAZ = auto()
    BLA = auto()

@dataclass
class PluginMetadata:
    name: str
    description: str
    keywords: List[str]
    requested_wasi_interfaces: RequestedWasiInterfaces

class LogLevel(Enum):
    INFO = 0
    WARN = 1
    ERROR = 2


