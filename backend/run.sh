#! /bin/bash

.venv/bin/python  manage.py collectstatic --noinput
.venv/bin/python manage.py migrate
.venv/bin/python manage.py ingest_pubkeys
exec .venv/bin/uwsgi --emperor /etc/uwsgi/sites/backend.ini
