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

asmFilePath = ''
outFile = ''

wCount = 0
for w in sys.argv:
    if (w.lower() == "-f"):
        asmFilePath = sys.argv[wCount+1]

    if (w.lower() == "-o"):
        outFile = sys.argv[wCount+1]

    wCount = wCount + 1

if (os.path.isfile(asmFilePath)):
    machineString = ''

    asmFile = open(asmFilePath, "r")
    hackFile = open(outFile, 'w')
    for line in asmFile:
        tmpLine = line.replace('\n', '').replace(' ', '').replace('\r', '')

        # If is not a comment
        if not('//' in tmpLine):
            # If the line is not empty
            if (len(tmpLine) > 1):
                print tmpLine
                # If is an A-Instruction
                if('@' in tmpLine):
                    tmpLine = tmpLine.replace('@', '')

                    try: # If is an address
                        tmpLine = int(tmpLine)
                        tmpLine = '0'+format(tmpLine, "015b")
                        machineString = machineString + tmpLine
                        hackFile.write( tmpLine+'\n' )
                    except: # If is a Tag
                        pass

                # If it is a C-Instruction
                if ('=' in tmpLine):

                    tmpLine = tmpLine.split('=')
                    if (len(tmpLine) == 2):
                        cIns = '111' + cInstructionTable_comp[tmpLine[1]] + cInstructionTable_dest[tmpLine[0]] + cInstructionTable_jump['null']
                        machineString = machineString + cIns
                        hackFile.write( cIns+"\n" )


    asmFile.close()
    hackFile.close()

    print machineString
else:
    print("The file %s does not exists."%(asmFilePath))
