.PHONY: install
install:
	.venv/bin/pip install -r requirements.txt


.PHONY: venv
venv:
	python3 -m venv .venv


.PHONY: run
run:
	.venv/bin/python manage.py runserver


.PHONY: migrate
migrate:
	.venv/bin/python manage.py migrate


.PHONY: migrations
migrations:
	.venv/bin/python manage.py makemigrations


.PHONY: superuser
superuser:
	.venv/bin/python manage.py createsuperuser


.PHONY: fmt
fmt:
	.venv/bin/black backend users --exclude /migration


.PHONY: check-fmt
check-fmt:
	.venv/bin/black backend users --check --exclude /migrations


.PHONY: mypy
mypy:
	.venv/bin/mypy --config-file mypy.ini .
