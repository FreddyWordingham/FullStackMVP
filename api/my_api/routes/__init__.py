from .auth import auth_router
from .calculator import calculator_router
from .tests import tests_router


__all__ = [
    "auth_router",
    "calculator_router",
    "tests_router",
]
