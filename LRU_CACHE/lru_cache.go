package main

import (
	"fmt"
)

type LRUCache struct {
	capacity int
	cache    map[int]*cacheNode
	head     *cacheNode
	tail     *cacheNode
}

type cacheNode struct {
	key   int
	value int
	prev  *cacheNode
	next  *cacheNode
}

func NewLRUCache(capacity int) *LRUCache {
	return &LRUCache{
		capacity: capacity,
		cache:    make(map[int]*cacheNode),
	}
}

func (c *LRUCache) Get(key int) int {
	if node, ok := c.cache[key]; ok {
		c.moveToFront(node)
		return node.value
	}
	return -1
}

func (c *LRUCache) Put(key int, value int) {
	if node, ok := c.cache[key]; ok {
		node.value = value
		c.moveToFront(node)
	} else {
		newNode := &cacheNode{key: key, value: value}
		c.cache[key] = newNode
		c.addToFront(newNode)

		if len(c.cache) > c.capacity {
			c.removeTail()
		}
	}
}

func (c *LRUCache) moveToFront(node *cacheNode) {
	if node == c.head {
		return
	}

	c.removeNode(node)
	c.addToFront(node)
}

func (c *LRUCache) removeNode(node *cacheNode) {
	prevNode := node.prev
	nextNode := node.next

	if prevNode != nil {
		prevNode.next = nextNode
	} else {
		c.head = nextNode
	}

	if nextNode != nil {
		nextNode.prev = prevNode
	} else {
		c.tail = prevNode
	}
}

func (c *LRUCache) addToFront(node *cacheNode) {
	if c.head == nil {
		c.head = node
		c.tail = node
	} else {
		node.next = c.head
		c.head.prev = node
		c.head = node
	}
}

func (c *LRUCache) removeTail() {
	if c.tail == nil {
		return
	}

	delete(c.cache, c.tail.key)
	if c.tail == c.head {
		c.head = nil
		c.tail = nil
	} else {
		c.tail = c.tail.prev
		c.tail.next = nil
	}
}

func main() {
	cache := NewLRUCache(2)

	cache.Put(1, 1)
	cache.Put(2, 2)
	fmt.Println(cache.Get(1)) // Output: 1
	cache.Put(3, 3)
	fmt.Println(cache.Get(2)) // Output: -1 (not found)
	cache.Put(4, 4)
	fmt.Println(cache.Get(1)) // Output: -1 (not found)
	fmt.Println(cache.Get(3)) // Output: 3
	fmt.Println(cache.Get(4)) // Output: 4
}
