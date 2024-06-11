from ninja import Router
from typing import List
from django.shortcuts import get_object_or_404


from users.models import User
from users.schema import UserSchema
from utils.schema import NotFoundSchema

router = Router()


@router.get("/", response={200: List[UserSchema]})
def list_users(request):
    queryset = User.objects.all()
    return queryset


@router.get("/{user_id}", response={200: UserSchema, 404: NotFoundSchema})
def get_user(request, user_id: int):
    user = get_object_or_404(User, id=user_id)
    return user


@router.post("/", response={200: UserSchema})
def new_user(request, payload: UserSchema):
    user = User.objects.create(**payload.dict())
    return user


@router.put("/{user_id}", response={200: UserSchema, 404: NotFoundSchema})
def update_user(request, user_id: int, data: UserSchema):
    user = get_object_or_404(User, id=user_id)
    for attribute, value in data.dict().items():
        setattr(user, attribute, value)
        user.save()
    return user
