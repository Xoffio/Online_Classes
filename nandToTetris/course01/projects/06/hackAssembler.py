import sys
import os
import struct

cInstructionTable_comp = {
    '0':    '0101010',
    '1':    '0111111',
    '-1':   '0111010',
    'D':    '0001100',
    'A':    '0110000',
    '!D':   '0001101',
    '!A':   '0110001',
    '-D':   '0001111',
    '-A':   '0110011',
    'D+1':  '0011111',
    'A+1':  '0110111',
    'D-1':  '0001110',
    'A-1':  '0110010',
    'D+A':  '0000010',
    'D-A':  '0010011',
    'A-D':  '0000111',
    'D&A':  '0000000',
    'D|A':  '0010101',
    'M':    '1110000',
    '!M':   '1110001',
    '-M':   '1110011',
    'M+1':  '1110111',
    'M-1':  '1110010',
    'D+M':  '1000010',
    'D-M':  '1010011',
    'M-D':  '1000111',
    'D&M':  '1000000',
    'D|M':  '1010101'
}

cInstructionTable_dest = {
    'null': '000',
    'M':    '001',
    'D':    '010',
    'MD':   '011',
    'A':    '100',
    'AM':   '101',
    'AD':   '110',
    'AMD':  '111'
}

cInstructionTable_jump = {
    'null': '000',
    'JGT':  '001',
    'JEQ':  '010',
    'JGE':  '011',
    'JLT':  '100',
    'JNE':  '101',
    'JLE':  '110',
    'JMP':  '111'
}

labelsTable = {
    'SP':   '000000000000000',
    'LCL':  '000000000000001',
    'ARG':  '000000000000010',
    'THIS': '000000000000011',
    'THAT': '000000000000100',
    'R0':   '000000000000000',
    'R1':   '000000000000001',
    'R2':   '000000000000010',
    'R3':   '000000000000011',
    'R4':   '000000000000100',
    'R5':   '000000000000101',
    'R6':   '000000000000110',
    'R7':   '000000000000111',
    'R8':   '000000000001000',
    'R9':   '000000000001001',
    'R10':  '000000000001010',
    'R11':  '000000000001011',
    'R12':  '000000000001100',
    'R13':  '000000000001101',
    'R14':  '000000000001110',
    'R15':  '000000000001111',
    'SCREEN':   '100000000000000',
    'KEYBOARD': '110000000000000'
}

asmFilePath = ''
outFile = ''
logFlag = False

wCount = 0
for w in sys.argv:
    if (w.lower() == "-f"):
        asmFilePath = sys.argv[wCount+1]

    if (w.lower() == "-o"):
        outFile = sys.argv[wCount+1]

    if (w.lower() == "-l"):
        logFlag = True

    wCount = wCount + 1

if (os.path.isfile(asmFilePath)):
    machineString = ''

    asmFile = open(asmFilePath, "r")
    hackFile = open(outFile, 'w')

    # Add new labels
    startFreeMemory = 16;
    pc = 0
    for line in asmFile:
        tmpLine = line.split("//")[0].replace('\n', '').replace(' ', '').replace('\r', '')

        if (len(tmpLine) > 1):
            if ('(' in tmpLine and ')' in tmpLine):
                tmpLine = tmpLine.replace('(', '').replace(')', '')
                if not(tmpLine in labelsTable):
                    address = format(pc, "015b")
                    labelsTable[tmpLine] = address
                    pc = pc - 1

                    if (logFlag):
                        print("label: %s, address: %s"%(tmpLine, address))
            pc = pc + 1
    asmFile.seek(0);

    for line in asmFile:
        tmpLine = line.split("//")[0].replace('\n', '').replace(' ', '').replace('\r', '')

        # If the line is not empty and not a label
        if (len(tmpLine) > 1) and ( not('(' in tmpLine and ')' in tmpLine) ):
            instruction = tmpLine
            if (logFlag):
                print instruction

            # If is an A-Instruction
            if('@' in tmpLine):
                tmpLine = tmpLine.replace('@', '')

                try: # If is an address
                    instruction = int(tmpLine)
                    instruction = '0'+format(instruction, "015b")
                    machineString = machineString + instruction
                except: # If is a Tag
                    if (tmpLine in labelsTable): # If the tag exists already
                        instruction = '0'+labelsTable[tmpLine]
                    else: # It is new variable
                        address = format(startFreeMemory, "015b")
                        labelsTable[tmpLine] = address
                        startFreeMemory = startFreeMemory + 1
                        instruction = '0'+labelsTable[tmpLine]

            # If it is a C-Instruction
            if ('=' in tmpLine):
                tmpLine = tmpLine.split('=')
                if (len(tmpLine) == 2):
                    instruction = '111' + cInstructionTable_comp[tmpLine[1]] + cInstructionTable_dest[tmpLine[0]] + cInstructionTable_jump['null']
                    machineString = machineString + instruction
            if (';' in tmpLine):
                tmpLine = tmpLine.split(';')
                if (len(tmpLine) == 2):
                    instruction = '111' + cInstructionTable_comp[tmpLine[0]] + cInstructionTable_dest['null'] + cInstructionTable_jump[tmpLine[1]]
                    machineString = machineString + instruction

            hackFile.write( instruction+"\n" )

    asmFile.close()
    hackFile.close()

    if (logFlag):
        print machineString
else:
    print("The file %s does not exists."%(asmFilePath))
