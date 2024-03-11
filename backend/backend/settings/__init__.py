import os

if os.environ.get("ENV") == "prod":
    from backend.settings.prod import *
else:
    from backend.settings.dev import *
