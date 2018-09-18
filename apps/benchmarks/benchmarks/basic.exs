{:ok, set} = FST.words_fst()

Benchee.run(%{
  "FST.query/3" => fn -> FST.query(set, "hello", 1) end
 })
