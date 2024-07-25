from fastapi import APIRouter, Depends

from ..hooks import get_profile


tests_router = APIRouter()


@tests_router.get("/ping", tags=["Tests"], operation_id="ping", response_model=bool)
async def ping() -> bool:
    """
    Ping the server to check if it is running.
    """

    return True


@tests_router.get(
    "/auth_ping", tags=["Tests"], operation_id="authPing", response_model=bool
)
async def auth_ping(
    _profile=Depends(get_profile),
) -> bool:
    return True
