FROM python:3.11

WORKDIR /usr/src/app
RUN python3 -m venv .venv

RUN .venv/bin/pip install uwsgi==2.0.21.0 --no-cache-dir
RUN mkdir -p /var/log/uwsgi
COPY uwsgi.ini /etc/uwsgi/sites/backend.ini

RUN mkdir -p /usr/src/app

COPY requirements.txt /usr/src/app/
RUN .venv/bin/pip install -r requirements.txt --no-cache-dir

COPY . /usr/src/app
RUN chmod +x /usr/src/app/run.sh

EXPOSE 8000

CMD ["/usr/src/app/run.sh"]