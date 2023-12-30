output: strings.o utility.o
	gcc strings.o utility.o -o output -lm

strings.o: strings.c
	gcc -I ../ -c strings.c

utility.o: ../utility/utility.c ../utility/utility.h
	gcc -c ../utility/utility.c -lm

make clean:
	rm *.o output