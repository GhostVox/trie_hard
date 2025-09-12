# This is just an example to get you started. You may wish to put all of your
# tests into a single file, or separate them into multiple `test1`, `test2`
# etc. files (better names are recommended, just make sure the name starts with
# the letter 't').
#
# To run these tests, simply execute `nimble test`.

import std/[
  unittest,
  options,
  strutils,
  sequtils,
  random
]

import triehard

func incv(val: var int): int =
  inc val
  result = val

test "insert/get for int":
  var
    rng {.compileTime.} = initRand(332952359032850924)
    count {.compileTime.} = 0

  const cases = [
    ("hello", incv count),
    ("there", incv count),
    ("meow", incv count)
  ]

  var trie = initTrie[int]()

  for (k, v) in cases:
    trie.insert(k, v)

  for (k, v) in cases:
    let val = trie.get(k)

    require val.isSome()
    require val.get() == v

test "delete for bool":
  let cases = [
    ("object", true),
    ("exit!", false),
    ("stop", false),
    ("robin", true),
    ("robert", false)
  ]

  var trie = initTrie[bool]()

  for (k, v) in cases:
    trie.insert(k, v)

  for (k, _) in cases:
    discard trie.delete(k)

    require trie.get(k).isNone()

test "insert/get/delete for float with test_data":
  var
    rng {.compileTime.} = initRand(332952359032850924)
    count {.compileTime.} = 0

  const cases = staticRead("../../test_data.txt")
      .strip()
      .split("\n")
      .mapIt((it, incv count))


  var trie = initTrie[int]()

  for (k, v) in cases:
    trie.insert(k, v)

  for (k, v) in cases:
    let val = trie.get(k)

    require not val.isNone()
    require val.get() == v

    discard trie.delete(k)

    require trie.get(k).isNone()
