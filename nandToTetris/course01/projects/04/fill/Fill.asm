// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm
// Solved by: Ricardo Marques

// ------ Area where I check the keyboard ------
(CHECK)
@24576
D=M

@SETZERO
D-1, JLT
@screenValue
M=-1            // If any button is press, Set pixels to black
@getPointer
0, JMP
(SETZERO)
@screenValue
M=0             // Else, set pixels to white

D=M
@lastState      // Here I compare with the last stage
D=D+M
@CHECK          // If the last stage and the new one are not equal
D+1, JNE        // then, fill the scenn. Else check the keyboard again

(getPointer)
@SCREEN
D=A             // Get the position of the screen
@screenPointer
M=D             // Save screen address in screenPointer
M=M-1

(LOOP)
@screenPointer
M=M+1           // Increment the pointer

@screenValue
D=M             // Get the value to print in the pixel

@screenPointer
A=M
M=D             // Print the pixel


@SCREEN
D=A
@8192
D=D+A
@screenPointer
D=D-M

@LOOP
D-1, JNE        // Jump back to the loop if I have not fill all the pixels

@screenValue
D=M
@lastState
M=D             // Update lastState
@CHECK
0, JMP          // Else, check the keyboard again.
