counts = []
arrin = {}
printed=0
with open('model.txt') as f:
    for line in f:
        doesContainAlready = False
        line = line.replace('\n', '')
        pre = line.split(';')[0]
        post = line.split(';')[1]
        n=0
        #for i in counts:
        #    if i.startswith(line):
        #        doesContainAlready = True
        #        break
        #    n+=1 #good riddance (maybe)
        try:
            n = int(arrin.get(line))
            doesContainAlready = True
        except:
            doesContainAlready = False
        if doesContainAlready == False:
            counts.append(line + ';1')
            arrin[line] = len(counts) - 1
        elif doesContainAlready == True:
            try:
                counts[n] = line + ';' + str(int(counts[n].split(';')[2]) + 1)
            except:
                pass
        printed+=1
        print(printed)
print(counts)
with open('counts.txt', 'w') as f:
    for i in counts:
        i = i.replace('\n', '\\n')
        f.write(i + '\n')
