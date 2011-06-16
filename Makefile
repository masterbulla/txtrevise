# Makefile for txtrevise
# ======================
# Requirements to build:
# * Make
# * Python
# * Py2exe (Win) or Freeze (Unix)
# * MSYS (Win)
# * UPX

TARGET = txtrevise
INSTALL_DIR_U = /usr/bin 
INSTALL_DIR_W = /c/dev/bin
FREEZE = /usr/bin/freeze.sh # Shell script for Freeze

make: 
	@echo Build ${TARGET} program
	@echo make py2exe - build + install py2exe executable
	@echo make freeze - build + install Freeze Unix exectuable

py2exe: create_exe.py ${TARGET}.py
	@echo Building executable for Win...
	python create_exe.py py2exe
	rm -r build
	rm dist/w9xpopen.exe
	mv dist/${TARGET}.exe dist/t.exe
	upx -9 -o dist/${TARGET}.exe dist/t.exe
	rm dist/t.exe
	@echo Installing executable...
	mv dist/${TARGET}.exe ${INSTALL_DIR_W}
	@echo Done.
	
freeze: ${TARGET}.py
	cp Makefile Makefile.1
	@echo Building executable for Unix-like...
	${FREEZE} ${TARGET}.py
	make
	rm Makefile
	rm *.c
	rm *.o
	mv Makefile.1 Makefile
	mv ${TARGET} t
	upx -9 -o ${TARGET} t
	rm t
	@echo Installing executable...
	mv ${TARGET} ${INSTALL_DIR_U}
	