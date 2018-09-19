{:ok, set} = FST.words_fst()
{:ok, index} = Tantivy.words()

Benchee.run(%{
      "FST.query/3" => fn -> FST.query(set, "hello", 1) end,
      "Tantivy.query/2" => fn -> Tantivy.query(index, "hello") end
 })
