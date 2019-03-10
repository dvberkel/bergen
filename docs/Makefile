SRC=src
MODULES=${SRC}/*.elm
TARGET=js
NAME=brnfck
OUTPUT=${TARGET}/${NAME}.js
MINIMIZED=${TARGET}/${NAME}.min.js

${MINIMIZED}: ${OUTPUT}
	uglifyjs $< --compress 'pure_funcs="F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9",pure_getters,keep_fargs=false,unsafe_comps,unsafe' | uglifyjs --mangle --output=$@

${OUTPUT}: ${SRC}/Main.elm ${MODULES}
	elm make --optimize --output=$@ $<
