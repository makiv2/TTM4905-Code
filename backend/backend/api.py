from ninja import NinjaAPI, Router, Schema

from pubkeys.api import router as pubkey_router
from django_ratelimit.exceptions import Ratelimited


api = NinjaAPI()
api.add_router("/pubkeys", pubkey_router)


@api.exception_handler(Ratelimited)
def too_many_requests(request, exc):
    return api.create_response(
        request,
        {"message": "Too many requests. Please wait"},
        status=429,
    )


class HealthCheckSchema(Schema):
    message: str


@api.get("/healthcheck", auth=None, response={200: HealthCheckSchema})
def health_check(request):
    return 200, {"message": "good health"}
