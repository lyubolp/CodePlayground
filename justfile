venv:
    . .venv/bin/activate

init:
    python3 -m venv .venv
    venv
    pip install -r requirements.txt

lint: venv
    python3 -m pylint */*.py main.py --fail-under 9
    mypy src main.py
    flake8 src main.py

test: venv
    python3 -m unittest discover -s tests

push: venv lint test
    git push