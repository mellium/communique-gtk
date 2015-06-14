REBUILD_FLAG =
VENV=env
BIN=$(VENV)/bin
ACTIVATE=source $(BIN)/activate

.PHONY: all
all: test build

.PHONY: pre-commit
pre-commit: .git/hooks/pre-commit
.git/hooks/pre-commit: .pre-commit-config.yaml $(VENV)
	$(ACTIVATE); pre-commit install

$(VENV): $(VENV)/bin/activate

$(VENV)/bin/activate: requirements.txt
	test -d $(VENV) || virtualenv -p /usr/bin/python3 --system-site-packages $(VENV)
	$(ACTIVATE); pip install -r requirements.txt
	touch $(BIN)/activate


.PHONY: test
test: $(VENV) pre-commit
	$(ACTIVATE); tox $(REBUILD_FLAG)

.PHONY: build
build: $(VENV)

.PHONY: run
run: build $(VENV)
	$(ACTIVATE); python -m yokel

.PHONY: debug
debug: build $(VENV)
	GTK_DEBUG=interactive $(ACTIVATE); python -m yokel

.PHONY: clean
clean:
	find . -iname '*.pyc' | xargs rm -f
	rm -rf .tox
	rm -rf $(VENV)
