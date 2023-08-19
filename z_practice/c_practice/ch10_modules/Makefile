output: utility.o pig_latin.o
	gcc utility.o pig_latin.o -o output -lm

pig_latin.o: pig_latin.c
	gcc -I ../ -c pig_latin.c

utility.o: ../utility/utility.c ../utility/utility.h
	gcc -c ../utility/utility.c -lm

make clean:
	rm *.o output