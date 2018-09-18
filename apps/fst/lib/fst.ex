defmodule FST do
  @moduledoc "nif bindings to https://github.com/BurntSushi/fst"

  use Rustler, otp_app: :fst, crate: :fst_nifs

  def words_fst do
    :ok = build_set_from_file(DataSource.path_to_words())
    # :ok = build_set_from_file("/usr/share/dict/words")

    receive do
      {:ok, _set} = success -> success
      other -> other
    end
  end

  def build_set_from_file(_path), do: error()
  def put(_set, _value), do: error()
  def query(_set, _query, _distance), do: error()

  defp error do
    :erlang.nif_error(:nif_not_loaded)
  end
end
