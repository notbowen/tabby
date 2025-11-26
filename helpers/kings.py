import itertools

moves = {}

for x in range(8):
    for y in range(8):
        m = 0

        for dx, dy in itertools.product([-1, 0, 1], repeat=2):
            if dx == 0 and dy == 0:
                continue

            nx = x + dx
            ny = y + dy

            if nx < 0 or nx > 7 or ny < 0 or ny > 7:
                continue

            m |= 1 << (ny * 8) + nx

        moves[y * 8 + x] = m

print([v for _, v in sorted(moves.items())])
