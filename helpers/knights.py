moves = {}

dirs = [(-2, -1), (-1, -2), (1, -2), (2, -1), (2, 1), (1, 2), (-1, 2), (-2, 1)]

for col in range(8):
    for row in range(8):
        n = 0

        for x, y in dirs:
            r = row + x
            c = col + y

            if r < 0 or r > 7 or c < 0 or c > 7:
                continue

            n |= 1 << (c * 8 + r)

        moves[col * 8 + row] = n

print([v for _, v in sorted(moves.items())])
