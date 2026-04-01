#!/bin/bash
cargo run -p cli -- sample.dtex          # .tex生成
latexmk -pdfdvi -latex=platex sample.tex
