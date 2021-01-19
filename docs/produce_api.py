#!/usr/bin/python3
import sys,random

class generator():
    def gen(letter,howLong):
        letter = ''.join(e for e in letter if e.isalnum()).strip().replace('\n', '')
        oldLetter = ''
        full = []
        fullstr = letter + ' '
        with open('result.txt', 'w') as f:
            f.write(letter + ' ')
        letter= letter[len(letter) -1]
        for temp in range(howLong):
            weights=[]
            possibles = []
            amount = 0
            try:
                with open('counts.txt') as f:
                    for line in f:
                        if line.startswith(letter):
                            possibles.append(line.replace('\n', '').replace('\\n', ''))
            except:
                with open('docs/counts.txt') as f:
                    for line in f:
                        if line.startswith(letter):
                            possibles.append(line.replace('\n', '').replace('\\n', ''))
            print(possibles)
            #for i in possibles:
            #    try:
            #        if int(i.split(';')[2]) > amount and not i == oldLetter and random.randrange(0,10) > 2:
            #            amount = int(i.split(';')[2])
            #            oldLetter = i
            #            nextletter = i.split(';')[1]
            #    except:
            #        pass
            for i in possibles:
                weights.append(int(i.split(';')[2]))
            try:
                nextletter = random.choices(possibles, weights=weights, k=1)[0].split(';')[1]
            except:
                nextletter = random.choice(full)
                pass
            print(nextletter)
            #with open('result.txt', 'a') as f:
            #    f.write(nextletter + ' ')
            fullstr += nextletter + ' '
            full.append(nextletter)
            #letter = nextletter
        return fullstr
