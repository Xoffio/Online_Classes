import sys
import os

instructionTable = {
    '0':    '0101010',
    '1':    '0111111',
    '-1':   '0111010',
    'D':    '0001100',
    'A':    '0110000',
    '!D':   '0001101',
    '!A':   '0110001',
    '-D':   '0001111',
    '-A':   '0110011'
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
    '!D':   '1001101',
    '!A':   '1110001',
    '-D':   '1001111',
    '-A':   '1110011',
    'D+1':  '1011111',
    'A+1':  '1110111',
    'D-1':  '1001110',
    'A-1':  '1110010',
    'D+A':  '1000010',
    'D-A':  '1010011',
    'A-D':  '1000111',
    'D&A':  '1000000',
    'D|A':  '1010101',

}

if (len(sys.argv) == 2):
    asmFilePath = sys.argv[1]

    if (os.path.isfile(asmFilePath)):
        machineString = ''

        asmFile = open(asmFilePath, "r")
        for line in asmFile:
            tmpLine = line.replace('\n', '').replace(' ', '')

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
                            tmp = '0'+format(tmpLine, "015b")
                            machineString = machineString + tmp
                        except: # If is a Tag
                            pass


        asmFile.close()
        print machineString
    else:
        print("The file %s does not exists."%(asmFilePath))

else:
    print ("Error")
