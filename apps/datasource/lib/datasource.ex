defmodule DataSource do
  @moduledoc """
  Documentation for DataSource.
  """

  def path_to_words do
    :datasource
    |> Application.app_dir("priv")
    |> Path.join("sorted_words2.txt")
  end
end
