# Partition Labels

题解：因为同一个字母只能在一个区间出现，所以每个字母最后出现的位置就需要密切关注。

我们可以用一个大小为 26 的数组来保存每个字母出现的位置。在一个 `for` 循环中遍历字符串，每次遇到相同字母就更新它在数组中的位置信息。遍历完成后数组中保存的就是该字母在字符串中最后出现的位置。

> Rust 无法索引字符串，需要将字符串用 `chars()` 方法转换成一个字符的迭代器，用 `enumerate()` 方法获取每个字符及其在字符串中的位置。

接下来在另一个 `for` 循环中比较哪个字母的位置更远，最远的字母的位置即为是字符串切分的位置。然后从切分位置右边为起点，重复比较字母的最远位置，直到字符串的末尾。

时间复杂度为 $O(n)$，$n$ 为字符串长度。空间复杂度为 $O(|\sum|)$，其中 $|\sum|$ 表示字符串中的字符集，本题中只有小写字母，因此等于 26。