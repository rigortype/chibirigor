# frozen_string_literal: true

require 'prism'

module Chibirigor
  module_function

  # 式（Prism のノード）から型を求める。型検査器の心臓。
  # わからなければ Dynamic を返す（失敗しない＝脅かさない）。
  def type_of(node, diagnostics)
    case node
    when Prism::IntegerNode then Type::Const[node.value]
    when Prism::FloatNode   then Type::Const[node.value]
    when Prism::StringNode  then Type::Const[node.unescaped]
    when Prism::TrueNode    then Type::Const[true]
    when Prism::FalseNode   then Type::Const[false]
    when Prism::CallNode    then type_of_call(node, diagnostics)
    else Type::Dynamic.new
    end
  end

  # メソッド送信。Part 1 では算術 `+` だけを特別扱いする。
  def type_of_call(node, diagnostics)
    receiver = node.receiver ? type_of(node.receiver, diagnostics) : nil
    args = node.arguments&.arguments || []

    if node.name == :+ && receiver && Type.integerish?(receiver)
      arg = type_of(args.first, diagnostics)
      unless Type.integerish?(arg)
        diagnostics << diagnostic(node, "整数に #{arg} は足せません")
        return Type::Dynamic.new
      end
      # 値そのもの（Const[3]）には畳まず、Integer に丸める
      return Type::Nominal[:Integer]
    end

    Type::Dynamic.new # それ以外のメソッドはまだ知らない → 脅かさない
  end

  def diagnostic(node, message)
    { line: node.location.start_line, message: message }
  end
end
