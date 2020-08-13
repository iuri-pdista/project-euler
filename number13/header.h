#ifndef HEADER_H_INCLUDED
#define HEADER_H_INCLUDED

typedef struct fiftyDigitNumber {
	int * digits;
	struct  fiftyDigitNumber * next;
} as FiftyDigitNumber;

int* readNumbersFile(FILE*);

FiftyDigitNumber* newNumber(int** digits, FiftyDigitNumber* next);

int* sumDigits();
