from ninja import Router
from typing import List
from django.shortcuts import get_object_or_404


from pubkeys.models import Pubkey
from pubkeys.schema import PubkeySchema
from utils.schema import NotFoundSchema

router = Router()


@router.get("/", response={200: List[PubkeySchema]})
def list_pubkeys(request):
    queryset = Pubkey.objects.all()
    return queryset


@router.get("/{pubkey_id}", response={200: PubkeySchema, 404: NotFoundSchema})
def get_pubkey(request, pubkey_id: int):
    pubkey = get_object_or_404(Pubkey, id=pubkey_id)
    return pubkey


@router.post("/", response={200: PubkeySchema})
def new_pubkey(request, payload: PubkeySchema):
    pubkey = Pubkey.objects.create(**payload.dict())
    return pubkey


@router.put("/{pubkey_id}", response={200: PubkeySchema, 404: NotFoundSchema})
def update_pubkey(request, pubkey_id: int, data: PubkeySchema):
    pubkey = get_object_or_404(Pubkey, id=pubkey_id)
    for attribute, value in data.dict().items():
        setattr(pubkey, attribute, value)
        pubkey.save()
    return pubkey
