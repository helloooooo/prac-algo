from sys import stdin
a = stdin.readline().rstrip().split('/')
year = int(a[0])
month = int(a[1])
day = int(a[2])

if year <= 2018:
    print("Heisei")
elif year >= 2019 and month <=4:
    print("Heisei")
else:
    print("TBD")