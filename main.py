from typing import Iterable

from planner import Graph, bfs


class JugFilling(Graph[tuple[int, int, int]]):
    """taken from raymond hettinger @raymondh"""

    def __init__(self, capacity: tuple[int, int, int]):
        self.capacity = capacity

    def adjacent(self, node: tuple[int, int, int]) -> Iterable[tuple[int, int, int]]:
        for i in range(len(node)):
            for j in range(len(node)):
                if i == j:
                    continue

                qty = min(node[i], self.capacity[j] - node[j])

                if not qty:
                    continue

                dup = list(node)
                dup[i] -= qty
                dup[j] += qty

                yield tuple(dup)


res = bfs(
    start=(0, 0, 8),
    goal=(0, 4, 4),
    graph=JugFilling(capacity=(3, 5, 8))
)

print(res)
print(res.as_list())
