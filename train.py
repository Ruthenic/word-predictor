model = []
with open('training-dataset.txt') as f:
    for line in f:
        lines = line.split(' ')
        n = 0
        for i in lines:
            try:
                model.append(lines[n].replace(';', '') + ';' + lines[n+1].replace(';', ''))
            except:
                pass
            n+=1
print(model)
with open('model.txt', 'w') as f:
    for i in model:
        if not '#&gt' in i:
            i = i.replace('\n', '\\n')
            f.write(i + '\n')
