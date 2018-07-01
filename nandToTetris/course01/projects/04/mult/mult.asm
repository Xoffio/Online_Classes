// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm
// Solved by: Ricardo Marques

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)

@R1
D=M
@count
M=D
M=M+1
M=M+1   // Set counter to R1 + 2

@R2
M=0     // Set output to 0

(LOOP)
@count
M=M-1

@R0
D=M     // Take R0

@R2
M=D+M   // Add R0 with R2(output)

@count
D=M

@LOOP
D-1, JGT  // Decrement counter until get 0

@R0
D=M
@R2
M=M-D   // Extra code to works with 0 multiplications
