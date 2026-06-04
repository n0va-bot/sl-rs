#==========================================
#    Makefile: makefile for sl 5.1
#	Copyright 1993-2014 Toyoda Masashi (mtoyoda@acm.org)
#	Copyright 2026 N0\A (n0va@krzak.org)
#==========================================

CC=gcc
CFLAGS=-O -Wall

all: sl

sl: sl.c sl.h
	$(CC) $(CFLAGS) -o sl sl.c -lncurses

clean:
	rm -f sl

distclean: clean
