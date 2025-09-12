import std/[
  tables,
  options
]


type
  Node*[T] = ref object
    children: Table[char, Node[T]]
    value: Option[T]

  Trie*[T] = object
    root: Node[T]


func newNode*[T](): Node[T] =
  Node[T](children: initTable[char, Node[T]](), value: none[T]())

proc addChild[T](self: Node[T], ch: char): Node[T] =
  ## Adds a child nodw
  if self.children.hasKey(ch):
    result = self.children[ch]
  else:
    result = newNode[T]()
    self.children[ch] = result

func initTrie*[T](): Trie[T] =
  Trie[T](root: newNode[T]())

proc len*[T](self: Trie[T]): int =
  self.root.children.len()

proc insert*[T](self: var Trie[T], key: string, value: T) =
  var node = self.root

  for ch in key:
    node = node.addChild(ch)

  node.value = some(value)

proc get*[T](self: var Trie[T], key: string): Option[T] =
  var node = self.root

  for ch in key:
    if node.children.hasKey(ch):
      node = node.children[ch]
    else:
      return none[T]()

  node.value

proc delete[T](node: Node[T], key: string): bool =
  if key.len() == 0:
    if node.value.isSome():
      node.value = none[T]()
      return node.children.len() == 0

    return false

  result = false
  
  let
    ch = key[0]
    should_delete =
      if node.children.hasKey(ch):
        node.children[ch].delete(key[1..^1])
      else:
        return false

  if should_delete:
    node.children.del(ch)
    result = not node.value.isSome() and node.children.len() == 0

proc delete*[T](self: var Trie[T], key: string): bool =
  if key.len() == 0:
    return false

  result = self.root.delete(key)
