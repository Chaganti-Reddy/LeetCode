from typing import Optional

class Node:
    def __init__(self, key: int, value: int):
        self.key = key
        self.value = value
        self.prev: Optional['Node'] = None
        self.next: Optional['Node'] = None

class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.keyToNode = {}
        self.head = Node(-1, -1)
        self.tail = Node(-1, -1)
        self.connect(self.head, self.tail)

    def get(self, key: int) -> int:
        if key not in self.keyToNode:
            return -1
        node = self.keyToNode[key]
        self.refresh(node)
        return node.value

    def put(self, key: int, value: int) -> None:
        if key in self.keyToNode:
            node = self.keyToNode[key]
            node.value = value
            self.refresh(node)
        else:
            if len(self.keyToNode) == self.capacity:
                last = self.tail.prev
                self.remove(last)
                del self.keyToNode[last.key]
            newNode = Node(key, value)
            self.moveToHead(newNode)
            self.keyToNode[key] = newNode

    def connect(self, node1: Node, node2: Node):
        node1.next = node2
        node2.prev = node1

    def moveToHead(self, node: Node):
        self.connect(node, self.head.next)
        self.connect(self.head, node)

    def remove(self, node: Node):
        self.connect(node.prev, node.next)

    def refresh(self, node: Node):
        self.remove(node)
        self.moveToHead(node)