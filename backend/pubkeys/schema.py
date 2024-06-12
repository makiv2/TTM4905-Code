from ninja import Schema


class PubkeySchema(Schema):
    id: int
    key: str
