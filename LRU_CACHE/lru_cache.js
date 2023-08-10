class LRUCache {
  constructor(capacity) {
    this.capacity = capacity;
    this.cache = {};
    this.head = { key: 'head', next: null, prev: null };
    this.tail = { key: 'tail', next: null, prev: null };
    this.head.next = this.tail;
    this.tail.prev = this.head;
  }

  get(key) {
    if (this.cache[key]) {
      this.moveToHead(this.cache[key]);
      return this.cache[key].value;
    } else {
      return -1; // Key not found
    }
  }

  put(key, value) {
    if (this.cache[key]) {
      this.cache[key].value = value;
      this.moveToHead(this.cache[key]);
    } else {
      if (Object.keys(this.cache).length >= this.capacity) {
        this.removeTail();
      }
      const newNode = { key, value };
      this.cache[key] = newNode;
      this.addToHead(newNode);
    }
  }

  moveToHead(node) {
    this.removeNode(node);
    this.addToHead(node);
  }

  removeNode(node) {
    const prevNode = node.prev;
    const nextNode = node.next;
    prevNode.next = nextNode;
    nextNode.prev = prevNode;
  }

  addToHead(node) {
    const oldHeadNext = this.head.next;
    this.head.next = node;
    node.prev = this.head;
    node.next = oldHeadNext;
    oldHeadNext.prev = node;
  }

  removeTail() {
    const tailNode = this.tail.prev;
    this.removeNode(tailNode);
    delete this.cache[tailNode.key];
  }
}

const cache = new LRUCache(3); // Create an LRU cache with a capacity of 3

cache.put('key1', 'value1');
cache.put('key2', 'value2');
cache.put('key3', 'value3');

console.log(cache.get('key1')); // Output: "value1"
console.log(cache.get('key2')); // Output: "value2"
console.log(cache.get('key3')); // Output: "value3"

cache.put('key4', 'value4'); // This will remove the least recently used item ("key1")

console.log(cache.get('key1')); // Output: -1 (Key not found)