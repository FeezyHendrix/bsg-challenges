class LRUCache:
    def __init__(self, capacity) -> None:
        self.capacity = capacity
        self.cache = {}
        self.head = { 'key': 'head', 'next': None, 'prev': None }
        self.tail = { 'key': 'tail', 'next': None, 'prev': None }
        self.head['next'] = self.tail
        self.tail['prev'] = self.head
    

    def get(self, key):
        if (self.cache.get(key) != None):
            self.moveToHead(self.cache.get(key))
            return self.cache[key]['value']
        else:
            return -1
    

    def put(self, key, value):
        if self.cache.get(key) != None:
            self.cache.get(key).value = value
            self.moveToHead(self.cache[key])
        else:
            if (len(self.cache) >= self.capacity):
                self.removeTail()
            newNode = { 'key': key, 'value': value }
            self.cache[key] = newNode
            self.addToHead(newNode)

    def moveToHead(self, node):
        self.removeNode(node)
        self.addToHead(node)
    
    def addToHead(self, node):
        oldHeadNext = self.head.get('next')
        self.head['next'] = node
        node['prev'] = self.head
        node['next'] = oldHeadNext
        oldHeadNext['prev'] = node
    
    def removeNode(self, node):
        prevNode = node['prev']
        nextNode = node['next']
        prevNode['next'] = nextNode
        nextNode['prev'] = prevNode
    
    def removeTail(self):
        tailNode = self.tail.get('prev')
        self.removeNode(tailNode)
        self.cache.pop(tailNode.get('key'))




# Example usage
lru_cache = LRUCache(3)
lru_cache.put(1, 1)
lru_cache.put(2, 2)
lru_cache.put(3, 3)
print(lru_cache.get(2))  # Output: 2
lru_cache.put(4, 4)
print(lru_cache.get(1))  # Output: -1 (not found)
print(lru_cache.get(3)) 