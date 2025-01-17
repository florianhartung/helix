from typing import TypeVar, Generic, Union, Optional, Protocol, Tuple, List, Any, Self
from types import TracebackType
from enum import Flag, Enum, auto
from dataclasses import dataclass
from abc import abstractmethod
import weakref

from .types import Result, Ok, Err, Some
from .imports import types

def log(level: types.LogLevel, msg: str) -> None:
    """
    Logs some message in the helix log file.
    """
    raise NotImplementedError

def get_text_selection() -> Optional[str]:
    """
    Logs some message in the helix log file.
    """
    raise NotImplementedError

def set_editor_status(msg: str) -> None:
    """
    --- Functions ---
    """
    raise NotImplementedError

def test() -> None:
    raise NotImplementedError


class HelloWorld(Protocol):

    @abstractmethod
    def get_metadata(self) -> types.PluginMetadata:
        """
        Every plugin has to provide some mandatory metadata, which may be displayed to the user.
        This is the only function from which a plugin may not call imported functions.
        """
        raise NotImplementedError

    @abstractmethod
    def initialize(self) -> None:
        """
        This function is called directly after the editor state is initialized.
        It allows plugins to make changes to the editor before it is rendered for the first time.
        """
        raise NotImplementedError

    @abstractmethod
    def handle_key_press(self, c: str) -> None:
        raise NotImplementedError

