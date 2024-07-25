from fastapi import FastAPI, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse
from fastapi.staticfiles import StaticFiles
from mangum import Mangum
import os

from my_api import routes


tags_metadata = [
    {
        "name": "Authentication",
        "description": "Operations related to user authentication.",
    },
    {
        "name": "Calculator",
        "description": "Core functionality operations.",
    },
    {
        "name": "Tests",
        "description": "Operations for testing the API.",
    },
]


api_app = FastAPI(openapi_tags=tags_metadata)
api_app.include_router(routes.auth_router, prefix="")
api_app.include_router(routes.calculator_router, prefix="")
api_app.include_router(routes.tests_router, prefix="")


app = FastAPI()


app.mount("/api", api_app)
app.mount(
    "/static",
    StaticFiles(
        directory=os.path.join(os.path.dirname(__file__), "../../app/build/static")
    ),
    name="static",
)


# Serve the React app
@app.get("/{full_path:path}")
async def serve_react_app(full_path: str, request: Request):
    if full_path.startswith("static/"):
        static_file_path = os.path.join(
            os.path.dirname(__file__), "../../app/build", full_path
        )
        return FileResponse(static_file_path)

    index_file_path = os.path.join(
        os.path.dirname(__file__), "../../app/build/index.html"
    )
    return FileResponse(index_file_path)


# Cross-Origin Resource Sharing (CORS)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


# Define the Lambda handler
handler = Mangum(app)


def lambda_handler(event, context):
    if "source" in event and event["source"] == "aws.events":
        print("This is a warm-up invocation.")
        return {}
    else:
        return handler(event, context)
