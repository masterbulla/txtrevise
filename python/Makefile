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
	dist/${TARGET}
	#strip dist/${TARGET}

instnix:
	@echo Installing executable...
	mv dist/${TARGET} ${INSTALL_DIR_U}

instwin:
	@echo Installing executable...
	mv dist/${TARGET} ${INSTALL_DIR_W}

testnix:
	cp sample.txt rsample.txt
	${TARGET} -i
	@echo
	${TARGET} -v
	@echo
	cat -n rsample.txt
	${TARGET} -f rsample.txt -l 1 -m "colours" -r "colors"
	cat -n rsample.txt
	${TARGET} -q -r rsample.txt -l 2 -m "hates" -r "loves"
	cat -n rsample.txt

cleannix:
	rm rsample.txt
	rm ${INSTALL_DIR_U}/${TARGET}
