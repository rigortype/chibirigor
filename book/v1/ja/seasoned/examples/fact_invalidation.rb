# frozen_string_literal: true

# The Seasoned chibirigor Part 6 ― 「再代入で事実が消える」最小スケッチ（FactStore の核）。
# 依存ゼロ・単体で走る。`ruby fact_invalidation.rb` で自己チェックが緑になる。
#
# 本物の Rigor の FactStore は 6 バケツ＋payload＋polarity＋stability を持つが、ここでは
# 「不変ストア」「バケツ指定の無効化」だけを最小化して、再代入で事実が消える様子を見る。

Fact = Struct.new(:bucket, :target, :predicate)

# region factstore
# 不変な事実の束。with_fact / invalidate_target は *新しい* ストアを返す。
class FactStore
  def initialize(facts = [])
    @facts = facts.freeze
  end

  def with_fact(bucket, target, predicate)
    FactStore.new(@facts + [Fact.new(bucket, target, predicate)])
  end

  # target に関する事実を消した新ストア。buckets を指定すると、そのバケツだけ消す。
  def invalidate_target(target, buckets: nil)
    kept = @facts.reject do |f|
      f.target == target && (buckets.nil? || buckets.include?(f.bucket))
    end
    FactStore.new(kept)
  end

  def predicates_for(target)
    @facts.select { |f| f.target == target }.map(&:predicate)
  end
end
# endregion

# --- 自己チェック -----------------------------------------------------------
if __FILE__ == $PROGRAM_NAME
  results = {}

  # if x（nil チェック）を通った後：x は non-nil、という local_binding 事実を足す
  store = FactStore.new.with_fact(:local_binding, :x, 'non-nil')
  results['fact is present after narrowing'] = store.predicates_for(:x) == ['non-nil']

  # 再代入 x = ... → x の local_binding 事実は消える
  after_reassign = store.invalidate_target(:x, buckets: [:local_binding])
  results["reassignment clears x's local_binding fact"] = after_reassign.predicates_for(:x) == []

  # バケツ指定：obj のメソッド呼び出しは object_content だけ疑う。
  # local_binding（obj 自体の束縛）は残る。
  mixed = FactStore.new
                   .with_fact(:local_binding, :obj, 'is-a User')
                   .with_fact(:object_content, :obj, 'name is set')
  after_call = mixed.invalidate_target(:obj, buckets: [:object_content])
  results['method call drops object_content but keeps local_binding'] =
    after_call.predicates_for(:obj) == ['is-a User']

  results.each { |name, ok| puts "#{ok ? 'PASS' : 'FAIL'}: #{name}" }
  exit(results.values.all? ? 0 : 1)
end
