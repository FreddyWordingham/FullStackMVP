from fastapi import APIRouter
from pydantic import BaseModel
from typeguard import typechecked


calculator_router = APIRouter()


class SamplePointRequest(BaseModel):
    real: float
    imag: float
    max_iterations: int = 1000


class SamplePointResponse(BaseModel):
    iterations: int


@calculator_router.post(
    "/sample_point",
    tags=["Calculator"],
    operation_id="samplePoint",
    response_model=SamplePointResponse,
)
async def sample_point(request: SamplePointRequest) -> bool:
    """
    Ping the server to check if it is running.
    """

    c = complex(request.real, request.imag)
    iterations = sample_mandelbrot_point(c, request.max_iterations)

    return SamplePointResponse(iterations=iterations)


@typechecked
def sample_mandelbrot_point(c: complex, max_iterations: int) -> int:
    """
    Calculate the number of iterations required for a point in the Mandelbrot set to escape.
    """

    z = 0
    for n in range(max_iterations):
        if abs(z) > 2:
            return n
        z = z * z + c
    return max_iterations
