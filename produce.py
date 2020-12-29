import random
letter = input("What word would you like the AI to start with? ")
howLong = int(input("How many words would you like to generate? "))
oldLetter = ''
with open('result.txt', 'w') as f:
    f.write(letter + ' ')
letter= letter[len(letter) -1]
for temp in range(howLong):
    possibles = []
    amount = 0
    with open('counts.txt') as f:
        for line in f:
            if line.startswith(letter):
                possibles.append(line.replace('\n', ''))
    print(possibles)
    for i in possibles:
        try:
            if int(i.split(';')[2]) > amount and i != oldLetter and random.randrange(0,10) > 2:
                amount = int(i.split(';')[2])
                oldLetter = i
                nextletter = i.split(';')[1]
        except:
            pass
    print(nextletter)
    with open('result.txt', 'a') as f:
        f.write(nextletter.replace('\\n', '\n') + ' ')
    letter = nextletter
