# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 位置指定で推論型を引く（実 Rigor の `rigor type-of FILE:LINE:COL` の極小版）。
  # line は 1 始まり、col は 1 始まり（人間向け）。指定位置を含む*最小の式*の型を返す。
  # その位置に式が無ければ nil。診断は出さない（annotate と同じく表示のための道具）。
  def type_at(source, line, col)
    program = Prism.parse(source).value
    col0 = col - 1 # 内部は 0 始まり列に揃える（Prism の location に合わせる）
    scope = Scope.new

    program.statements.body.each do |stmt|
      # 位置を含む文に当たったら、その文の手前までで育てた scope で型を引く。
      # （文の中の参照は、それ以前の束縛で解決される。引数は未束縛＝untyped。）
      return type_of(node_at(stmt, line, col0), scope, []) if contains?(stmt.location, line, col0)

      _type, scope = eval_statement(stmt, scope, [])
    end
    nil
  end

  # 位置を含む最小（最深）のノードを返す。どの子も含まなければ自分自身。
  def node_at(node, line, col0)
    node.compact_child_nodes.each do |child|
      next unless child.location && contains?(child.location, line, col0)

      deeper = node_at(child, line, col0)
      return deeper if deeper
    end
    contains?(node.location, line, col0) ? node : nil
  end

  # location が (line, col0) を含むか。終端列は半開区間（end_column は含めない）。
  def contains?(location, line, col0)
    return false if line < location.start_line || line > location.end_line
    return false if line == location.start_line && col0 < location.start_column
    return false if line == location.end_line && col0 >= location.end_column

    true
  end
end
