import produce_api

letter = str(input("Hello! What word would you like to start with? ")) #var name is letters because legacy
howLong = int(input('How many words would you like to generate? '))
output = produce_api.generator.gen(letter,howLong)
print(output)
with open('result.txt', 'w') as f:
    f.write(output)
print("Thank you for using produce.py!")
