# Makefile

E        := examples
BINDIR   := bin
LIBDIR   := lib
TESTDIR  := test
SOURCES  := mecab.rs

.PHONY: all
all: libmecab

libmecab: setup-lib $(SOURCES)
	rustc -Z extra-debug-info -O mecab.rs --out-dir $(LIBDIR)

.PHONY: test
test: setup-test $(SOURCES)
	rustc -O mecab.rs --test --out-dir $(TESTDIR)

wakachigaki: setup-bin libmecab $(E)/wakachigaki.rs
	rustc -Z extra-debug-info -O $(E)/wakachigaki.rs -L $(LIBDIR) --out-dir $(BINDIR)

katakanize: setup-bin libmecab $(E)/katakanize.rs
	rustc -O $(E)/katakanize.rs -L $(LIBDIR) --out-dir $(BINDIR)

multithread-simple: setup-bin libmecab $(E)/multithread-simple.rs
	rustc -O $(E)/multithread-simple.rs -L $(LIBDIR) --out-dir $(BINDIR)

collect-nouns: setup-bin libmecab $(E)/collect-nouns.rs
	rustc -O $(E)/collect-nouns.rs -L $(LIBDIR) --out-dir $(BINDIR)

softwakachi: setup-bin libmecab $(E)/softwakachi.rs
	rustc -O $(E)/softwakachi.rs -L $(LIBDIR) --out-dir $(BINDIR)

setup-bin:
	mkdir -p $(BINDIR)

setup-lib:
	mkdir -p $(LIBDIR)

setup-test:
	mkdir -p $(TESTDIR)

.PHONY: clean
clean:
	@rm -rf "$(BINDIR)"
	@rm -rf "$(LIBDIR)"
	@rm -rf "$(TESTDIR)"
