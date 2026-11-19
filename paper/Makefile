# Build the research PDF. Needs a TeX toolchain: latexmk + pdflatex (TeX Live), or tectonic.
# Every number comes from paper/generated/numbers.tex, made by tools/paper/numbers.py.
all: main.pdf

generated/numbers.tex: FORCE
	cd .. && python3 -I tools/paper/numbers.py

main.pdf: main.tex sections/*.tex generated/numbers.tex
	@if command -v latexmk >/dev/null; then latexmk -pdf -interaction=nonstopmode main.tex; \
	elif command -v tectonic >/dev/null; then tectonic main.tex; \
	else echo "no TeX toolchain: install texlive (latexmk) or tectonic"; exit 1; fi

check:
	cd .. && python3 -I tools/paper/numbers.py --check

clean:
	rm -f *.aux *.log *.out *.fls *.fdb_latexmk *.toc

FORCE:
.PHONY: all check clean FORCE
