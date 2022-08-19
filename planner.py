from typing import *
from collections import deque


T = TypeVar("T")


class Graph(Protocol[T]):
    def adjacent(self, node: T) -> Iterable[T]:
        ...


class Path:
    def __init__(self, node: T, steps: int, parent: Optional["Path"]):
        self.node = node
        self.steps = steps
        self.parent = parent

    def as_list(self) -> list[T]:
        return list(self.as_iter())

    def as_iter(self) -> Iterable[T]:
        yield self.node
        if self.parent:
            yield from self.parent.as_iter()

    def __repr__(self):
        return (f"<{self.__class__.__qualname__} "
                f"node={self.node!r} "
                f"steps={self.steps!r} "
                f"parent={(self.parent.node if self.parent else None)!r}"
                ">")


def bft(start: T, graph: Graph[T]) -> Iterable[Path]:
    frontier = deque([Path(node=start, steps=0, parent=None)])
    seen: Set[T] = set()

    while frontier:
        path = frontier.popleft()
        if path.node in seen:
            continue

        yield path
        seen.add(path.node)

        for adj_node in graph.adjacent(path.node):
            frontier.append(Path(
                node=adj_node,
                steps=path.steps + 1,
                parent=path
            ))


def bfs(start: T, goal: T, graph: Graph[T]):
    for path in bft(start, graph):
        if path.node == goal:
            return path
