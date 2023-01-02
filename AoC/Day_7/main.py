def get_fs_size(filename):
    with open(filename) as file:
        lines = file.readlines()
    fs = {('/',): 0}
    path = []
    for line in lines:
        match line.split():
            case ["$", "ls"]:
                pass
            case ["$", "cd", ".."]:
                path = path[:-1]
            case ["$", "cd", directory]:
                path += [directory]
            case ["dir", directory]:
                fs.setdefault(tuple(path + [directory]), 0)
            case [size, filename]:
                for i in range(1, len(path) + 1):
                    fs[tuple(path[:i])] += int(size)
    return fs


fs = get_fs_size("input.txt")

print("===part 1===\n", sum([x for x in fs.values() if x <= 100000]))

print("=== part 2 ===")
FS_SIZE = 70000000
SPACE_NEEDED = 30000000
unused = FS_SIZE - fs[("/",)]
size_to_del = SPACE_NEEDED - unused
big_enough_dirs = {k: v for (k, v) in fs.items() if v >= size_to_del}
print(" - Qualifying Dirs ")
for k, v in big_enough_dirs.items():
    print("    - ", end="")
    print(*k, sep="/", end="")
    print(": ", v)
print("Smallest: ", min(big_enough_dirs.values()))
