defmodule Tantivy do
  @moduledoc "nifs for https://github.com/tantivy-search/tantivy"

  use Rustler, otp_app: :tantivy, crate: :tantivy_nifs

  def words do
    :ok = build_index_from_file(DataSource.path_to_words())

    receive do
      {:ok, _index} = success -> success
      other -> other
    end
  end

  def build_index_from_file(path), do: error()
  def query(_index, _query), do: error()

  defp error do
    :erlang.nif_error(:nif_not_loaded)
  end
end
