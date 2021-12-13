with open("input.txt") as f:
    a = [int(n) for n in f.read().split("\n")]

# Part one
print(next(a[i]*a[j] for i in range(len(a)) for j in range(i, len(a)) if a[i]+a[j] == 2020))

# Part two
print(next(a[i]*a[j]*a[k] for i in range(len(a)) for j in range(i, len(a)) for k in range(j, len(a)) if a[i]+a[j]+a[k] == 2020))