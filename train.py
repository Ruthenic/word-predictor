model = []
with open('training-dataset.txt') as f:
    for line in f:
        lines = line.split(' ')
        n = 0
        for i in lines:
            try:
                model.append(lines[n] + ';' + lines[n+1])
            except:
                pass
            n+=1
print(model)
with open('model.txt', 'w') as f:
    for i in model:
        i = i.replace('\n', '\\n')
        f.write(i + '\n')
