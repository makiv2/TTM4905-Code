from ninja import Router
from typing import List
from django.shortcuts import get_object_or_404


from users.models import User
from users.schema import UserSchema
from utils.schema import NotFoundSchema

router = Router()


@router.get("/", response={200: List[UserSchema]})
def list_users(request):
    queryset = Users.objects.all()
    return queryset