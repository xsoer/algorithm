// Trie (音 try) 是一种树数据结构，又称为字典树，前缀树，用于检索某个单词或前缀是否存在于树中。Trie 应用广泛，包括打字预测，自动补全，拼写检查等
// 平衡树和哈希表也能够用于搜索单词，为什么还需要 Trie 呢？哈希表能在 O(1) 时间内找到单词，但却无法快速地找到具有同一前缀的全部单词或按字典序枚举出所有存储的单词。
// Trie 树优于哈希表的另一个点是，单词越多，哈希表就越大，这意味着可能出现大量冲突，时间复杂度可能增加到 O(n)。
// 与哈希表相比，Trie 树存储多个具有相同前缀的单词时可以使用更少的空间，时间复杂度也只有 O(m)，m 为单词长度，而在平衡树中查找单词的时间复杂度为 O(m log(n))

