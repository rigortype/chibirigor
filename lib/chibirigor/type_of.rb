# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 式（Prism のノード）から型を求める。型検査器の心臓。
  # scope は型環境（変数名→型）。わからなければ Dynamic を返す（脅かさない）。
  def type_of(node, scope, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::LocalVariableReadNode then scope.local(node.name) || Type::Dynamic.new
    when Prism::CallNode then type_of_call(node, scope, diagnostics)
    else Type::Dynamic.new
    end
  end

  # メソッド送信。レシーバと各引数の型を求め、ディスパッチ表に委ねる。
  # （Part 1 の `+` 場当たり特別扱いを、Part 2 で手書きの表に一般化した。）
  def type_of_call(node, scope, diagnostics)
    receiver = node.receiver ? type_of(node.receiver, scope, diagnostics) : Type::Dynamic.new
    arg_types = (node.arguments&.arguments || []).map { |arg| type_of(arg, scope, diagnostics) }
    Dispatch.dispatch(receiver, node.name, arg_types, node, diagnostics)
  end

  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
