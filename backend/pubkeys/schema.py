from ninja import Schema


class UserSchema(Schema):
    id: int
    username: str
    password: str
