# =====================================
# Makefile for txtrevise
# =====================================
# Requirements to build:
# * Make
# * Python
# * Py2exe (Win) or Freeze (Unix)
# * MSYS (Win)
# * UPX
# =====================================

TARGET = txtrevise
INSTALL_DIR_U = /usr/bin
INSTALL_DIR_W = /c/dev/bin
FREEZE = cxfreeze

make:
	@echo Build ${TARGET} program
	@echo make py2exe - build py2exe executable
	@echo make freeze - build Freeze Unix exectuable
	@echo make instnix - install Freeze executable
	@echo make instwin - install py2exe executable

py2exe: create_exe.py ${TARGET}.py
	@echo Building executable for Win...
	python create_exe.py py2exe
	rm -r build
	rm dist/w9xpopen.exe
	strip dist/${TARGET}.exe
	rm dist/t.exe

freeze: ${TARGET}.py
	cp Makefile Makefile.1
	@echo Building executable for Unix-like...
	${FREEZE} ${TARGET}.py --target-dir dist
	make
	rm Makefile
	rm -f *.c
	rm -f  *.o
	mv Makefile.1 Makefile

instnix:
	@echo Installing executable...
	mv dist/${TARGET} ${INSTALL_DIR_U}

instwin:
	@echo Installing executable...
	mv dist/${TARGET} ${INSTALL_DIR_W}

testnix:
	cp sample.txt rsample.txt
	${TARGET} -h
	@echo
	${TARGET} -v
	@echo
	cat -n rsample.txt
	@echo
	${TARGET} -f rsample.txt -l 1 -m "colours" -r "colors"
	@echo
	cat -n rsample.txt
	@echo
	${TARGET} -q -f rsample.txt -l 2 -m "hates" -r "loves"
	@echo
	cat -n rsample.txt

cleannix:
	rm -f rsample.txt
	rm -f ${INSTALL_DIR_U}/${TARGET}
