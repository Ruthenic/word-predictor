from numba import jit,njit,cuda
import time

@jit
def counter():
    counts = []
    printed=0
    f = open('model.txt')
    for line in f:
        doesContainAlready = False
        line = line.replace('\n', '')
        pre = line.split(';')[0]
        post = line.split(';')[1]
        n=0
        for i in counts:
            if i.startswith(line):
                doesContainAlready = True
                break
            n+=1
        if doesContainAlready == False:
            counts.append(line + ';1')
        elif doesContainAlready == True:
            counts[n] = line + ';' + str(int(counts[n].split(';')[2]) + 1)
            
        printed+=1
        print(printed)
    f.close()
    f = open('counts.txt', 'w')
    for i in counts:
        i = i.replace('\n', '\\n')
        f.write(i + '\n')
    f.close()

counter()
