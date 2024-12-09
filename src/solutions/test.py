input = open("input.txt", "r", encoding="utf-8").read()
lines = [l.strip() for l in input.split("\n") if l.strip() != ""]

line = [int(ch) for ch in lines[0]]
disk = []
idx = 0
for i in range(0, len(line), 2):
    block = line[i]
    disk += [str(idx)] * block

    if i + 1 < len(line):
        space = line[i + 1]
        disk += ["."] * space
    idx += 1

print(disk)
free_idx = 0
block_idx = len(disk) - 1

while True:
    while free_idx < len(disk):
        if disk[free_idx] == ".":
            break
        free_idx += 1
    while block_idx >= 0:
        if disk[block_idx].isnumeric():
            break
        block_idx -= 1

    if free_idx >= block_idx:
        break

    disk[free_idx], disk[block_idx] = disk[block_idx], disk[free_idx]

sum = 0
for i, block in enumerate(disk):
    if block == ".":
        break
    sum += int(block) * i
print(sum)
